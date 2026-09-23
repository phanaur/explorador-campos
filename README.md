# Explorador de campos

Proyecto personal de aprendizaje para construir en Rust una aplicación de
escritorio que permita explorar campos físicos bidimensionales.

La primera etapa se limitará al campo eléctrico producido por una o dos cargas
puntuales. El objetivo inicial no es crear un laboratorio completo, sino
desarrollar y comprobar un núcleo físico pequeño antes de representarlo de
forma visual.

## Estado

La primera etapa está completada. La aplicación representa dos cargas puntuales
arrastrables, calcula su campo eléctrico total y lo dibuja sobre una cuadrícula
con un gradiente logarítmico de intensidad. Cada carga muestra además un signo
visual: rojo para la positiva y azul para la negativa. El núcleo matemático, el
cálculo físico y la proyección de coordenadas permanecen separados y están
respaldados por 18 pruebas unitarias.

El punto actual y el siguiente paso se mantienen en [ESTADO.md](ESTADO.md).
Las reglas para trabajar con asistentes de IA están en [AGENTS.md](AGENTS.md).

## Primera etapa

- [x] Calcular el campo de una carga puntual en dos dimensiones.
- [x] Verificar propiedades conocidas mediante pruebas.
- [x] Dibujar el campo en una cuadrícula.
- [x] Mover una carga con el ratón.
- [x] Añadir una segunda carga.
- [x] Detenerse y decidir si merece la pena continuar.

## Principio de trabajo

Un solo cambio conceptual cada vez. Comprensión y capacidad de comprobación por
encima de velocidad, cantidad de funciones o acabado visual.
