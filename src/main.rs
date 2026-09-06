use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn module_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn module(&self) -> f64 {
        self.module_squared().sqrt()
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct Particula {
    masa: f64,
    posicion: Vector2D,
    carga: f64,
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2d_module() {
        let a = Vector2D { x: 3.0, y: 4.0 };
        let solution: f64 = 5.0;
        let result = (a.module() - solution).abs();
        assert!(result < 0.000001);
    }
}
