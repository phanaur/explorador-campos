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
use physics::{Particle, electric_field_ch_point, total_electric_field};
mod screen;
use macroquad::{color::hsl_to_rgb, prelude::*};

use crate::screen::{screen_to_world, world_to_screen};

#[macroquad::main("BasicShapes")]
async fn main() {
    let part: Particle = Particle {
        mass: 1.0,
        pos: Vector2D { x: 3.0, y: 2.0 },
        charge: 1e-9,
        radius: 0.5,
    };

    let max_field_value = electric_field_ch_point(
        &part,
        Vector2D {
            x: part.pos.x + part.radius * 1.01,
            y: part.pos.y,
        },
    )
    .module();
    let min_field_value = max_field_value / 5e3;

    loop {
        clear_background(BLACK);

        let width = screen_width();
        let height = screen_height();
        let scale = 20_f64;
        let part_scr = world_to_screen(part.pos, width, height, scale);

        draw_circle(
            part_scr.x as f32,
            part_scr.y as f32,
            (part.radius * scale) as f32,
            WHITE,
        );

        let cuad: Vector2D = screen_to_world(
            Vector2D {
                x: width as f64,
                y: height as f64,
            },
            width,
            height,
            scale,
        );
        let mut x = -cuad.x;
        let mut y = cuad.y;
        while y < -cuad.y {
            while x < cuad.x {
                let point: Vector2D = Vector2D { x, y };

                let field_in_point: Vector2D = electric_field_ch_point(&part, point);

                if field_in_point.module() != 0.0 {
                    let field_in_point_dir: Vector2D = field_in_point.unit();

                    let end_arrow_field: Vector2D = field_in_point_dir * 5_f64 / scale + point;

                    let point_scr: Vector2D = world_to_screen(point, width, height, scale);
                    let end_arrow_field_src: Vector2D =
                        world_to_screen(end_arrow_field, width, height, scale);

                    let clamp = field_in_point
                        .module()
                        .clamp(min_field_value, max_field_value);
                    let t = (clamp.ln() - min_field_value.ln())
                        / (max_field_value.ln() - min_field_value.ln());
                    let color_hue = ((1_f64 - t) * 0.66_f64) as f32;
                    let color = hsl_to_rgb(color_hue, 1.0, 0.5);
                    draw_line(
                        point_scr.x as f32,
                        point_scr.y as f32,
                        end_arrow_field_src.x as f32,
                        end_arrow_field_src.y as f32,
                        2.0,
                        color,
                    );
                }
                x += 0.5_f64;
            }
            x = -cuad.x;
            y += 0.5_f64;
        }

        next_frame().await
    }
}
