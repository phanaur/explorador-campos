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
/// - Campo total en un punto.
/// - Dibujado de partículas.
/// - Dibujado del campo: segmentos de longitud fija en la dirección del campo con una escala de color que representa el valor del módulo del vector
/// - Establecimiento de condiciones para poder mover partículas a otras posiciones, recalculando el campo sobre la marcha
///
/// En este código, hay implementaciones que contienen cosas como iter(), collect(), map(), etc. Esas instrucciones se han
///   incorporado basándo mi criterio en las sugerencias del editor de texto. Posteriormente se procederá a un uso de bucles
///   estándar para despúes volver a los diseños con esas características, aprendiendo en el proceso lo que hace cada cosa.
///
/// Al final, la decisión última sobre el código, su comportamiento e implementación es mía, con independencia de quién genere ese código.
/// En todo momento sé lo que hace el código y se comporta de la manera que yo espero
mod math;
use math::Vector2D;
mod physics;
use physics::{Particle, total_electric_field};
mod screen;
use macroquad::{color::hsl_to_rgb, prelude::*};

use crate::{
    physics::ElectricFieldPoint,
    screen::{screen_to_world, world_to_screen},
};

/// Constantes utilizadas en el proyecto.
const PIXELS_PER_METER: f64 = 20_f64;
const GRID_STEP_METERS: f64 = 0.5;
const HSL_SATURATION: f32 = 1.0;
const HSL_LIGHTNESS: f32 = 0.5;
const HSL_BLUE_HUE: f64 = 0.66_f64;

const FIELD_SEGMENT_LENGTH_PIXELS: f64 = 5_f64;
const FIELD_SEGMENT_WIDTH_PIXELS: f32 = 2_f32;

const PARTICLE_RADIUS_SCALE_FACTOR: f64 = 1.2_f64;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut particles: Vec<Particle> = vec![
        Particle {
            mass: 1.0,
            position: Vector2D { x: 3.0, y: 2.0 },
            charge: 1e-9,
            radius: 0.5,
        },
        Particle {
            mass: 1.0,
            position: Vector2D { x: -3.0, y: -2.0 },
            charge: -1e-9,
            radius: 0.5,
        },
    ];

    let mut max_field_magnitude = f64::NEG_INFINITY;
    let mut min_field_magnitude = f64::INFINITY;

    let mut electric_field_points: Vec<ElectricFieldPoint> = Vec::new();

    let mut is_dragging = false;
    let mut dragged_particle_index: Option<usize> = None;

    // Establece un vector que obtiene los radios declarados de las partículas para su manipulación
    //let mut drawn_particle_radius: Vec<f64> =
    //    particles.iter().map(|particle| particle.radius).collect();

    let mut drawn_particle_radius: Vec<f64> = Vec::new();
    for particle in &particles {
        drawn_particle_radius.push(particle.radius);
    }

    loop {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();

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

        // Itera sobre el Vec de partículas, viendo si la posición del cursor se encuentra dentro del radio de alguna de ellas.
        //let mouse_is_over_particle: Vec<bool> = particles
        //    .iter()
        //    .map(|particle| {
        //        (mouse_position_world.x - particle.position.x).abs().powi(2)
        //            + (mouse_position_world.y - particle.position.y).abs().powi(2)
        //            <= particle.radius.powi(2)
        //    })
        //    .collect();

        let mut mouse_is_over_particle: Vec<bool> = Vec::new();
        for particle in &particles {
            if (mouse_position_world.x - particle.position.x).abs().powi(2)
                + (mouse_position_world.y - particle.position.y).abs().powi(2)
                <= particle.radius.powi(2)
            {
                mouse_is_over_particle.push(true);
                continue;
            }
            mouse_is_over_particle.push(false);
        }

        // Si el ratón está sobre alguna partícula y el click izquierdo está pulsado, establece la condición de desplazamiento a true. Sólo de esa partícula
        //if mouse_is_over_particle.iter().any(|&is_over| is_over)
        //    && is_mouse_button_pressed(MouseButton::Left)
        //{
        //    is_dragging = true;
        //    dragged_particle_index = mouse_is_over_particle.iter().position(|&is_over| is_over);

        // Busca la partícula sobre la que se clicka y modifica su radio haciéndolo más grande para ver cuál se está moviendo.
        //    for (index, particle) in particles.iter().enumerate() {
        //        if Some(index) == dragged_particle_index {
        //            drawn_particle_radius[index] = particle.radius * PARTICLE_RADIUS_SCALE_FACTOR;
        //        }
        //    }
        //}

        for n in 0..particles.len() {
            if mouse_is_over_particle[n] && is_mouse_button_pressed(MouseButton::Left) {
                is_dragging = true;
                dragged_particle_index = Some(n);

                if let Some(n) = dragged_particle_index {
                    drawn_particle_radius[n] = particles[n].radius * PARTICLE_RADIUS_SCALE_FACTOR;
                }
            }
        }

        // Si la condición de desplazamiento es true y el click sigue presionado, actualiza la posición de esa partícula con la posición del cursor
        if is_dragging
            && is_mouse_button_down(MouseButton::Left)
            && let Some(index) = dragged_particle_index
        {
            particles[index].position = mouse_position_world;
        }

        // Si el click se ha liberado, establece la condición de desplazamiento a false y vuelve a dejar los radios de todas las partículas como estaban.
        // Podría haberse modificado sólo aquella modificada antes, pero el coste de computación es muy bajo.
        if is_mouse_button_released(MouseButton::Left) {
            is_dragging = false;
            dragged_particle_index = None;
            for n in 0..particles.len() {
                drawn_particle_radius[n] = particles[n].radius;
            }
        }
        electric_field_points.clear();

        // Se establecen las dimensiones del mundo físico partiendo de las de la pantalla, manteniendo un paso en la cuadrícula constante.
        // Se itera en filas, de izquierda a derecha y de arriba a abajo.
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

                let field_at_point: Vector2D = total_electric_field(&particles, point);

                electric_field_points.push(ElectricFieldPoint {
                    point: Vector2D {
                        x: point.x,
                        y: point.y,
                    },
                    field: Vector2D {
                        x: field_at_point.x,
                        y: field_at_point.y,
                    },
                });
                x += GRID_STEP_METERS;
            }
            x = -bottom_right_world.x;
            y += GRID_STEP_METERS;
        }

        // Se establecen los límites inferior y superior de la escala de color
        for point in &electric_field_points {
            let magnitude = point.field.magnitude();
            if magnitude == 0.0 || magnitude == f64::NEG_INFINITY || magnitude == f64::INFINITY {
                //Condición para evitar infinitos al tomar logaritmos.
                continue;
            }
            if magnitude > max_field_magnitude {
                max_field_magnitude = magnitude;
            }
            if magnitude < min_field_magnitude {
                min_field_magnitude = magnitude;
            }
        }

        // Se itera sobre el vector partículas calculando las posiciones de cada una en pantalla
        //let mut particles_screen_position: Vec<Vector2D> = particles
        //    .iter()
        //    .map(|particle| world_to_screen(particle.position, width, height, PIXELS_PER_METER))
        //    .collect();
        let mut particles_screen_position: Vec<Vector2D> = Vec::new();
        for particle in &particles {
            particles_screen_position.push(world_to_screen(
                particle.position,
                width,
                height,
                PIXELS_PER_METER,
            ));
        }

        draw_electric_field(
            &electric_field_points,
            width,
            height,
            PIXELS_PER_METER,
            min_field_magnitude,
            max_field_magnitude,
        );
        draw_particles(
            &particles_screen_position,
            &drawn_particle_radius,
            &particles,
        );

        // Se reestablecen los límites de la escala de color.
        max_field_magnitude = f64::NEG_INFINITY;
        min_field_magnitude = f64::INFINITY;

        next_frame().await
    }
}

