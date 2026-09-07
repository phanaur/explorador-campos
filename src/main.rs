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
mod math;
mod physics;
use physics::{Particle, electric_field_ch_point, total_electric_field};
mod screen;
fn main() {}
