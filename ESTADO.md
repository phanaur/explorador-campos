# Estado del proyecto

Última actualización: 2026-09-09

## Fase actual

Primera partícula fija y un segmento orientado según su campo eléctrico dibujados con Macroquad. El campo se evalúa en el origen y se normaliza para darle una longitud visual fija de 40 píxeles. Las transformaciones puras están implementadas en ambos sentidos entre coordenadas físicas y de pantalla. Núcleo matemático (`src/math.rs`), núcleo físico (`src/physics.rs`) y proyección (`src/screen.rs`) permanecen desacoplados. 18 pruebas unitarias verificadas.

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
- **Representación elegida:** Macroquad sustituye a la opción inicial de Raylib.
  El usuario prefiere su integración con Cargo y evitar la compilación de la
  biblioteca C de Raylib. El arranque usa la macro de entrada y el uso mínimo de
  `async`/`await` para avanzar entre fotogramas, sin introducir otras tareas asíncronas.
- **Diseño inicial:** cálculo físico puro antes que representación gráfica.
- **Modularización y separación de responsabilidades:** división explícita del código en `math.rs` (herramienta matemática abstracta reutilizable) y `physics.rs` (dominio físico electrostático que consume `crate::math`), manteniendo `main.rs` como orquestador y punto de entrada limpio.
- **Sistema de coordenadas y proyección desacoplada:** coordenadas cartesianas 2D para el espacio físico en metros. Función pura `world_to_screen` en `src/screen.rs` que invierte el eje $Y$, escala y traslada el origen al centro de la pantalla en píxeles. Se mantiene independiente de librerías gráficas externas, delegando la conversión al tipo de la GPU a la frontera de dibujo.
- **Unidades:** Sistema Internacional (metros, culombios, newtons por culombio).
- **Representación de carga:** posición 2D y valor escalar con signo en el propio dato numérico, evitando banderas o condicionales.
- **Tipos de dominio iniciales:** estructura con campos nombrados `Vector2D` (`x: f64`, `y: f64`) para posiciones y vectores en el plano, y `Particle` (`mass: f64`, `pos: Vector2D`, `charge: f64`).
- **Semántica de copia para vectores:** `Vector2D` implementa `Clone` y `Copy` al ser un tipo pequeño de datos contiguos (16 bytes), facilitando el paso por valor en operaciones algebraicas.
- **Semántica de entidad para partículas:** `Particle` representa una entidad física con estado y no implementa `Copy`. Las funciones que inspeccionan sus propiedades reciben préstamos inmutables (`&Particle`).
- **Normalización vectorial:** método `unit` en `Vector2D` para obtener el vector unitario en la misma dirección.
- **Sobrecarga de operadores:** implementación de traits de `std::ops` (`Sub`, `Add`, `Mul<f64>`, `Div<f64>`, `Neg`, `AddAssign`, `SubAssign`) para expresar operaciones algebraicas de forma idiomática.
- **Pruebas de coma flotante:** validación con tolerancia (épsilon) y diferencia absoluta (`abs`) o módulo euclídeo (`module()`) con `assert!`, evitando la igualdad estricta de `assert_eq!`.
- **Modelo de partícula y singularidad:** `Particle` incorpora `radius: f64` modelando una corteza esférica delgada. Para distancias al cuadrado menores o iguales al radio al cuadrado ($r^2 \le R^2$), `electric_field_ch_point` retorna un vector nulo (`Vector2D { x: 0.0, y: 0.0 }`). Esto resuelve la singularidad en $r = 0$ y evita divisiones por cero (`NaN`/`inf`) sin introducir raíces cuadradas adicionales.
- **Superposición electrostática y colecciones:** función pura `total_electric_field` desacoplada del contenedor mediante una rodaja (`&[Particle]`), permitiendo evaluar campos sobre cualquier secuencia contigua sin exigir la propiedad de un `Vec`.
- **Forma de trabajo:** un lenguaje y un cambio conceptual cada vez; la IA
  actuará como tutora salvo petición explícita de implementación completa.

## Estado técnico comprobado

- Proyecto de Cargo modularizado, con Macroquad declarado como dependencia externa (`0.4.16` en `Cargo.toml`) y `Cargo.lock` actualizado.
- Toolchain estable instalada: Rust 1.98.0.
- `Vector2D` aislado en `src/math.rs` con `Copy`, `Clone`, métodos propios e implementaciones completas de `std::ops`.
- `Particle` y funciones de cálculo (`electric_field_ch_point` y `total_electric_field`) aisladas en `src/physics.rs`.
- Funciones puras `world_to_screen` y `screen_to_world` implementadas y probadas en `src/screen.rs`.
- `src/main.rs` conectando los módulos (`mod math; mod physics; mod screen;`).
- Módulo de pruebas unitarias con 18 pruebas pasando al 100% (11 en `math`, 4 en `physics`, 3 en `screen`), con comparaciones mediante diferencias absolutas o módulos vectoriales.
- La proyección se comprueba en el origen y en el punto (3, 2) m, que con una ventana de 800 × 600 píxeles y escala de 20 píxeles por metro se transforma en (460, 260) píxeles.
- La transformación inversa se comprueba con el caso (460, 260) píxeles → (3, 2) m para la misma ventana y escala.
- `src/main.rs` contiene una única función principal decorada con la macro de Macroquad. Mantiene una partícula fija fuera del bucle y, en cada fotograma, consulta el tamaño actual de la ventana, proyecta su posición y dibuja su radio físico convertido a píxeles.
- El primer muestreo visual evalúa el campo en (0, 0) m, normaliza el resultado y calcula el extremo como punto de muestreo más desplazamiento. La longitud dibujada no depende del módulo físico del campo.
- `world_to_screen` recibe las dimensiones de pantalla como `f32`, igual que las devuelve Macroquad, y mantiene sus cálculos y su resultado en `f64`.
- `cargo fmt --check`, `cargo check`, `cargo test` y `cargo clippy` terminan correctamente con este arranque. Persisten advertencias de elementos e importaciones sin usar.

