/// Incorporación de Vector2D
use crate::math::Vector2D;

/// Constantes universales utilizadas:
const K: f64 = 8.9875e9; // Nm^2C^{-2}

/// Struct de Partícula.
pub struct Particle {
    pub mass: f64,
    pub pos: Vector2D,
    pub charge: f64,
    pub radius: f64,
}

// Función cálculo del campo en un punto

pub fn electric_field_ch_point(part: &Particle, point: Vector2D) -> Vector2D {
    let distance_vec = point - part.pos;

    let distance_sq = distance_vec.module_squared();

    if distance_sq <= part.radius.powi(2) {
        return Vector2D { x: 0.0, y: 0.0 };
    }

    let distance_unit = distance_vec.unit();

    distance_unit * (K * part.charge / distance_sq)
}

pub fn total_electric_field(part_list: &[Particle], point: Vector2D) -> Vector2D {
    let mut total_field: Vector2D = Vector2D { x: 0.0, y: 0.0 };

    for part in part_list {
        total_field += electric_field_ch_point(part, point);
    }
    total_field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_field() {
        let part = Particle {
            mass: 1.0,
            pos: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
            radius: 1_f64,
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
            radius: 1_f64,
        };
        let point_a = Vector2D { x: 3.0, y: 4.0 };
        let point_b = Vector2D { x: 6.0, y: 8.0 };

        let e_field_a_mod = electric_field_ch_point(&part, point_a).module();
        let e_field_b_mod = electric_field_ch_point(&part, point_b).module();

        assert!((e_field_a_mod - 4_f64 * e_field_b_mod).abs() < 0.000001);
    }

    #[test]
    fn test_electric_field_null() {
        let part = Particle {
            mass: 1_f64,
            pos: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
            radius: 1_f64,
        };
        let point = Vector2D { x: 0.0, y: 0.0 };
        let e_field = electric_field_ch_point(&part, point);

        assert!(e_field.x.abs() < 0.000001 && e_field.y.abs() < 0.000001);
    }

    #[test]
    fn test_total_electric_field() {
        let mut part_list = Vec::new();
        part_list.push(Particle {
            mass: 1.0,
            pos: Vector2D { x: 0.0, y: 0.0 },
            charge: 1e-9,
            radius: 1.0,
        });
        part_list.push(Particle {
            mass: 1.0,
            pos: Vector2D { x: 3.0, y: 3.0 },
            charge: 1e-9,
            radius: 1.0,
        });

        let point = Vector2D { x: 1.5, y: 1.5 };

        let solution = Vector2D { x: 0.0, y: 0.0 };

        let total_field = total_electric_field(&part_list, point);

        let result = (total_field - solution).module();

        assert!(result < 0.000001);
    }
}
