use crate::math::Vector2D;
pub fn world_to_screen(part_pos: Vector2D, width: i32, height: i32, scale: f64) -> Vector2D {
    Vector2D {
        x: width as f64 / 2.0 + part_pos.x * scale,
        y: height as f64 / 2.0 - part_pos.y * scale,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversion() {
        let width: i32 = 1920;
        let height: i32 = 1080;
        let scale: f64 = 0.5;
        let phys_vec: Vector2D = Vector2D { x: 0_f64, y: 0_f64 };
        let solution: Vector2D = Vector2D {
            x: 960_f64,
            y: 540_f64,
        };

        let result: Vector2D = world_to_screen(phys_vec, width, height, scale) - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }
}
