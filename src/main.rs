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
///
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Constantes universales utilizadas:

const K: f64 = 8.9875e9; // Nm^2C^{-2}

/// En este punto, definimos el tipo matemático Vector2D, así como sus diferentes implementaciones y operaciones
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

    fn unit(self) -> Vector2D {
        self / self.module()
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

// Struct de Partícula.

struct Particle {
    mass: f64,
    pos: Vector2D,
    charge: f64,
}

// Función cálculo del campo en un punto

fn electric_field_ch_point(part: &Particle, point: Vector2D) -> Vector2D {
    let distance_vec = point - part.pos;

    let distance_sq = distance_vec.module_squared();

    let distance_unit = distance_vec.unit();

    distance_unit * (K * part.charge / distance_sq)
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

    #[test]
    fn test_electric_field() {
        let part = Particle {
            mass: 1.0,
            pos: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
        };
        let point = Vector2D { x: 3.0, y: 4.0 };

        let e_field = electric_field_ch_point(&part, point);

        let solution: Vector2D = Vector2D {
            x: (3.0 / 5.0),
            y: (4.0 / 5.0),
        } * (8.9875 / 25.0);

        let result_abs = (e_field - solution).module().abs();

        assert!(result_abs < 0.000001 && e_field.x >= 0.0 && e_field.y >= 0.0);
    }

    #[test]
    fn test_inverse_sq_electric_field() {
        let part = Particle {
            mass: 1.0,
            pos: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
        };
        let point_a = Vector2D { x: 3.0, y: 4.0 };
        let point_b = Vector2D { x: 6.0, y: 8.0 };

        let e_field_a_mod = electric_field_ch_point(&part, point_a).module();
        let e_field_b_mod = electric_field_ch_point(&part, point_b).module();

        assert!((e_field_a_mod - 4_f64 * e_field_b_mod).abs() < 0.000001);
    }
}
