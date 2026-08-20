# Raycasting

Ray caster en primera persona hecho en Rust desde cero, para el curso de Gráficas por Computadora. Renderiza un laberinto propio (más grande que el de referencia del curso) con texturas, sprites animados, minimapa, audio y un flujo completo de pantallas.

Ver `TAREA.md` para la consigna original y `PLAN.md` para el detalle de las etapas de desarrollo.

## Cómo correr

Requiere [Rust](https://www.rust-lang.org/) instalado (`cargo`).

```bash
git clone <url-de-este-repo>
cd graficas-Raycasting
cargo run --release
```

`--release` es importante: en modo debug el raycaster es notablemente más lento.

## Controles

| Tecla / acción | Efecto |
| --- | --- |
| `W` | Avanzar |
| `S` | Retroceder |
| `A` / `D` | Girar (teclado) |
| Mover el mouse | Girar (horizontal) |
| `W`/`S` o flechas ↑/↓ | Elegir nivel (en el menú) |
| `Enter` / `Espacio` | Confirmar (jugar, o volver al menú) |
| `Esc` | Salir |

> Nota: `minifb` (la librería de ventana) no soporta recentrar el cursor de forma portable, así que el mouse gira la cámara por el movimiento entre cuadros, sin volver solo al centro. El cursor puede quedarse en un borde de la ventana; la rotación sigue funcionando igual.

## Estructura del proyecto

```
src/
  main.rs              # bucle principal: input -> estado -> render -> throttle a 15 FPS
  framebuffer.rs        # buffer de píxeles
  maze.rs                # carga de laberintos desde archivo de texto
  player.rs               # movimiento, colisiones, entrada de teclado/mouse
  raycaster.rs             # DDA: lanza un rayo y devuelve distancia/tipo de pared
  textures.rs               # carga de texturas de pared (con respaldo a color plano)
  sprite.rs                  # sprites generados por código (antorcha, cofre)
  font.rs                     # fuente de bitmap propia para todo el texto en pantalla
  state.rs                     # máquina de estados: Bienvenida / Jugando / Éxito / Tiempo agotado
  audio.rs                     # música de fondo y efectos de sonido (rodio)
  render/
    walls.rs                     # vista 3D: proyección de paredes + piso/techo con niebla
    sprites.rs                    # dibujo de sprites tipo billboard con z-buffer
    minimap.rs                     # minimapa en la esquina superior derecha
    hud.rs                          # FPS, cronómetro, contador de golpes
    screen.rs                       # pantallas de bienvenida/éxito/tiempo agotado
assets/
  levels/            # laberintos en texto plano (level1.txt, level2.txt, level3.txt)
  textures/          # texturas de pared (wall_1..wall_4)
  audio/
    sfx/             # efectos de sonido (se suben al repo, licencia libre)
    music/           # música de fondo (NO se sube al repo, ver más abajo)
```

## Laberintos

Los 3 niveles (`assets/levels/level*.txt`) están generados de forma que siempre son resolubles (verificado con BFS al generarlos) y son de mayor tamaño que el laberinto de referencia del curso (9×13):

- `level1.txt` — 17×25
- `level2.txt` — 13×19
- `level3.txt` — 15×21

Formato del archivo: cada carácter es una celda. `1`, `2`, `3`, `4` son los distintos materiales de pared (cada uno con su propia textura), espacio es piso libre, `p` marca dónde arranca el jugador, `g` marca la meta.

## Música de fondo

La carpeta `assets/audio/music/` no se sube al repositorio porque la música de fondo tiene derechos de autor. Para tener música al correr el juego, coloca un archivo llamado exactamente `background.mp3` en esa carpeta (ver `assets/audio/music/README.md`). Si el archivo no está, el juego avisa por consola y sigue funcionando normalmente, sin música.

## Objetivos de la rúbrica cubiertos

- Laberinto propio, de mayor tamaño que el de referencia, verificado como resoluble.
- El jugador no atraviesa paredes ni el juego crashea (manejo de errores en carga de assets, audio y accesos al mapa).
- Textura distinta por cada tipo de pared.
- FPS mostrados en pantalla, estabilizados a propósito en ~15.
- Cámara con movimiento adelante/atrás, rotación por teclado y por mouse.
- Minimapa en una esquina, independiente de la vista principal.
- Música de fondo + efectos de sonido (pasos, victoria).
- Sprites animados (antorchas parpadeantes, cofre con destello), generados por código.
- Pantalla de bienvenida con selección entre 3 niveles.
- Pantalla de éxito y pantalla de tiempo agotado (límite de 60 segundos por intento), con cronómetro y contador de choques contra paredes en vivo.

Pendiente/fuera de esta entrega: soporte de gamepad (la dependencia `gilrs` está incluida pero todavía no conectada) y el objetivo de correr en hardware distinto a una computadora (se evalúa como fase separada más adelante, en una calculadora Casio fx-9860GII).

## Créditos

- Punto de partida y estructura base (framebuffer, carga de laberinto, movimiento) adaptados del repositorio del curso: [menene/cc2018-2026-02-10](https://github.com/menene/cc2018-2026-02-10).
- Texturas de pared: recursos libres (CC0).
