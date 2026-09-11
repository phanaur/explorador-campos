use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// En este punto, definimos el tipo matemático Vector2D, así como sus diferentes implementaciones y operaciones
#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

/// Métodos propios
impl Vector2D {
    pub fn magnitude_squared(self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn dot_product(self, rhs: Vector2D) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn normalized(self) -> Vector2D {
        self / self.magnitude()
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
    fn test_vector2d_magnitude() {
        let a = Vector2D { x: 3.0, y: 4.0 };
        let expected: f64 = 5.0;

        let absolute_error = (a.magnitude() - expected).abs();

        assert!(absolute_error < 0.000001);
    }

    #[test]
    fn test_vector2d_magnitude_squared() {
        let a = Vector2D { x: 3.0, y: 4.0 };
        let expected: f64 = 25.0;

        let absolute_error = (a.magnitude_squared() - expected).abs();

        assert!(absolute_error < 0.000001);
    }

    #[test]
    fn test_scalar_multiplication() {
        let a = Vector2D { x: 3.0, y: 4.0 };
        let scalar: f64 = 3.0;
        let expected = Vector2D { x: 9.0, y: 12.0 };

        let difference_magnitude = (a * scalar - expected).magnitude().abs();

        assert!(difference_magnitude < 0.000001);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let expected = Vector2D { x: 1.0, y: 0.0 };
        a -= b;

        let difference = a - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }

    #[test]
    fn test_sub() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let expected = Vector2D { x: 1.0, y: 0.0 };
        let difference = a - b;

        let difference_from_expected = difference - expected;

        assert!(
            difference_from_expected.x.abs() < 0.000001
                && difference_from_expected.y.abs() < 0.000001
        );
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let expected = Vector2D { x: 3.0, y: 2.0 };
        a += b;

        let difference = a - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }

    #[test]
    fn test_add() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let b = Vector2D { x: 1.0, y: 1.0 };
        let expected = Vector2D { x: 3.0, y: 2.0 };
        let sum = a + b;

        let difference = sum - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }

    #[test]
    fn test_div() {
        let a = Vector2D { x: 2.0, y: 2.0 };
        let scalar: f64 = 2.0;
        let expected = Vector2D { x: 1.0, y: 1.0 };

        let difference = a / scalar - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }

    #[test]
    fn test_mul() {
        let a = Vector2D { x: 2.0, y: 1.0 };
        let scalar: f64 = 2.0;
        let expected = Vector2D { x: 4.0, y: 2.0 };

        let difference_magnitude = (a * scalar - expected).magnitude().abs();

        assert!(difference_magnitude < 0.000001);
    }

    #[test]
    fn test_vector2d_dot_product() {
        let a = Vector2D { x: 1.0, y: 1.0 };
        let b = Vector2D { x: 2.0, y: 2.0 };
        let expected: f64 = 4.0;

        let absolute_error = (a.dot_product(b) - expected).abs();

        assert!(absolute_error < 0.000001);
    }

    #[test]
    fn test_vector2d_normalized() {
        let a = Vector2D { x: 2.0, y: 2.0 };
        let magnitude_squared: f64 = 8.0;
        let expected = Vector2D {
            x: 2.0 / magnitude_squared.sqrt(),
            y: 2.0 / magnitude_squared.sqrt(),
        };

        let difference = a.normalized() - expected;

        assert!(difference.x.abs() < 0.000001 && difference.y.abs() < 0.000001);
    }
}
