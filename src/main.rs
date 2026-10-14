//! Explorador de campos eléctricos: cálculo físico, interacción y representación.
//! La física y la proyección permanecen en sus módulos independientes.
//! Los segmentos muestran dirección; su color representa la magnitud relativa
//! dentro del rango de muestras válidas de cada fotograma.
//!
//! Los recorridos con iteradores se recuperaron tras trabajar sus equivalentes
//! con bucles. Los comentarios describen ahora las decisiones vigentes para
//! evitar presentar como pendiente ese aprendizaje ya completado.
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
const GRID_STEP_METERS: f64 = 1.0;
const HSL_SATURATION: f32 = 1.0;
const HSL_LIGHTNESS: f32 = 0.5;
const HSL_BLUE_HUE: f64 = 0.66_f64;

const FIELD_SEGMENT_LENGTH_PIXELS: f64 = 5_f64;
const FIELD_SEGMENT_WIDTH_PIXELS: f32 = 2_f32;

const AXES_WIDTH_PIXELS: f32 = 2_f32;
const AXES_STEP_METERS: f64 = 1.0;
// Es media longitud: las marcas se extienden 3 px a cada lado del eje.
const AXES_STEP_LENGTH_PIXELS: f32 = 3_f32;
// Se etiqueta cada cinco marcas; el valor escrito se obtiene del paso en metros.
const AXIS_LABEL_EVERY_N_TICKS: i32 = 5;
const X_LABEL_VERTICAL_GAP_PIXELS: f32 = 2.0;
const Y_LABEL_HORIZONTAL_GAP_PIXELS: f32 = 4.0;
const AXIS_LABEL_EDGE_MARGIN_PIXELS: f32 = 8.0;

const FONT_SIZE: f32 = 18.0;

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

    // Se reutiliza el almacenamiento de las muestras entre fotogramas.
    let mut electric_field_points: Vec<ElectricFieldPoint> = Vec::new();
    // Some(index) es el único estado de arrastre: un booleano adicional podría
    // contradecirlo. Las futuras inserciones o eliminaciones deberán cancelar
    // la selección, porque un índice no es una identidad persistente.
    let mut dragged_particle_index: Option<usize> = None;

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

        // Se busca directamente la primera coincidencia solo al pulsar:
        // evita crear un Vec<bool> por fotograma para usarlo una sola vez.
        if is_mouse_button_pressed(MouseButton::Left) {
            dragged_particle_index = particle_under_cursor(&particles, mouse_position_world);
        }
        if is_mouse_button_down(MouseButton::Left)
            && let Some(index) = dragged_particle_index
        {
            if let Some(particle) = particles.get_mut(index) {
                particle.position = mouse_position_world;
            } else {
                // Un índice fuera de la colección cancela el arrastre sin panic.
                dragged_particle_index = None;
            }
        }
        if is_mouse_button_released(MouseButton::Left) {
            dragged_particle_index = None;
        }

        let bottom_right_world = screen_to_world(
            Vector2D {
                x: width as f64,
                y: height as f64,
            },
            width,
            height,
            PIXELS_PER_METER,
        );
        sample_electric_field(&particles, bottom_right_world, &mut electric_field_points);
        // El rango pertenece a este fotograma: no hay límites persistentes
        // que puedan olvidarse de reiniciar al modificar el bucle principal.
        let magnitude_range = field_magnitude_range(&electric_field_points);
        let center_screen =
            world_to_screen(Vector2D { x: 0.0, y: 0.0 }, width, height, PIXELS_PER_METER);

        draw_axes(center_screen, width, height);
        draw_electric_field(
            &electric_field_points,
            width,
            height,
            PIXELS_PER_METER,
            magnitude_range,
        );
        draw_particles(
            &particles,
            dragged_particle_index,
            width,
            height,
            PIXELS_PER_METER,
        );
        next_frame().await
    }
}

fn particle_under_cursor(particles: &[Particle], cursor: Vector2D) -> Option<usize> {
    // La resta de vectores expresa la distancia directamente. Elevar al
    // cuadrado ya elimina el signo, por lo que los abs anteriores sobraban.
    particles.iter().position(|particle| {
        (cursor - particle.position).magnitude_squared() <= particle.radius.powi(2)
    })
}