fn draw_particles(
    particles_screen_positions: &[Vector2D],
    actual_particle_radius: &[f64],
    particles: &[Particle],
) {
    // Para cada tupla generada por las posiciones en pantalla y los respectivos radios de cada partícula se dibuja un círculo blanco
    for ((screen_position, radius), particle) in particles_screen_positions
        .iter()
        .zip(actual_particle_radius.iter())
        .zip(particles.iter())
    {
        draw_circle(
            screen_position.x as f32,
            screen_position.y as f32,
            (radius * PIXELS_PER_METER) as f32,
            WHITE,
        );

        if particle.charge != 0_f64 {
            if particle.charge > 0.0 {
                draw_line(
                    (screen_position.x - radius * PIXELS_PER_METER / 2.0) as f32,
                    screen_position.y as f32,
                    (screen_position.x + radius * PIXELS_PER_METER / 2.0) as f32,
                    screen_position.y as f32,
                    FIELD_SEGMENT_WIDTH_PIXELS,
                    RED,
                );
                draw_line(
                    screen_position.x as f32,
                    (screen_position.y - radius * PIXELS_PER_METER / 2.0) as f32,
                    screen_position.x as f32,
                    (screen_position.y + radius * PIXELS_PER_METER / 2.0) as f32,
                    FIELD_SEGMENT_WIDTH_PIXELS,
                    RED,
                );
            } else if particle.charge < 0.0 {
                draw_line(
                    (screen_position.x - radius * PIXELS_PER_METER / 2.0) as f32,
                    screen_position.y as f32,
                    (screen_position.x + radius * PIXELS_PER_METER / 2.0) as f32,
                    screen_position.y as f32,
                    FIELD_SEGMENT_WIDTH_PIXELS,
                    BLUE,
                );
            }
        }
    }
}

fn draw_electric_field(
    electric_field_points: &[ElectricFieldPoint],
    width: f32,
    height: f32,
    pixels_per_meter: f64,
    min_field_magnitude: f64,
    max_field_magnitude: f64,
) {
    if max_field_magnitude <= min_field_magnitude {
        return;
    }

    for point in electric_field_points {
        let field_magnitude = point.field.magnitude();
        // Si la magnitud del vector campo es 0.0 no se dibuja en pantalla
        if field_magnitude != 0.0 {
            let field_direction: Vector2D = point.field.normalized();

            // Se establece la misma longitud de segmento para cada punto de la cuadrícula.
            let field_segment_end_world: Vector2D =
                field_direction * FIELD_SEGMENT_LENGTH_PIXELS / pixels_per_meter + point.point;

            // Se calculan los puntos inicial y final en pantalla de cada vector representativo del campo
            // en cada punto de la cuadrícula
            let point_screen_position: Vector2D =
                world_to_screen(point.point, width, height, pixels_per_meter);
            let field_segment_end_screen: Vector2D =
                world_to_screen(field_segment_end_world, width, height, pixels_per_meter);

            // Se limita el valor del campo entre dos magnitudes para poder escalarlo a un rango de valores
            // que va del azul al rojo en el formato HSL
            let clamped_field_magnitude =
                field_magnitude.clamp(min_field_magnitude, max_field_magnitude);
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
    }
}
