/// #Proyecto: Explorador de Campos Eléctricos
/// Este proyecto permite ver la dirección y el módulo del campo generado
/// por diferentes partículas en un espaacio. También permitirá ver cómo una
/// pertícula móvil ve cambiado su estado de movimiento en base al campo existente
/// en el punto en el que esté.
/// Por ahora, se han implementado las siguientes funciones:
/// - Establecimmiento del struct Vector2D.
/// - Establecimiento del struct Partícula.
/// - Establecimiento de las diferentes operaciones matemáticas referentes a Vector2D.
/// - Cálculo del campo eléctrico generado por una partícula en un punto.
mod math;
use math::Vector2D;
mod physics;
use physics::{Particle, electric_field_at_point, total_electric_field};
mod screen;
use macroquad::{color::hsl_to_rgb, prelude::*};

use crate::screen::{screen_to_world, world_to_screen};

/// Constantes utilizadas en el proyecto.
const PIXELS_PER_METER: f64 = 20_f64;
const GRID_STEP_METERS: f64 = 0.5;
const HSL_SATURATION: f32 = 1.0;
const HSL_LIGHTNESS: f32 = 0.5;
const HSL_BLUE_HUE: f64 = 0.66_f64;

const FIELD_SEGMENT_LENGTH_PIXELS: f64 = 5_f64;
const FIELD_SEGMENT_WIDTH_PIXELS: f32 = 2_f32;

const MAX_FIELD_SAMPLE_RADIUS_FACTOR: f64 = 1.01;
const FIELD_COLOR_RANGE_RATIO: f64 = 5E3;

const PARTICLE_RADIUS_SCALE_FACTOR: f64 = 1.2_f64;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut particle: Particle = Particle {
        mass: 1.0,
        position: Vector2D { x: 3.0, y: 2.0 },
        charge: 1e-9,
        radius: 0.5,
    };

    let max_field_magnitude = electric_field_at_point(
        &particle,
        Vector2D {
            x: particle.position.x + particle.radius * MAX_FIELD_SAMPLE_RADIUS_FACTOR,
            y: particle.position.y,
        },
    )
    .magnitude();
    let min_field_magnitude = max_field_magnitude / FIELD_COLOR_RANGE_RATIO;

    let mut is_dragging = false;
    let mut drawn_particle_radius = particle.radius;
    loop {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();
        let mut particle_screen_position =
            world_to_screen(particle.position, width, height, PIXELS_PER_METER);

        let mouse_position_screen = mouse_position();
        let mouse_position_world = screen_to_world(
            Vector2D {
                x: mouse_position_screen.0 as f64,
                y: mouse_position_screen.1 as f64,
            },
            width,
            height,
            PIXELS_PER_METER,
        );

        let mouse_is_over_particle = (mouse_position_world.x - particle.position.x).abs().powi(2)
            + (mouse_position_world.y - particle.position.y).abs().powi(2)
            <= particle.radius.powi(2);

        if mouse_is_over_particle && is_mouse_button_pressed(MouseButton::Left) {
            is_dragging = true;
            drawn_particle_radius = particle.radius * PARTICLE_RADIUS_SCALE_FACTOR;
        }

        if is_dragging && is_mouse_button_down(MouseButton::Left) {
            particle.position = mouse_position_world;
            particle_screen_position =
                world_to_screen(particle.position, width, height, PIXELS_PER_METER);
        }
        if is_mouse_button_released(MouseButton::Left) {
            is_dragging = false;
            drawn_particle_radius = particle.radius;
        }

        draw(
            &particle,
            particle_screen_position,
            width,
            height,
            drawn_particle_radius,
            min_field_magnitude,
            max_field_magnitude,
        );

        next_frame().await
    }
}

fn draw(
    particle: &Particle,
    particle_screen_position: Vector2D,
    width: f32,
    height: f32,
    actual_particle_radius: f64,
    min_field_magnitude: f64,
    max_field_magnitude: f64,
) {
    let bottom_right_world: Vector2D = screen_to_world(
        Vector2D {
            x: width as f64,
            y: height as f64,
        },
        width,
        height,
        PIXELS_PER_METER,
    );
    let mut x = -bottom_right_world.x;
    let mut y = bottom_right_world.y;
    while y < -bottom_right_world.y {
        while x < bottom_right_world.x {
            let point: Vector2D = Vector2D { x, y };

            let field_at_point: Vector2D = electric_field_at_point(particle, point);

            if field_at_point.magnitude() != 0.0 {
                let field_direction: Vector2D = field_at_point.normalized();

                let field_segment_end_world: Vector2D =
                    field_direction * FIELD_SEGMENT_LENGTH_PIXELS / PIXELS_PER_METER + point;

                let point_screen_position: Vector2D =
                    world_to_screen(point, width, height, PIXELS_PER_METER);
                let field_segment_end_screen: Vector2D =
                    world_to_screen(field_segment_end_world, width, height, PIXELS_PER_METER);

                let clamped_field_magnitude = field_at_point
                    .magnitude()
                    .clamp(min_field_magnitude, max_field_magnitude);
                let t = (clamped_field_magnitude.ln() - min_field_magnitude.ln())
                    / (max_field_magnitude.ln() - min_field_magnitude.ln());
                let color_hue = ((1_f64 - t) * HSL_BLUE_HUE) as f32;
                let color = hsl_to_rgb(color_hue, HSL_SATURATION, HSL_LIGHTNESS);
                draw_line(
                    point_screen_position.x as f32,
                    point_screen_position.y as f32,
                    field_segment_end_screen.x as f32,
                    field_segment_end_screen.y as f32,
                    FIELD_SEGMENT_WIDTH_PIXELS,
                    color,
                );
            }
            x += GRID_STEP_METERS;
        }
        x = -bottom_right_world.x;
        y += GRID_STEP_METERS;
    }
    draw_circle(
        particle_screen_position.x as f32,
        particle_screen_position.y as f32,
        (actual_particle_radius * PIXELS_PER_METER) as f32,
        WHITE,
    );
}
