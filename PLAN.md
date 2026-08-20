# Plan de trabajo — Raycaster en Rust

Plan por **etapas** (no por días/tiempo). Cada etapa es un hito funcional con dependencias reales del proyecto: no se puede texturizar antes de tener raycasting 3D, no puede haber minimapa antes de tener un mundo cargado, etc. Si en el camino cambia algo del diseño (el laberinto, una textura, el enfoque de una feature), el cambio se ubica dentro de la etapa a la que pertenece conceptualmente.

Ver `TAREA.md` para la consigna y rúbrica completas.

## Contexto

El profesor da como base el repo `https://github.com/menene/cc2018-2026-02-10`, en **Rust** (`minifb` + `nalgebra-glm`), organizado por ramas. Las ramas de Raycasting (`07-RC-01-MAZE-LOADER`, `08-RC-02-MAZE-PLAYER`, `09-RC-03-MAZE-MOVEMENT`) solo traen:

- `framebuffer.rs`: buffer de píxeles básico (`point`, `clear`, colores).
- `maze.rs`: carga un `.txt` a `Vec<Vec<char>>`, detecta `p` (jugador) y `g` (meta).
- `player.rs`: `Player{pos, a}`, movimiento WASD **sin colisión** (atraviesa paredes).
- `caster.rs`: lanza un rayo pintando píxel a píxel sobre el framebuffer — **no** devuelve distancia.
- `main.rs`: renderiza solo una **vista 2D top-down** del mapa con un abanico de rayos dibujado encima.

**No existe en ninguna rama publicada** la proyección en primera persona, texturas, colisiones, minimapa, audio, sprites ni pantallas de estado (la rama `10-RC-04-MAZE-EVENTS`, que traería texturas/sprites, aún no está publicada). La base del profesor solo cubre el andamiaje (ventana, framebuffer, input crudo, parseo de laberinto); el raycaster real y todo lo demás se construye desde cero.

## Decisiones ya tomadas

- Lenguaje: **Rust**, reutilizando y extendiendo la base del profesor.
- Laberinto propio, distinto al del profesor, con dimensiones ≥ al original (9×13).
- Pantalla de bienvenida **con selección entre múltiples niveles** (2-3 laberintos).
- Música de fondo con una canción de Taylor Swift que el usuario agrega él mismo (no se commitea al repo público por riesgo de copyright; se documenta dónde colocarla). Efectos de sonido con licencia libre sí se commitean.
- Gamepad en PC: se agrega como mejora de UX, pero en la rúbrica detallada los 20 pts de "soporte a mando" están anidados bajo la categoría de "hardware distinto a computadora tradicional" — no son puntos garantizados solo por tener gamepad en una PC normal.
- **La calculadora Casio fx-9860GII queda completamente fuera de esta entrega.** Se retoma después, como proyecto aparte en C (toolchain `fxsdk`+`gint`, confirmado viable). El proyecto en PC se hace funcional y completo primero.

## Cobertura de la rúbrica en esta entrega (solo PC)

La nota máxima son 100 puntos, pero sumando el máximo de cada categoría de la tabla detallada da 195 — no hace falta perseguir todo para llegar a 100.

| Categoría | Máximo en rúbrica | ¿Se cubre en esta entrega? |
|---|---|---|
| Hardware distinto a PC + mando (anidado) | 70 | No — fase futura con la Casio, fuera de esta entrega |
| Estética del nivel (subjetivo) | 30 | Sí, apuntamos al máximo — depende del criterio del profesor |
| 15 FPS estables mostrados | 15 | Sí |
| Cámara (WASD + rotación mouse) | 30 | Sí |
| Minimapa en esquina | 10 | Sí |
| Música de fondo (+bono Taylor Swift) | 5 (el +5 de TS solo aparece en el enunciado, no en la tabla) | Sí, con ambigüedad sobre el bono |
| Efectos de sonido | 10 | Sí |
| Animación de sprite | 20 | Sí |
| Bienvenida con selección de niveles | 15 | Sí |
| Pantalla de éxito | 10 | Sí |
| **Total objetivo (sin hardware)** | — | **145 posibles → tope real 100/100** |

Ejecutado bien, este plan puede llegar a 100/100 sin tocar la calculadora. Los puntos con incertidumbre real son los subjetivos (estética) y el bono ambiguo de Taylor Swift.

## Etapas

### Etapa 0 — Setup del proyecto
`cargo new`, migrar y adaptar `framebuffer.rs` / `maze.rs` / `player.rs` de la base del profesor, definir la estructura de carpetas (`src/render/`, `assets/levels/`, `assets/textures/`, `assets/audio/`), y dejar el `Cargo.toml` con las dependencias nuevas previstas (`image`, `rodio`, `gilrs`) aunque no se usen todas todavía.

### Etapa 1 — Mundo y laberinto
Diseño del laberinto propio (dimensiones ≥ al original, varios tipos de pared distintos), formato de carga desde archivo, y estructura de datos preparada para múltiples niveles (aunque el menú de selección llegue en una etapa posterior).

### Etapa 2 — Movimiento y colisiones
Jugador con movimiento adelante/atrás y rotación (WASD), colisión sólida contra las paredes (no atravesarlas), movimiento escalado por delta de tiempo. Es uno de los tres requisitos obligatorios de la tarea (nota 0 si falla).