fn sample_electric_field(
    particles: &[Particle],
    bottom_right_world: Vector2D,
    electric_field_points: &mut Vec<ElectricFieldPoint>,
) {
    // Separar el muestreo deja en main la secuencia de interacción y dibujo.
    // clear conserva la capacidad y cada campo se calcula una sola vez.
    electric_field_points.clear();
    let mut y = bottom_right_world.y;
    while y < -bottom_right_world.y {
        let mut x = -bottom_right_world.x;
        while x < bottom_right_world.x {
            let point = Vector2D { x, y };
            let field = total_electric_field(particles, point);
            // Vector2D es Copy: reconstruir x e y no aportaba información.
            electric_field_points.push(ElectricFieldPoint { point, field });
            x += GRID_STEP_METERS;
        }
        y += GRID_STEP_METERS;
    }
}

fn field_magnitude_range(points: &[ElectricFieldPoint]) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for point in points {
        let magnitude = point.field.magnitude();
        // is_finite excluye también NaN. Los ceros no tienen dirección y no
        // admiten logaritmo; ninguno debe alterar los límites del gradiente.
        if magnitude.is_finite() && magnitude > 0.0 {
            min = min.min(magnitude);
            max = max.max(magnitude);
        }
    }
    // None distingue la ausencia de muestras válidas de un rango uniforme.
    if min.is_finite() {
        Some((min, max))
    } else {
        None
    }
}

fn field_color_fraction(magnitude: f64, min: f64, max: f64) -> Option<f64> {
    // Esta función no usa Macroquad: permite comprobar la escala sin ventana
    // y filtra los datos inválidos antes de normalizar vectores o dibujar.
    if !magnitude.is_finite()
        || magnitude <= 0.0
        || !min.is_finite()
        || min <= 0.0
        || !max.is_finite()
        || max < min
    {
        return None;
    }
    let min_log = min.ln();
    let log_span = max.ln() - min_log;
    if log_span <= 0.0 {
        // Un campo uniforme no es nulo: se dibuja con el color intermedio.
        // También cubre límites distintos cuyos logaritmos redondean igual.
        return Some(0.5);
    }
    let magnitude_log = magnitude.clamp(min, max).ln();
    Some(((magnitude_log - min_log) / log_span).clamp(0.0, 1.0))
}

fn draw_particles(
    particles: &[Particle],
    dragged_particle_index: Option<usize>,
    width: f32,
    height: f32,
    pixels_per_meter: f64,
) {
    // Posición y radio de dibujo se calculan desde cada partícula: no hay
    // colecciones paralelas que desincronizar al insertar o eliminar elementos.
    for (index, particle) in particles.iter().enumerate() {
        let screen_position = world_to_screen(particle.position, width, height, pixels_per_meter);
        let radius = if dragged_particle_index == Some(index) {
            particle.radius * PARTICLE_RADIUS_SCALE_FACTOR
        } else {
            particle.radius
        };
        // El aumento es solo visual. Particle::radius y el campo no cambian;
        // al soltar, None restaura el tamaño sin recorrer ni modificar radios.
        let radius_pixels = radius * pixels_per_meter;
        draw_circle(
            screen_position.x as f32,
            screen_position.y as f32,
            radius_pixels as f32,
            WHITE,
        );

        let symbol_color = if particle.charge > 0.0 {
            RED
        } else if particle.charge < 0.0 {
            BLUE
        } else {
            continue;
        };
        // Positivos y negativos comparten el segmento horizontal; solo los
        // positivos añaden el vertical. Se calcula una vez la media longitud.
        let symbol_half_length_pixels = radius_pixels / 2.0;
        draw_line(
            (screen_position.x - symbol_half_length_pixels) as f32,
            screen_position.y as f32,
            (screen_position.x + symbol_half_length_pixels) as f32,
            screen_position.y as f32,
            FIELD_SEGMENT_WIDTH_PIXELS,
            symbol_color,
        );
        if particle.charge > 0.0 {
            draw_line(
                screen_position.x as f32,
                (screen_position.y - symbol_half_length_pixels) as f32,
                screen_position.x as f32,
                (screen_position.y + symbol_half_length_pixels) as f32,
                FIELD_SEGMENT_WIDTH_PIXELS,
                symbol_color,
            );
        }
    }
}

