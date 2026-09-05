# Estado del proyecto

Última actualización: 2026-09-05

## Fase actual

Preparación del modelo físico mínimo. Definidas las bases conceptuales (coordenadas, unidades y carga); pendiente de implementar los tipos en Rust.

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
- **Forma de trabajo:** un lenguaje y un cambio conceptual cada vez; la IA
  actuará como tutora salvo petición explícita de implementación completa.

## Estado técnico comprobado

- Proyecto mínimo de Cargo preparado, sin dependencias externas.
- Toolchain estable instalada: Rust 1.98.0.
- No existe todavía código de dominio.
- No se ha añadido Raylib.
- `cargo fmt --check`, `cargo check`, `cargo test` y
  `cargo clippy -- -D warnings` terminan correctamente.
- La suite contiene todavía 0 pruebas porque no existe código físico.

## Siguiente paso

Modelar en Rust los tipos de datos mínimos para posición 2D y carga puntual, y escribir la primera prueba para el cálculo del campo eléctrico puro.

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
- **2026-09-05:** se acordaron las decisiones de diseño físico: coordenadas
  cartesianas 2D desacopladas de pantalla, unidades en el SI y representación
  escalar con signo para la carga puntual.
