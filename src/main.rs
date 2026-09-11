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

#[macroquad::main("BasicShapes")]
async fn main() {
    let particle: Particle = Particle {
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

    loop {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();
        let particle_screen_position =
            world_to_screen(particle.position, width, height, PIXELS_PER_METER);

        draw_circle(
            particle_screen_position.x as f32,
            particle_screen_position.y as f32,
            (particle.radius * PIXELS_PER_METER) as f32,
            WHITE,
        );

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

                let field_at_point: Vector2D = electric_field_at_point(&particle, point);

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

        next_frame().await
    }
}
