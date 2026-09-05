struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn module(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    fn module_squared(&self) -> f64 {
        self.module().powi(2)
    }
}

struct Particula {
    masa: f64,
    posicion: Vector2D,
    carga: f64,
}
fn main() {}
