use crate::math::Vector2D;
pub fn world_to_screen(
    world_point: Vector2D,
    width: f32,
    height: f32,
    pixels_per_meter: f64,
) -> Vector2D {
    Vector2D {
        x: width as f64 / 2.0 + world_point.x * pixels_per_meter,
        y: height as f64 / 2.0 - world_point.y * pixels_per_meter,
    }
}

pub fn screen_to_world(
    screen_point: Vector2D,
    width: f32,
    height: f32,
    pixels_per_meter: f64,
) -> Vector2D {
    Vector2D {
        x: (screen_point.x - width as f64 / 2.0) / pixels_per_meter,
        y: (height as f64 / 2.0 - screen_point.y) / pixels_per_meter,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_world_origin_maps_to_screen_center() {
        let width: f32 = 1920 as f32;
        let height: f32 = 1080 as f32;
        let pixels_per_meter: f64 = 0.5;
        let world_origin: Vector2D = Vector2D { x: 0_f64, y: 0_f64 };
        let expected: Vector2D = Vector2D {
            x: 960_f64,
            y: 540_f64,
        };

        let difference: Vector2D =
            world_to_screen(world_origin, width, height, pixels_per_meter) - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }

    #[test]
    fn test_world_point_to_screen() {
        let width: f32 = 800 as f32;
        let height: f32 = 600 as f32;
        let pixels_per_meter: f64 = 20 as f64;

        let world_point = Vector2D { x: 3_f64, y: 2_f64 };

        let expected = Vector2D {
            x: 460_f64,
            y: 260_f64,
        };

        let difference = world_to_screen(world_point, width, height, pixels_per_meter) - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }

    #[test]
    fn test_screen_point_to_world() {
        let width: f32 = 800_f32;
        let height: f32 = 600_f32;
        let pixels_per_meter: f64 = 20_f64;

        let screen_point: Vector2D = Vector2D {
            x: 460_f64,
            y: 260_f64,
        };

        let expected: Vector2D = Vector2D { x: 3_f64, y: 2_f64 };

        let difference: Vector2D =
            screen_to_world(screen_point, width, height, pixels_per_meter) - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }
}
