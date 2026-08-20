# Proyecto 1: Raycasting - Código

## Información general

| Campo | Detalle |
|---|---|
| **Fecha de entrega** | Viernes 21 de agosto de 2026, 23:59 |
| **Puntos posibles** | 14 |
| **Intentos permitidos** | Ilimitados |
| **Disponible hasta** | 21 de agosto de 2026, 23:59 |
| **Estado** | Intento 1 - En progreso |

## Objetivo

Demostrar los conocimientos adquiridos durante la tercera parte del curso.

## Requisitos de entrega

- Entregar un **Ray Caster simple** (usando **Rust** o **C++**) que renderice un nivel entero y jugable.
- La entrega debe ser un **link a GitHub**.
- Debe incluirse un **pequeño video** del software funcionando.

## Requisitos obligatorios (para obtener nota distinta de 0)

- El laberinto debe ser **diferente** al proporcionado por el profesor, pero con **iguales o mayores dimensiones**.
- Al controlar al jugador:
  - No se debe poder **atravesar las paredes**.
  - El juego **no debe crashear**.
- Se debe colocar un **color diferente (o una textura)** para cada una de las paredes distintas en el mapa.

## Código de referencia del profesor

Repositorio del curso (se puede copiar/reutilizar sin problema):

```
https://github.com/menene/cc2018-2026-02-10
```

> Las clases relevantes de Ray Casting (RC) son las que inician con **07**, **08** y **09**.

## Sistema de puntuación (máximo 100 puntos)

La nota máxima es de 100 puntos. Se pueden escoger libremente los objetivos a cumplir (no hay puntos extra por encima de 100).

| Objetivo | Puntos |
|---|---|
| Implementar el proyecto en hardware distinto a una computadora tradicional *(criterio subjetivo)* | 0 a 50 |
| — Si además agregan soporte para control (mando) | 20 |
| Estética del nivel *(criterio subjetivo)* | 0 a 30 |
| Mantener alrededor de 15 FPS (los FPS deben desplegarse en pantalla) | 15 |
| Implementar cámara con movimiento hacia adelante/atrás y rotación | 20 |
| — Rotación adicional con el mouse (solo horizontal) | +10 |
| Implementar un minimapa (posición del jugador en el mundo; debe estar en una esquina, **no** al lado del mapa principal) | 10 |
| Agregar música de fondo | 5 |
| — Adicional si la música de fondo es de **Taylor Swift** (10 en total) | +5 |
| Agregar efectos de sonido | 10 |
| Agregar al menos 1 animación a algún sprite en pantalla | 20 |
| Agregar pantalla de bienvenida | 5 |
| — Si permite seleccionar entre múltiples niveles | +10 |
| Agregar pantalla de éxito al cumplirse una condición en el nivel | 10 |

**Texturas:** se pueden usar texturas libres de internet para texturizar el mundo 3D.

## Rúbrica detallada

### 1. Hardware distinto (criterio subjetivo) — 50 pts

| Calificación | Puntos |
|---|---|
| Multiple Hardware | 50 |
| Soporte a mando | 20 |
| Sin marcas | 0 |

### 2. Estética del nivel (criterio subjetivo) — 30 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 30 |
| Sin marcas | 0 |

### 3. 15 FPS estables — 15 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 15 |
| Sin marcas | 0 |

### 4. Cámara con movimiento — 30 pts

| Calificación | Puntos |
|---|---|
| Movimiento con mouse | 30 |
| Movimiento WASD | 20 |
| Sin marcas | 0 |

### 5. Minimapa — 10 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 10 |
| Sin marcas | 0 |

### 6. Música de fondo — 5 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 5 |
| Sin marcas | 0 |

### 7. Efectos de sonido — 10 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 10 |
| Sin marcas | 0 |

### 8. Animación de sprite — 20 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 20 |
| Sin marcas | 0 |

### 9. Pantalla de bienvenida — 15 pts

| Calificación | Puntos |
|---|---|
| Con selección de niveles | 15 |
| Solo pantalla | 5 |
| Sin marcas | 0 |

### 10. Pantalla de éxito — 10 pts

| Calificación | Puntos |
|---|---|
| Con marcas | 10 |
| Sin marcas | 0 |