fn draw_electric_field(
    electric_field_points: &[ElectricFieldPoint],
    width: f32,
    height: f32,
    pixels_per_meter: f64,
    magnitude_range: Option<(f64, f64)>,
) {
    let Some((min, max)) = magnitude_range else {
        return;
    };
    for point in electric_field_points {
        let Some(t) = field_color_fraction(point.field.magnitude(), min, max) else {
            continue;
        };
        let Some(field_direction) = point.field.normalized() else {
            continue;
        };
        let field_segment_end_world =
            field_direction * FIELD_SEGMENT_LENGTH_PIXELS / pixels_per_meter + point.point;
        let point_screen_position = world_to_screen(point.point, width, height, pixels_per_meter);
        let field_segment_end_screen =
            world_to_screen(field_segment_end_world, width, height, pixels_per_meter);
        // Se mantiene la escala relativa al fotograma: un mismo color no
        // representa necesariamente el mismo valor absoluto entre escenas.
        let color_hue = ((1.0 - t) * HSL_BLUE_HUE) as f32;
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

fn axis_tick_value(tick_index: i32, step_meters: f64) -> f64 {
    // El contador identifica la marca; su coordenada física depende del paso.
    // Así, la quinta marca con paso de 2 m se etiqueta como 10, no como 5.
    tick_index as f64 * step_meters
}

fn draw_x_axis_tick(center_screen: Vector2D, mark_x_pixels: f32, tick_index: i32) {
    // Ambos tramos usan esta función: cualquier ajuste de alineación se aplica
    // en un único lugar sin cambiar los cuatro recorridos que ya se conocen.
    draw_line(
        mark_x_pixels,
        center_screen.y as f32 + AXES_STEP_LENGTH_PIXELS,
        mark_x_pixels,
        center_screen.y as f32 - AXES_STEP_LENGTH_PIXELS,
        AXES_WIDTH_PIXELS,
        WHITE,
    );
    if tick_index % AXIS_LABEL_EVERY_N_TICKS != 0 {
        return;
    }
    let value = axis_tick_value(tick_index, AXES_STEP_METERS);
    // Se convierte cada texto una vez y se reutiliza al medir y dibujar.
    let label = value.to_string();
    let unsigned_label = value.abs().to_string();
    let dimensions = measure_text(&label, None, FONT_SIZE as u16, 1.0);
    let digits_width = measure_text(&unsigned_label, None, FONT_SIZE as u16, 1.0).width;
    let sign_width = dimensions.width - digits_width;
    let label_bottom_y_pixels =
        center_screen.y as f32 - AXES_STEP_LENGTH_PIXELS - X_LABEL_VERTICAL_GAP_PIXELS;
    let label_baseline_y_pixels = label_bottom_y_pixels + dimensions.offset_y - dimensions.height;
    draw_text(
        &label,
        mark_x_pixels - sign_width - digits_width / 2.0,
        label_baseline_y_pixels,
        FONT_SIZE,
        WHITE,
    );
}

fn draw_y_axis_tick(center_screen: Vector2D, mark_y_pixels: f32, tick_index: i32) {
    // La misma colocación se usa arriba y abajo. mark_y_pixels ya es f32;
    // se elimina la conversión redundante señalada por Clippy.
    draw_line(
        center_screen.x as f32 + AXES_STEP_LENGTH_PIXELS,
        mark_y_pixels,
        center_screen.x as f32 - AXES_STEP_LENGTH_PIXELS,
        mark_y_pixels,
        AXES_WIDTH_PIXELS,
        WHITE,
    );
    if tick_index % AXIS_LABEL_EVERY_N_TICKS != 0 {
        return;
    }
    let label = axis_tick_value(tick_index, AXES_STEP_METERS).to_string();
    let dimensions = measure_text(&label, None, FONT_SIZE as u16, 1.0);
    let label_left_x_pixels =
        center_screen.x as f32 + AXES_STEP_LENGTH_PIXELS + Y_LABEL_HORIZONTAL_GAP_PIXELS;
    draw_text(
        &label,
        label_left_x_pixels,
        mark_y_pixels + dimensions.offset_y - dimensions.height / 2.0,
        FONT_SIZE,
        WHITE,
    );
}

fn draw_axes(center_screen: Vector2D, width: f32, height: f32) {
    draw_line(
        center_screen.x as f32 - width / 2.0,
        center_screen.y as f32,
        center_screen.x as f32 + width / 2.0,
        center_screen.y as f32,
        AXES_WIDTH_PIXELS,
        WHITE,
    );
    draw_line(
        center_screen.x as f32,
        center_screen.y as f32 - height / 2.0,
        center_screen.x as f32,
        center_screen.y as f32 + height / 2.0,
        AXES_WIDTH_PIXELS,
        WHITE,
    );
    let step_pixels = (AXES_STEP_METERS * PIXELS_PER_METER) as f32;
    let mut tick_index = 1;
    let mut mark_x_pixels = center_screen.x as f32 + step_pixels;
    while mark_x_pixels < center_screen.x as f32 + width / 2.0 {
        draw_x_axis_tick(center_screen, mark_x_pixels, tick_index);
        mark_x_pixels += step_pixels;
        tick_index += 1;
    }
    tick_index = -1;
    mark_x_pixels = center_screen.x as f32 - step_pixels;
    while mark_x_pixels > center_screen.x as f32 - width / 2.0 {
        draw_x_axis_tick(center_screen, mark_x_pixels, tick_index);
        mark_x_pixels -= step_pixels;
        tick_index -= 1;
    }
    // En pantalla Y crece hacia abajo: ese tramo representa valores negativos.
    tick_index = -1;
    let mut mark_y_pixels = center_screen.y as f32 + step_pixels;
    while mark_y_pixels < center_screen.y as f32 + height / 2.0 {
        draw_y_axis_tick(center_screen, mark_y_pixels, tick_index);
        mark_y_pixels += step_pixels;
        tick_index -= 1;
    }
    tick_index = 1;
    mark_y_pixels = center_screen.y as f32 - step_pixels;
    while mark_y_pixels > center_screen.y as f32 - height / 2.0 {
        draw_y_axis_tick(center_screen, mark_y_pixels, tick_index);
        mark_y_pixels -= step_pixels;
        tick_index += 1;
    }
    draw_axis_units(center_screen, width, height);
}

fn draw_axis_units(center_screen: Vector2D, width: f32, height: f32) {
    // La unidad de X queda debajo del eje; sus números están encima.
    let x_axis_label = "x (m)";
    let x_axis_label_dimensions = measure_text(x_axis_label, None, FONT_SIZE as u16, 1.0);
    let x_axis_label_left_x_pixels = center_screen.x as f32 + width / 2.0
        - AXIS_LABEL_EDGE_MARGIN_PIXELS
        - x_axis_label_dimensions.width;
    let x_axis_label_top_y_pixels =
        center_screen.y as f32 + AXES_STEP_LENGTH_PIXELS + X_LABEL_VERTICAL_GAP_PIXELS;

    draw_text(
        x_axis_label,
        x_axis_label_left_x_pixels,
        x_axis_label_top_y_pixels + x_axis_label_dimensions.offset_y,
        FONT_SIZE,
        WHITE,
    );

    // La unidad de Y queda a la izquierda del eje; sus números están a la derecha.
    let y_axis_label = "y (m)";
    let y_axis_label_dimensions = measure_text(y_axis_label, None, FONT_SIZE as u16, 1.0);
    let y_axis_label_left_x_pixels = center_screen.x as f32
        - AXES_STEP_LENGTH_PIXELS
        - Y_LABEL_HORIZONTAL_GAP_PIXELS
        - y_axis_label_dimensions.width;
    let y_axis_label_top_y_pixels =
        center_screen.y as f32 - height / 2.0 + AXIS_LABEL_EDGE_MARGIN_PIXELS;

    draw_text(
        y_axis_label,
        y_axis_label_left_x_pixels,
        y_axis_label_top_y_pixels + y_axis_label_dimensions.offset_y,
        FONT_SIZE,
        WHITE,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Estas pruebas ejercitan cálculo y selección sin llamar al contexto gráfico.
    fn sample_with_magnitude(x: f64) -> ElectricFieldPoint {
        ElectricFieldPoint {
            point: Vector2D { x: 0.0, y: 0.0 },
            field: Vector2D { x, y: 0.0 },
        }
    }

    #[test]
    fn field_range_ignores_zero_and_non_finite_samples() {
        let points = [0.0, f64::NAN, f64::INFINITY, 2.0, 8.0].map(sample_with_magnitude);
        assert_eq!(field_magnitude_range(&points), Some((2.0, 8.0)));
    }

    #[test]
    fn field_range_is_absent_without_valid_samples() {
        assert_eq!(field_magnitude_range(&[]), None);
        let points = [0.0, f64::NAN, f64::INFINITY].map(sample_with_magnitude);
        assert_eq!(field_magnitude_range(&points), None);
    }

    #[test]
    fn uniform_nonzero_field_gets_a_color() {
        let points = [3.0, 3.0].map(sample_with_magnitude);
        let (min, max) = field_magnitude_range(&points).expect("Hay muestras válidas");
        assert_eq!(field_color_fraction(3.0, min, max), Some(0.5));
    }

    #[test]
    fn logarithmic_scale_maps_endpoints_and_geometric_midpoint() {
        // Un rango amplio comprueba la escala logarítmica, no una interpolación lineal.
        for (magnitude, expected) in [(1e-6, 0.0), (1.0, 0.5), (1e6, 1.0)] {
            let actual = field_color_fraction(magnitude, 1e-6, 1e6).expect("Rango válido");
            assert!((actual - expected).abs() < 1e-12);
        }
        assert_eq!(field_color_fraction(1e-8, 1e-6, 1e6), Some(0.0));
        assert_eq!(field_color_fraction(1e8, 1e-6, 1e6), Some(1.0));
    }

    #[test]
    fn color_scale_rejects_invalid_data() {
        for magnitude in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(field_color_fraction(magnitude, 1.0, 10.0), None);
        }
        for (min, max) in [
            (0.0, 10.0),
            (10.0, 1.0),
            (f64::NAN, 10.0),
            (1.0, f64::INFINITY),
        ] {
            assert_eq!(field_color_fraction(2.0, min, max), None);
        }
    }

    #[test]
    fn rounded_equal_logarithms_do_not_divide_by_zero() {
        let min = 1e300_f64;
        let max = f64::from_bits(min.to_bits() + 1);
        let fraction = field_color_fraction(min, min, max).expect("Ambos límites son válidos");
        assert!(fraction.is_finite() && (0.0..=1.0).contains(&fraction));
    }

    #[test]
    fn tick_labels_follow_the_physical_step() {
        assert_eq!(axis_tick_value(5, 2.0), 10.0);
        assert_eq!(axis_tick_value(-5, 2.0), -10.0);
        assert_eq!(axis_tick_value(5, 0.5), 2.5);
    }

    #[test]
    fn cursor_selects_first_overlap_and_accepts_radius_boundary() {
        let particle = Particle {
            mass: 1.0,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1e-9,
            radius: 1.0,
        };
        let particles = [particle, particle];
        assert_eq!(
            particle_under_cursor(&particles, Vector2D { x: 1.0, y: 0.0 }),
            Some(0)
        );
        assert_eq!(
            particle_under_cursor(&particles, Vector2D { x: 2.0, y: 0.0 }),
            None
        );
        assert_eq!(
            particle_under_cursor(&[], Vector2D { x: 0.0, y: 0.0 }),
            None
        );
    }
}