### Etapa 3 — Raycasting 3D (vista en primera persona)
El núcleo obligatorio del proyecto: DDA por celda, distancia perpendicular corregida (sin efecto fisheye), proyección de cada rayo como columna vertical en pantalla, con color plano por tipo de pared. Al terminar esta etapa el juego ya es jugable en primera persona de punta a punta.

### Etapa 4 — Texturizado de paredes
Reemplazar los colores planos por texturas reales por tipo de pared, con sombreado distinto según la orientación de la pared golpeada. Cubre el requisito obligatorio de "color o textura diferente por cada pared" con más fidelidad, y aporta a la estética del nivel.

### Etapa 5 — HUD y control de FPS
Contador de FPS visible en pantalla, con el framerate limitado deliberadamente a ~15fps estables.

### Etapa 6 — Cámara avanzada
Rotación horizontal con el mouse, sumada al giro por teclado ya existente.

### Etapa 7 — Minimapa
Overlay en una esquina de la pantalla (nunca al lado del mapa principal) mostrando el laberinto y la posición/orientación del jugador.

### Etapa 8 — Sprites animados
Sistema de z-buffer por columna (requisito técnico para que los sprites se recorten bien contra las paredes), renderizado de sprites tipo billboard, y al menos un sprite con animación por frames en el mundo.

### Etapa 9 — Audio
Efectos de sonido (pasos, llegada a la meta) y música de fondo en loop, con manejo robusto ante archivos faltantes (nunca debe crashear si falta un audio).

### Etapa 10 — Pantallas y flujo del juego
Máquina de estados completa: pantalla de bienvenida con selección entre los niveles disponibles, transición a la partida, y pantalla de éxito al cumplir la condición de meta.

### Etapa 11 — Gamepad (opcional / UX)
Soporte de control físico como alternativa a teclado/mouse. Mejora la demo pero no suma puntos garantizados por la rúbrica en PC normal.

### Etapa 12 — Pulido y hardening
Pase estético sobre el diseño del laberinto y las texturas (criterio subjetivo de la rúbrica), y una ronda deliberada de intentar romper el juego (esquinas del mapa, spam de teclas, alt-tab, asset faltante, gamepad desconectado a mitad de partida) arreglando cualquier crash encontrado. Es la etapa más importante junto con la 2 y la 3: un crash anula la nota completa.

### Etapa 13 — Entrega
README final (controles, cómo correr, dónde colocar el audio con copyright), verificación en un clon limpio del repo (`cargo run --release` sin los archivos locales del usuario), grabación del video de demo, y push + entrega del link.

## Fase futura (fuera de esta entrega) — Casio fx-9860GII

Se aborda después de tener el proyecto en PC terminado, como proyecto totalmente separado (la plataforma de entrega permite intentos ilimitados, así que no hay presión de meterla en este ciclo). Notas para cuando se retome:

- Proyecto en C aparte (carpeta `casio/`), toolchain `fxsdk` + `gint` (confirmado que soporta la fx-9860GII, SH3, GCC cruzado, CMake): https://git.planet-casio.com/Lephenixnoir/fxsdk
- Sin FPU en el hardware: tablas de seno/coseno precalculadas en fixed-point, nada de `f32`/`f64` en el hot path.
- Mismo concepto de DDA que en Rust, pero en fixed-point.
- Pantalla monocroma 128×64: sin texturas reales, usar patrones de dithering distintos por tipo de pared para seguir cumpliendo "color/textura distinta por pared" en escala de grises.
- Reducir columnas de rayos (32-64) por rendimiento del SH3.
- Laberinto embebido como array const (sin I/O de archivo en runtime).
- Sin mouse/gamepad ni audio (limitaciones de la calculadora); movimiento con las teclas de flecha nativas.

## Verificación

- `cargo build` y `cargo run --release` sin warnings críticos al cierre de cada etapa.
- Test manual de colisión: caminar directo contra cada tipo de pared y confirmar que el jugador se detiene sin atravesarla (etapa 2 en adelante).
- Test de robustez: renombrar/borrar temporalmente un archivo de textura o audio y confirmar que el juego sigue corriendo (fallback a color plano / sin sonido) en vez de crashear (etapa 12).
- Clonar el repo en una carpeta nueva y correr `cargo run --release` desde cero para simular la máquina del profesor, antes de la entrega final (etapa 13).
- Verificar visualmente: FPS estable cerca de 15, minimapa en una esquina, sprite animándose, flujo completo Bienvenida → Jugando → Éxito con niveles seleccionables.

## Archivos críticos

- `src/raycaster.rs` — corazón obligatorio del proyecto (DDA + corrección de fisheye).
- `src/player.rs` — colisiones + movimiento por dt (requisito obligatorio).
- `src/render/walls.rs` — integra raycaster + texturas + z-buffer.
- `src/state.rs` — máquina de estados Welcome/Playing/Success.
- `src/maze.rs` — carga de niveles propios + accessor seguro anti-crash.
- `Cargo.toml` — dependencias nuevas (`image`, `rodio`, `gilrs`).
