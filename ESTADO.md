# Estado del proyecto

Última actualización: 2026-09-23

## Fase actual

Dos partículas simultáneas de signos opuestos (`Vec<Particle>`), cada una arrastrable de forma independiente, con cuadrícula densa de vectores orientados, gradiente de color logarítmico (HSL de azul a rojo según la intensidad del campo eléctrico total) e indicador visual de signo de carga sobre cada partícula (cruz roja para cargas positivas, guion azul horizontal para negativas y ningún signo para carga nula). El campo de cada punto de la cuadrícula se calcula una sola vez por fotograma y se almacena en un `Vec<ElectricFieldPoint>` (posición y vector de campo), evitando invocar `total_electric_field` dos veces por punto: una primera pasada calcula y guarda, y además determina los extremos reales de magnitud del fotograma para la escala de color; una segunda pasada solo lee esos datos para dibujar. El espacio visible se deduce dinámicamente con `screen_to_world`, barriendo en pasos físicos de 0.5 m con segmentos de 5 px. Núcleo matemático (`src/math.rs`), núcleo físico (`src/physics.rs`) y proyección (`src/screen.rs`) permanecen desacoplados. Parte del andamiaje temporal de iteradores ya se ha sustituido por bucles y estructuras de control explícitas escritas por el usuario. 18 pruebas unitarias verificadas.

**El punto 6 del alcance de la primera etapa y el indicador de signo posterior están completados.** El proyecto vuelve a un punto de parada consciente: antes de modificar más código se decidirá si se continúa con un nuevo cambio conceptual, se pausa o se cierra esta fase.

## Objetivo acordado

Construir desde cero una aplicación de escritorio en Rust que permita explorar
visualmente campos físicos bidimensionales. La primera versión se limitará al
campo eléctrico de cargas puntuales.

El proyecto es un medio de aprendizaje y disfrute, sin fecha límite ni
obligación de convertirse en un producto terminado o en material de portfolio.

El usuario ha indicado (2026-09-11) que, a más largo plazo, este código se
incorporará a un laboratorio virtual de física más amplio. Esto no introduce
plazo ni obliga a un acabado concreto ahora: el ritmo sigue siendo "hasta
estar satisfecho" con cada parte, no una fecha de entrega.

## Alcance de la primera etapa

1. Representar una carga puntual y una posición en dos dimensiones.
2. Calcular el campo eléctrico en un punto mediante una función pura.
3. Comprobar con pruebas casos de simetría, cancelación y ley del inverso del
   cuadrado.
4. Dibujar una cuadrícula de vectores para una carga fija. (Completado con gradiente de color)
5. Permitir mover esa carga con el ratón. (Completado)
6. Añadir una segunda carga y revisar lo aprendido. (Completado)

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
- **Tipos de dominio iniciales:** estructura con campos nombrados `Vector2D` (`x: f64`, `y: f64`) para posiciones y vectores en el plano, y `Particle` (`mass: f64`, `position: Vector2D`, `charge: f64`).
- **Semántica de copia para vectores:** `Vector2D` implementa `Clone` y `Copy` al ser un tipo pequeño de datos contiguos (16 bytes), facilitando el paso por valor en operaciones algebraicas.
- **Semántica de entidad para partículas:** `Particle` representa una entidad física con estado y no implementa `Copy`. Las funciones que inspeccionan sus propiedades reciben préstamos inmutables (`&Particle`).
- **Normalización vectorial:** método `normalized` en `Vector2D` para obtener el vector unitario en la misma dirección.
- **Sobrecarga de operadores:** implementación de traits de `std::ops` (`Sub`, `Add`, `Mul<f64>`, `Div<f64>`, `Neg`, `AddAssign`, `SubAssign`) para expresar operaciones algebraicas de forma idiomática.
- **Pruebas de coma flotante:** validación con tolerancia (épsilon) y diferencia absoluta (`abs`) o magnitud euclídea (`magnitude()`) con `assert!`, evitando la igualdad estricta de `assert_eq!`.
- **Modelo de partícula y singularidad:** `Particle` incorpora `radius: f64` modelando una corteza esférica delgada. Para distancias al cuadrado menores o iguales al radio al cuadrado ($r^2 \le R^2$), `electric_field_at_point` retorna un vector nulo (`Vector2D { x: 0.0, y: 0.0 }`). Esto resuelve la singularidad en $r = 0$ y evita divisiones por cero (`NaN`/`inf`) sin introducir raíces cuadradas adicionales.
- **Superposición electrostática y colecciones:** función pura `total_electric_field` desacoplada del contenedor mediante una rodaja (`&[Particle]`), permitiendo evaluar campos sobre cualquier secuencia contigua sin exigir la propiedad de un `Vec`.
- **Muestreo en cuadrícula de paso fijo:** límites del mundo calculados dinámicamente según la ventana con `screen_to_world`. Se evalúan y dibujan en modo inmediato en cada fotograma.
- **Gradiente de color logarítmico (HSL):** representación de la magnitud del campo mediante `hsl_to_rgb` con saturación 1.0 y luminosidad 0.5. El tono varía entre 0.66 (azul, campo débil) y 0.0 (rojo, campo intenso) normalizando el logaritmo del módulo acotado con `clamp`.
- **Muestreo denso de visualización:** paso físico de 0.5 m y longitud de segmento de 5 px para lograr resolución visual adecuada sin solapamientos.
- **Arrastre con el ratón:** el clic inicial sobre la partícula activa un estado `is_dragging`; mientras el botón izquierdo sigue pulsado, su posición se actualiza con el cursor convertido a coordenadas físicas. La liberación termina el arrastre.
- **Señal visual de arrastre:** durante el arrastre aumenta únicamente el radio usado al dibujar, sin modificar `Particle::radius` ni el cálculo del campo.
- **Nomenclatura:** identificadores descriptivos en inglés para constantes, API y valores que viven fuera de una fórmula local; unidades incluidas cuando aclaran la interpretación. Se conservan nombres matemáticos breves cuando son convencionales y su alcance es pequeño.
- **Forma de trabajo:** un lenguaje y un cambio conceptual cada vez; la IA
  actuará como tutora salvo petición explícita de implementación completa.
