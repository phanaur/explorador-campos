use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// En este punto, definimos el tipo matemático Vector2D, así como sus diferentes implementaciones y operaciones
#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

/// Métodos propios
impl Vector2D {
    pub fn module_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn module(self) -> f64 {
        self.module_squared().sqrt()
    }

    pub fn scalar_prod(self, rhs: Vector2D) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn unit(self) -> Vector2D {
        self / self.module()
    }
}

/// Implementaciones de traits de std::ops para Vector2D
impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
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

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

/// Tests unitarios para Vector2D
#[cfg(test)]
mod test {
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
    fn test_sub_assign() {
        let mut a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let solution = Vector2D { x: 1.0, y: 0.0 };
        a -= b;

        let result = a - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }

    #[test]
    fn test_sub() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let solution = Vector2D { x: 1.0, y: 0.0 };
        let sub = a - b;

        let result = sub - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let solution = Vector2D { x: 3.0, y: 2.0 };
        a += b;

        let result = a - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }

    #[test]
    fn test_add() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let solution = Vector2D { x: 3.0, y: 2.0 };
        let sub = a + b;

        let result = sub - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }

    #[test]
    fn test_div() {
        let a = Vector2D { x: 2.0, y: 2.0 };
        let scalar: f64 = 2.0;
        let solution = Vector2D { x: 1.0, y: 1.0 };

        let result = a / scalar - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
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

    #[test]
    fn test_vector2d_unit() {
        let a = Vector2D { x: 2.0, y: 2.0 };
        let number: f64 = 8.0;
        let solution = Vector2D {
            x: 2.0 / number.sqrt(),
            y: 2.0 / number.sqrt(),
        };

        let result = a.unit() - solution;

        assert!(result.x.abs() < 0.000001 && result.y.abs() < 0.000001);
    }
}
