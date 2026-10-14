/// Incorporación de Vector2D
use crate::math::Vector2D;

/// Constantes universales utilizadas:
const COULOMB_CONSTANT: f64 = 8.9875e9; // Nm^2C^{-2}

/// Struct de Partícula.
// Copiar estos datos escalares es seguro: la selección para el arrastre se
// guarda fuera de Particle y el cálculo recibe préstamos, sin copiar entidades.
#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub mass: f64,
    pub position: Vector2D,
    pub charge: f64,
    pub radius: f64,
}

pub struct ElectricFieldPoint {
    pub point: Vector2D,
    pub field: Vector2D,
}

// Función cálculo del campo en un punto

pub fn electric_field_at_point(particle: &Particle, point: Vector2D) -> Vector2D {
    let displacement = point - particle.position;

    let distance_squared = displacement.magnitude_squared();

    if distance_squared <= particle.radius.powi(2) {
        return Vector2D { x: 0.0, y: 0.0 };
    }

    let Some(direction) = displacement.normalized() else {
        // La zona interior ya se trató arriba. Una dirección inválida aquí es
        // un fallo numérico: NaN lo hace visible al filtrado de representación,
        // sin confundirlo con un campo físicamente nulo.
        return Vector2D {
            x: f64::NAN,
            y: f64::NAN,
        };
    };

    direction * (COULOMB_CONSTANT * particle.charge / distance_squared)
}

pub fn total_electric_field(particles: &[Particle], point: Vector2D) -> Vector2D {
    let mut total_field: Vector2D = Vector2D { x: 0.0, y: 0.0 };

    for particle in particles {
        total_field += electric_field_at_point(particle, point);
    }
    total_field
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electric_field() {
        let particle = Particle {
            mass: 1.0,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
            radius: 1_f64,
        };
        let point = Vector2D { x: 3.0, y: 4.0 };

        let electric_field = electric_field_at_point(&particle, point);

        let expected: Vector2D = Vector2D {
            x: (3.0 / 5.0),
            y: (4.0 / 5.0),
        } * (8.9875 / 25.0);

        let absolute_error = (electric_field - expected).magnitude().abs();

        assert!(absolute_error < 0.000001 && electric_field.x >= 0.0 && electric_field.y >= 0.0);
    }

    #[test]
    fn test_inverse_square_electric_field() {
        let particle = Particle {
            mass: 1.0,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
            radius: 1_f64,
        };
        let point_a = Vector2D { x: 3.0, y: 4.0 };
        let point_b = Vector2D { x: 6.0, y: 8.0 };

        let field_magnitude_a = electric_field_at_point(&particle, point_a).magnitude();
        let field_magnitude_b = electric_field_at_point(&particle, point_b).magnitude();

        assert!((field_magnitude_a - 4_f64 * field_magnitude_b).abs() < 0.000001);
    }

    #[test]
    fn test_electric_field_null() {
        let particle = Particle {
            mass: 1_f64,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1.0e-9,
            radius: 1_f64,
        };
        let point = Vector2D { x: 0.0, y: 0.0 };
        let electric_field = electric_field_at_point(&particle, point);

        assert!(electric_field.x.abs() < 0.000001 && electric_field.y.abs() < 0.000001);
    }

    #[test]
    fn test_total_electric_field() {
        // vec! expresa la colección inicial completa sin una secuencia de push.
        let particles = vec![
            Particle {
                mass: 1.0,
                position: Vector2D { x: 0.0, y: 0.0 },
                charge: 1e-9,
                radius: 1.0,
            },
            Particle {
                mass: 1.0,
                position: Vector2D { x: 3.0, y: 3.0 },
                charge: 1e-9,
                radius: 1.0,
            },
        ];

        let point = Vector2D { x: 1.5, y: 1.5 };

        let expected = Vector2D { x: 0.0, y: 0.0 };

        let total_field = total_electric_field(&particles, point);

        let difference_magnitude = (total_field - expected).magnitude();

        assert!(difference_magnitude < 0.000001);
    }

    #[test]
    fn test_negative_charge_reverses_field() {
        let positive = Particle {
            mass: 1.0,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1e-9,
            radius: 1.0,
        };
        let negative = Particle {
            charge: -positive.charge,
            ..positive
        };
        let point = Vector2D { x: 3.0, y: 4.0 };
        let sum =
            electric_field_at_point(&positive, point) + electric_field_at_point(&negative, point);
        assert!(sum.magnitude() < 1e-6);
    }

    #[test]
    fn test_field_is_unchanged_by_translation() {
        let particle = Particle {
            mass: 1.0,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1e-9,
            radius: 1.0,
        };
        let point = Vector2D { x: 3.0, y: 4.0 };
        let translation = Vector2D { x: -7.0, y: 2.0 };
        let translated = Particle {
            position: particle.position + translation,
            ..particle
        };
        let difference = electric_field_at_point(&translated, point + translation)
            - electric_field_at_point(&particle, point);
        assert!(difference.magnitude() < 1e-6);
    }

    #[test]
    fn test_shell_radius_boundary() {
        let particle = Particle {
            mass: 1.0,
            position: Vector2D { x: 0.0, y: 0.0 },
            charge: 1e-9,
            radius: 1.0,
        };
        // Se conserva la convención r <= R: campo nulo también en la superficie.
        for x in [0.5, 1.0] {
            assert!(electric_field_at_point(&particle, Vector2D { x, y: 0.0 }).magnitude() < 1e-6);
        }
        let outside = electric_field_at_point(&particle, Vector2D { x: 2.0, y: 0.0 });
        assert!((outside.x - COULOMB_CONSTANT * particle.charge / 4.0).abs() < 1e-6);
        assert!(outside.y.abs() < 1e-6);
    }
}