- **Colección de partículas:** `Vec<Particle>` en vez de un array de tamaño fijo, decidido porque el usuario prevé añadir y quitar partículas en tiempo de ejecución (no solo tener siempre dos).
- **Identidad de la partícula arrastrada:** no se guarda un id persistente en `Particle`. Basta con hallar la posición (`usize`) dentro del `Vec` en el momento del clic mediante `Vec::iter().position(...)`, envuelta en `Option<usize>` mientras no haya ninguna arrastrada. Elegido explícitamente por el usuario tras valorar que no necesita que esa identidad sobreviva a inserciones o borrados.
- **Cálculo del campo en dos pasadas:** se prefiere calcular y guardar el vector de campo de cada punto de la cuadrícula (`ElectricFieldPoint { point, field }` en `physics.rs`) en una primera pasada, y usar esos datos ya calculados en la segunda pasada para dibujar, evitando invocar `total_electric_field` dos veces por punto. El usuario eligió esta vía exacta (basada en los extremos reales del fotograma) en vez de la heurística barata (extremo del campo en la superficie de cada partícula), asumiendo el coste de mantener una colección adicional a cambio de una escala de color correcta a cualquier número de partículas.
- **Exclusión de magnitudes degeneradas en la escala de color:** al buscar los extremos reales de magnitud del fotograma, se descartan los puntos con magnitud `0.0` (zona de exclusión dentro del radio de una partícula) antes de compararlos. Sin esta exclusión, un mínimo de magnitud exactamente `0.0` provoca `ln(0) = -inf`, y la normalización logarítmica de la línea siguiente da como resultado `NaN`, coloreando los segmentos de forma indistinguible del fondo. Se corrigió tras depuración empírica con `println!`, no por lectura estática del código.
- **Código sugerido por el editor como andamiaje temporal:** el usuario aceptó puntualmente construcciones con iteradores (`iter()`, `map()`, `collect()`, `position()`, `enumerate()`) sugeridas por el editor para comprobar que la lógica de selección y dibujo múltiple funcionaba, con la intención declarada de sustituirlas más adelante por bucles estándar que sepa escribir y explicar sin ayuda. Documentado con comentarios en la cabecera de `main.rs`.
- **Indicador de signo de carga sobre las partículas:** se dibuja una cruz roja sobre partículas con carga positiva y un guion azul horizontal sobre partículas con carga negativa, utilizando el color blanco de fondo del disco como contraste neutro. La dimensión del símbolo se escala a píxeles multiplicando el radio físico por `PIXELS_PER_METER / 2.0`. Para evitar duplicar memoria o crear vectores temporales paralelos, `draw_particles` recibe una rodaja `&[Particle]` directamente del contenedor principal y evalúa el signo mediante bifurcación condicional (`if` / `else if`).

