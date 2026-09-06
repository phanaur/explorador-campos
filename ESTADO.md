# Estado del proyecto

Última actualización: 2026-09-06

## Fase actual

Modelo físico mínimo en desarrollo. Estructura `Vector2D` con álgebra vectorial completa (`Add`, `Sub`, `Mul<f64>`, `Div<f64>`, `Neg`, `scalar_prod`, `module`, `module_squared`) y estructura `Particula`. 8 pruebas unitarias activas y verificadas.

## Objetivo acordado

Construir desde cero una aplicación de escritorio en Rust que permita explorar
visualmente campos físicos bidimensionales. La primera versión se limitará al
campo eléctrico de cargas puntuales.

El proyecto es un medio de aprendizaje y disfrute, sin fecha límite ni
obligación de convertirse en un producto terminado o en material de portfolio.

## Alcance de la primera etapa

1. Representar una carga puntual y una posición en dos dimensiones.
2. Calcular el campo eléctrico en un punto mediante una función pura.
3. Comprobar con pruebas casos de simetría, cancelación y ley del inverso del
   cuadrado.
4. Dibujar una cuadrícula de vectores para una carga fija.
5. Permitir mover esa carga con el ratón.
6. Añadir una segunda carga y revisar lo aprendido.

Al completar el punto 6 se detendrá el desarrollo para decidir conscientemente
si se continúa, se cierra o se redefine el proyecto.

## Decisiones tomadas

- **Lenguaje:** Rust estable. Se elige por su explicitud, su sistema de tipos,
  Cargo y el interés del usuario por comprender propiedad y referencias.
- **Formato:** aplicación de escritorio autocontenida. No se comenzará por la
  web para evitar mezclar lenguajes y plataformas.
- **Representación prevista:** Raylib cuando llegue la fase gráfica, porque su
  modelo de dibujo es sencillo y el usuario ya lo conoce.
- **Diseño inicial:** cálculo físico puro antes que representación gráfica.
- **Sistema de coordenadas:** cartesianas 2D para el espacio físico, independientes de la pantalla. La conversión a píxeles (zoom, desplazamiento) se delega a la capa gráfica futura.
- **Unidades:** Sistema Internacional (metros, culombios, newtons por culombio).
- **Representación de carga:** posición 2D y valor escalar con signo en el propio dato numérico, evitando banderas o condicionales.
- **Tipos de dominio iniciales:** estructura con campos nombrados `Vector2D` (`x: f64`, `y: f64`) para posiciones y vectores en el plano, y `Particula` compuesta por posición y carga.
- **Semántica de copia para vectores:** `Vector2D` implementa `Clone` y `Copy` al ser un tipo pequeño de datos contiguos (16 bytes), facilitando el paso por valor en operaciones algebraicas.
- **Sobrecarga de operadores:** implementación de traits de `std::ops` (iniciado con `Sub`) para expresar operaciones físicas y matemáticas de forma idiomática.
- **Pruebas de coma flotante:** validación con tolerancia (épsilon) y diferencia absoluta (`abs`) con `assert!`, evitando la igualdad estricta de `assert_eq!`.
- **Forma de trabajo:** un lenguaje y un cambio conceptual cada vez; la IA
  actuará como tutora salvo petición explícita de implementación completa.

## Estado técnico comprobado

- Proyecto mínimo de Cargo preparado, sin dependencias externas.
- Toolchain estable instalada: Rust 1.98.0.
- Tipos `Vector2D` y `Particula` definidos en `src/main.rs`.
- `Vector2D` cuenta con `Copy`, `Clone`, métodos `module`, `module_squared`, `scalar_prod`, e implementaciones de `std::ops` (`Sub`, `Add`, `Mul<f64>`, `Div<f64>`, `Neg`).
- Módulo de pruebas unitarias configurado con `#[cfg(test)]` y 8 pruebas activas pasando al 100%.
- No se ha añadido Raylib.

## Siguiente paso

Diseñar e implementar el cálculo de desplazamiento o vector de distancia relativa entre partículas (`Particula`).

## Fuera del alcance actual

- Líneas de campo y superficies equipotenciales.
- Campos gravitatorios, magnéticos o tridimensionales.
- Integración numérica de trayectorias.
- Exportación de datos o gráficas.
- Interfaz web, WebAssembly o aplicación móvil.
- Base de datos y datos de alumnos.
- GPU, paralelismo, arquitectura ECS o acabado visual avanzado.
- Teoría, unidades didácticas o conversión en un laboratorio completo.

## Preguntas aplazadas

- Cómo transformar coordenadas físicas en coordenadas de pantalla.
- Qué escala visual usar para campos con módulos muy diferentes.
- Si se representará primero el vector, el módulo mediante color o ambos.

Estas preguntas no deben resolverse hasta que afecten al siguiente paso.

## Registro de sesiones

- **2026-08-29:** se definieron el propósito, el alcance inicial, las reglas de
  tutoría y la estructura documental del repositorio. Se instaló Rust estable
  y se verificó el proyecto mínimo con Cargo y Clippy.
- **2026-09-05:** se acordaron las decisiones de diseño físico (coordenadas cartesianas 2D, unidades SI, carga escalar) y los contratos de tutoría. Se implementaron en Rust las estructuras `Vector2D` (con métodos `module` y `module_squared`) y `Particula`.
- **2026-09-06:** se estructuró el módulo de pruebas unitarias con `#[cfg(test)]` y comprobación con tolerancia (`abs` y `assert!`) para `f64`. Se optimizó `module_squared`, se derivaron `Clone` y `Copy` para `Vector2D` y se implementó el trait `std::ops::Sub`.
- **2026-09-06 (sesión 2):** se completó la base algebraica de `Vector2D` resolviendo sobrecarga homogénea (`Add`, `Sub`), heterogénea con escalares (`Mul<f64>`, `Div<f64>`), negación de vectores (`Neg`) y producto escalar (`scalar_prod`). Se corrigió el uso de `#[test]` individual y se alcanzaron 8 pruebas unitarias en verde.

