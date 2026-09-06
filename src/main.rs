use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn module_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn module(self) -> f64 {
        self.module_squared().sqrt()
    }

    fn scalar_prod(self, rhs: Vector2D) -> f64 {
        self.x * rhs.x + self.y * rhs.y
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

impl Mul<f64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f64) -> Vector2D {
        Vector2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f64) -> Vector2D {
        Vector2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Vector2D {
    type Output = Vector2D;

    fn neg(self) -> Self::Output {
        Vector2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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

    #[test]
    fn test_vector2d_module_squared() {
        let a = Vector2D { x: 3.0, y: 4.0 };
        let solution: f64 = 25.0;

        let result = (a.module_squared() - solution).abs();

        assert!(result < 0.000001);
    }

    #[test]
    fn test_scalar() {
        let a = Vector2D { x: 3.0, y: 4.0 };
        let scalar: f64 = 3.0;
        let solution = Vector2D { x: 9.0, y: 12.0 };

        let result = (a * scalar - solution).module().abs();

        assert!(result < 0.000001);
    }

    #[test]
    fn test_sub() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let solution = Vector2D { x: 1.0, y: 0.0 };
        let sub = a - b;

        let result = sub - solution;

        assert!(result.x < 0.000001 && result.y < 0.000001);
    }

    #[test]
    fn test_add() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let solution = Vector2D { x: 3.0, y: 2.0 };
        let sub = a + b;

        let result = sub - solution;

        assert!(result.x < 0.000001 && result.y < 0.000001);
    }

    #[test]
    fn test_div() {
        let a = Vector2D { x: 2.0, y: 2.0 };
        let scalar: f64 = 2.0;
        let solution = Vector2D { x: 1.0, y: 1.0 };

        let result = a / scalar - solution;

        assert!(result.x < 0.000001 && result.y < 0.000001);
    }

    #[test]
    fn test_mul() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let scalar: f64 = 2.0;
        let solution = Vector2D { x: 4.0, y: 2.0 };

        let result = (a * scalar - solution).module().abs();

        assert!(result < 0.000001);
    }

    #[test]
    fn test_vector2d_scalar_prod() {
        let a = Vector2D { x: 1.0, y: 1.0 };
        let b = Vector2D { x: 2.0, y: 2.0 };
        let solution: f64 = 4.0;

        let result = (a.scalar_prod(b) - solution).abs();

        assert!(result < 0.000001);
    }
}