## Estado técnico comprobado

- Proyecto de Cargo modularizado, con Macroquad declarado como dependencia externa (`0.4.16` en `Cargo.toml`) y `Cargo.lock` actualizado.
- Toolchain estable instalada: Rust 1.98.0.
- `Vector2D` aislado en `src/math.rs` con `Copy`, `Clone`, métodos propios e implementaciones completas de `std::ops`.
- `Particle` y funciones de cálculo (`electric_field_at_point` y `total_electric_field`) aisladas en `src/physics.rs`.
- Funciones puras `world_to_screen` y `screen_to_world` implementadas y probadas en `src/screen.rs`.
- `src/main.rs` conectando los módulos (`mod math; mod physics; mod screen;`).
- Módulo de pruebas unitarias con 18 pruebas pasando al 100% (11 en `math`, 4 en `physics`, 3 en `screen`), con comparaciones mediante diferencias absolutas o módulos vectoriales.
- `main.rs` maneja `Vec<Particle>` con dos partículas de signos opuestos: cuadrícula de campo total calculada y almacenada en `Vec<ElectricFieldPoint>` una vez por fotograma, extremos reales de magnitud (excluyendo `0.0`) para la escala de color, y dibujo de cada partícula, su signo y cada segmento a partir de esos datos ya calculados.
- Arrastre por índice: el clic localiza la partícula bajo el cursor con un bucle `for` indexado sobre `mouse_is_over_particle`, guarda ese índice en `Option<usize>`, mueve solo esa partícula mientras se mantiene pulsado mediante `if let Some(index)`, y solo agranda su radio de dibujo.
- Sustitución de iteradores por bucles estándar en `main.rs`: extracción inicial de radios con `for particle in &particles`, detección de ratón sobre partícula con `for particle in &particles` y `push`, y proyección de partículas a pantalla con bucle explícito.
- `draw_particles` recibe `&[Vector2D]`, `&[f64]` y `&[Particle]` encadenados con `zip`. Dibuja el disco blanco y, sobre él, una cruz roja para `charge > 0.0` o un segmento azul horizontal para `charge < 0.0`, escalados correctamente a píxeles.
- Partículas de prueba en `main.rs` inicializadas con cargas opuestas (`1e-9` y `-1e-9`) usando la macro `vec![]`.
- `Vector2D` deriva también `Debug`, incorporado durante una sesión anterior de depuración.
- `cargo fmt --check`, `cargo check`, `cargo test` (18 pruebas) y `cargo clippy` terminan correctamente; Clippy solo mantiene las advertencias de `dot_product` y `Particle::mass` sin usar y el uso deliberado de un bucle por índices durante la transición desde iteradores.

## Siguiente paso

Decidir conscientemente el próximo cambio conceptual o cerrar o pausar esta fase antes de modificar más código.

## Fuera del alcance actual

- Líneas de campo y superficies equipotenciales.
- Campos gravitatorios, magnéticos o tridimensionales.
- Integración numérica de trayectorias.
- Exportación de datos o gráficas.
- Interfaz web, WebAssembly o aplicación móvil.
- Base de datos y datos de alumnos.
- GPU, paralelismo, arquitectura ECS o acabado visual avanzado.
- Teoría, unidades didácticas o conversión en un laboratorio completo.
- Márgenes de pantalla que impidan dibujar en los bordes.
- Restricción para impedir que dos partículas se solapen o coexistan en la misma posición.

## Preguntas aplazadas

