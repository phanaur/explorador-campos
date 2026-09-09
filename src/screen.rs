use crate::math::Vector2D;
pub fn world_to_screen(part_pos: Vector2D, width: f32, height: f32, scale: f64) -> Vector2D {
    Vector2D {
        x: width as f64 / 2.0 + part_pos.x * scale,
        y: height as f64 / 2.0 - part_pos.y * scale,
    }
}

pub fn screen_to_world(part_pos_scr: Vector2D, width: f32, height: f32, scale: f64) -> Vector2D {
    Vector2D {
        x: (part_pos_scr.x - width as f64 / 2.0) / scale,
        y: (height as f64 / 2.0 - part_pos_scr.y) / scale,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversion() {
        let width: f32 = 1920 as f32;
        let height: f32 = 1080 as f32;
        let scale: f64 = 0.5;
        let phys_vec: Vector2D = Vector2D { x: 0_f64, y: 0_f64 };
        let solution: Vector2D = Vector2D {
            x: 960_f64,
            y: 540_f64,
        };

        let result: Vector2D = world_to_screen(phys_vec, width, height, scale) - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }

    #[test]
    fn pos_to_screen() {
        let width: f32 = 800 as f32;
        let height: f32 = 600 as f32;
        let scale: f64 = 20 as f64;

        let pos = Vector2D { x: 3_f64, y: 2_f64 };

        let solution = Vector2D {
            x: 460_f64,
            y: 260_f64,
        };

        let result = world_to_screen(pos, width, height, scale) - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }

    #[test]
    fn pos_to_world() {
        let width: f32 = 800_f32;
        let height: f32 = 600_f32;
        let scale: f64 = 20_f64;

        let pos_scr: Vector2D = Vector2D {
            x: 460_f64,
            y: 260_f64,
        };

        let solution: Vector2D = Vector2D { x: 3_f64, y: 2_f64 };

        let result: Vector2D = screen_to_world(pos_scr, width, height, scale) - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }
}