## Siguiente paso

Evitar la normalización de un campo nulo antes de extender el dibujo a varios puntos de muestreo.

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
- **2026-09-06 (sesión 3):** se implementó `unit` en `Vector2D` y la función pura `electric_field_ch_point`. Se profundizó en el sistema de propiedad de Rust, diferenciando el paso por valor de `Vector2D` del paso por préstamo inmutable `&Particle`. Se corrigieron trampas de coma flotante en los tests y se alcanzaron 10 pruebas unitarias en verde.
- **2026-09-06 (sesión 4):** se diseñó e implementó la prueba de la ley del inverso del cuadrado (`test_inverse_sq_electric_field`), validando que al duplicar la distancia la intensidad cae a la cuarta parte. Se consolidó el uso del atributo `#[test]` y el blindaje con `abs()` en aserciones de coma flotante. 11 pruebas unitarias en verde.
- **2026-09-06 (sesión 5):** se añadió el campo `radius` a `Particle` y se gestionó la singularidad en `electric_field_ch_point` devolviendo vector nulo cuando $r \le R$. Se añadió la prueba unitaria `test_electric_field_null` verificando el comportamiento en el origen `(0.0, 0.0)`. 12 pruebas unitarias en verde.
- **2026-09-06 (sesión 6):** se implementaron `AddAssign` y `SubAssign` para `Vector2D`. Se creó la función `total_electric_field` y se validó la cancelación del campo por superposición en `test_total_electric_field`. 15 pruebas unitarias en verde.
- **2026-09-07:** se blindaron las aserciones de pruebas con `abs()`. Se flexibilizó `total_electric_field` con rodajas `&[Particle]`. Se modularizó el proyecto extrayendo `src/math.rs` (álgebra vectorial pura y 11 tests) y `src/physics.rs` (dominio electrostático y 4 tests), conectados mediante `crate::math` y visibilidad explícita (`pub`). 15 pruebas unitarias verificadas en verde.
- **2026-09-07 (sesión 2):** se diseñó e implementó la función pura de proyección de coordenadas `world_to_screen` en `src/screen.rs`, resolviendo el cambio de signo en $Y$, el escalado y el centrado en pantalla. Se contrastó el modelo de memoria de Rust con C# (cero coste de abstracción en tipos `Copy` en la pila). 16 pruebas unitarias verificadas en verde.
- **2026-09-09:** el usuario añadió una prueba de proyección fuera del origen y corrigió la comparación de tolerancia aplicando `abs()` a ambas componentes. 17 pruebas pasando y formato verificado tras la corrección. `cargo check` y `cargo clippy` finalizaron correctamente en la revisión previa; persisten advertencias de elementos e importaciones sin usar. Se verificó también la corrección de los enlaces `CHATGPT.md` y `CLAUDE.md` a `GEMINI.md`.
- **2026-09-09 (elección gráfica):** tras comparar Raylib, Macroquad y ggez, el usuario eligió Macroquad por su integración con Cargo y para evitar la compilación de Raylib en C. Se actualizaron las instrucciones del proyecto; la dependencia y la ventana siguen pendientes.
- **2026-09-09 (dependencia gráfica):** el usuario añadió Macroquad mediante Cargo. Se verificaron los cambios en `Cargo.toml` y `Cargo.lock` y la finalización correcta de `cargo build`, con las advertencias de elementos sin usar ya existentes. La apertura de la ventana sigue pendiente.
- **2026-09-09 (primera ventana):** el usuario implementó el bucle gráfico y simplificó el arranque a una única función principal decorada con la macro de Macroquad, tras distinguir entre definir y llamar a una función. Confirmó apertura y cierre normales. Formato, comprobación, 17 pruebas y Clippy verificados con la estructura final, con advertencias de elementos sin usar.
- **2026-09-09 (primera partícula):** el usuario creó una partícula fija fuera del bucle y la dibujó tras proyectar su posición en cada fotograma. Detectó y corrigió que el radio físico también debía multiplicarse por la escala para obtener píxeles. Se verificaron formato, compilación, 17 pruebas y Clippy; persisten advertencias por partes del núcleo físico todavía no usadas en la aplicación.
- **2026-09-09 (transformación inversa):** el usuario dedujo e implementó `screen_to_world` y añadió una prueba para recuperar (3, 2) m desde (460, 260) píxeles con una ventana de 800 × 600 y escala 20 px/m. Formato, compilación, 18 pruebas y Clippy verificados; la función aún no se usa desde la aplicación.
- **2026-09-09 (primer vector de campo):** el usuario conectó `electric_field_ch_point` con la representación gráfica. Corrigió el uso inicial del campo sin normalizar y sumó el punto de muestreo al desplazamiento para obtener el extremo absoluto. Se verificaron formato, compilación, 18 pruebas y Clippy; persisten advertencias por elementos aún no utilizados.