- Si se añadirá geometría de punta de flecha a los segmentos o se mantendrán como líneas de dirección con color.

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
- **2026-09-09 (cuadrícula de vectores):** el usuario implementó el muestreo en cuadrícula mediante dos bucles `while` anidados con paso físico fijo de 2 m. El rango visible se calcula proyectando la esquina inferior derecha con `screen_to_world` y aprovechando la simetría respecto al centro. Se evalúa y dibuja en cada punto la dirección del campo eléctrico mediante un segmento orientado de 40 px. Formato, compilación, 18 pruebas y Clippy verificados; persisten advertencias por elementos no usados.
- **2026-09-09 (gradiente logarítmico HSL):** el usuario implementó el coloreado del campo mediante una escala logarítmica normalizada mapeada al tono HSL (azul en campo débil, rojo en campo intenso). Se refinó la cuadrícula a un paso de 0.5 m con segmentos de 5 px y se calcularon los extremos fuera del bucle. Formato, compilación, 18 pruebas y Clippy verificados; persisten advertencias por elementos no usados.
- **2026-09-11:** se extrajeron las constantes de visualización y se revisó la nomenclatura de `main.rs`, `math.rs`, `physics.rs` y `screen.rs`, sin cambiar el comportamiento. La API matemática usa `magnitude`, `magnitude_squared`, `dot_product` y `normalized`; el campo eléctrico se calcula mediante `electric_field_at_point`. Formato, compilación, 18 pruebas y Clippy verificados; persisten cuatro advertencias por elementos aún no usados.
- **2026-09-11 (arrastre):** el usuario implementó el movimiento de la partícula mediante un estado persistente entre fotogramas y distinguió `is_mouse_button_pressed`, `is_mouse_button_down` e `is_mouse_button_released`. El arrastre solo comienza sobre la partícula, continúa fuera de su radio hasta soltar y amplía temporalmente su tamaño dibujado sin alterar el radio físico. Se extrajo el dibujo a una función propia. Formato, compilación, 18 pruebas y Clippy verificados; persisten cuatro advertencias anteriores por elementos sin usar.
- **2026-09-11 (segunda carga, punto 6 completado):** el usuario pasó de una `Particle` a `Vec<Particle>` con dos cargas, adaptando el arrastre para seleccionar por índice (`Option<usize>` hallado con `position()`) cuál se mueve y cuál agranda su radio de dibujo. Para la escala de color eligió deliberadamente calcular el campo de la cuadrícula en dos pasadas, guardando cada punto y su vector de campo en un `Vec<ElectricFieldPoint>` para no invocar `total_electric_field` dos veces, en vez de una heurística barata basada en el radio de las partículas. Aceptó puntualmente construcciones con iteradores sugeridas por el editor como andamiaje temporal, con la intención declarada de sustituirlas más adelante por bucles estándar, y lo documentó con comentarios en `main.rs`. Varias rondas de revisión (REVISA) detectaron y el usuario corrigió: dos variables sombreadas que anulaban el cálculo (posición de pantalla de las partículas e índice de arrastre), el reseteo de los extremos de magnitud colocado en el punto equivocado del bucle, y el escalado del radio de dibujo aplicado a todas las partículas en vez de solo a la arrastrada. Con depuración empírica (`println!`) se encontró y corrigió un bug no visible por lectura estática: un mínimo de magnitud exactamente `0.0` producía `ln(0) = -inf` y por tanto `NaN` en el color, invisible sobre el fondo negro; se resolvió excluyendo del cálculo de extremos los puntos con magnitud `0.0` (o infinita). Formato, compilación y 18 pruebas verificados; Clippy señala tres avisos de estilo nuevos sin relevancia funcional. Con el punto 6 completado, queda abierto el punto de parada consciente del alcance de la primera etapa; se decide continuar, con el indicador de signo de carga como siguiente paso.
- **2026-09-12 (indicador de signo y transición a bucles explícitos):** el usuario implementó la distinción visual de carga sobre el disco blanco de cada partícula: una cruz roja para cargas positivas y un segmento horizontal azul para cargas negativas. Para no introducir colecciones redundantes, adaptó `draw_particles` para recibir `&[Particle]` además de posiciones y radios, encadenándolos con un segundo `zip` en el bucle de dibujo. Se descartó el uso inicial de `match` con comparaciones relacionales al aclarar la diferencia entre coincidencia de patrones y bifurcación condicional (`if` / `else if`), y se corrigió el escalado del radio a píxeles para calcular las dimensiones del símbolo. En una segunda parte de la sesión, el usuario inició la sustitución del andamiaje temporal de iteradores (`iter()`, `map()`, `collect()`, `position()`) por bucles estándar y comprobaciones explícitas en `main.rs`, resolviendo errores de propiedad y consumo mediante préstamos inmutables (`&particles`), tipos `Copy` en vectores primitivos y extracción de `Option<usize>` con `if let Some(index)`. Formato, compilación y 18 pruebas unitarias verificados.
- **2026-09-23 (sincronización y cierre documental):** al preparar el push se detectó que `origin/main` ya contenía el indicador de signo y la transición parcial a bucles explícitos. Se conservaron esos cambios remotos, se actualizó el README que aún describía el proyecto como no iniciado y se situó el proyecto en un nuevo punto de decisión consciente.
