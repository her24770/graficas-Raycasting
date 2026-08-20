use minifb::{Key, MouseMode, Window};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;

use crate::maze::Maze;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
}

/// Velocidad de movimiento en unidades del mundo por segundo (no por frame).
const MOVE_SPEED: f32 = 300.0;
/// Velocidad de rotación en radianes por segundo (teclado).
const ROTATION_SPEED: f32 = PI;
/// Radianes de giro por cada píxel de movimiento horizontal del mouse.
const MOUSE_SENSITIVITY: f32 = 0.003;
/// Radio de colisión del jugador, como fracción del tamaño de celda.
const PLAYER_RADIUS_RATIO: f32 = 0.2;

/// True si la celda que contiene (x, y) es sólida. Cualquier coordenada
/// fuera del laberinto también cuenta como sólida, para que nunca haya
/// panics por indexar fuera de rango.
fn is_wall(maze: &Maze, x: f32, y: f32, block_size: f32) -> bool {
    if x < 0.0 || y < 0.0 {
        return true;
    }

    let col = (x / block_size) as usize;
    let row = (y / block_size) as usize;

    match maze.get(row).and_then(|r| r.get(col)) {
        // La celda de meta es piso transitable (el jugador debe poder
        // pararse ahí para disparar la condición de victoria), no una pared.
        Some(' ') | Some('g') | Some('G') => false,
        Some(_) => true,
        None => true,
    }
}

/// True si un jugador con ese radio, centrado en `pos`, se solapa con una pared.
fn collides(maze: &Maze, pos: Vec2, radius: f32, block_size: f32) -> bool {
    let checks = [
        (pos.x - radius, pos.y),
        (pos.x + radius, pos.y),
        (pos.x, pos.y - radius),
        (pos.x, pos.y + radius),
    ];

    checks.iter().any(|&(x, y)| is_wall(maze, x, y, block_size))
}

/// Lee teclado y mouse, y actualiza al jugador. El movimiento se resuelve
/// por eje (slide): si el desplazamiento en x choca se descarta, pero el de
/// y se intenta igual, para poder deslizarse contra la pared en vez de
/// quedar pegado en las esquinas.
///
/// `mouse_prev_x` guarda la posición horizontal del mouse del frame
/// anterior para calcular el delta de rotación; se pasa desde afuera para
/// que sobreviva entre llamadas. En `None` (primer frame) no se aplica
/// rotación todavía, solo se registra la posición inicial.
pub fn process_events(
    window: &Window,
    player: &mut Player,
    maze: &Maze,
    block_size: f32,
    dt: f32,
    mouse_prev_x: &mut Option<f32>,
) {
    let mut turn = 0.0;
    let mut forward = 0.0;

    if window.is_key_down(Key::A) {
        turn -= ROTATION_SPEED;
    }

    if window.is_key_down(Key::D) {
        turn += ROTATION_SPEED;
    }

    if window.is_key_down(Key::W) {
        forward += 1.0;
    }

    if window.is_key_down(Key::S) {
        forward -= 1.0;
    }

    player.a += turn * dt;

    // Rotación con el mouse (solo horizontal), por delta de posición entre
    // frames. No se recentra el cursor (ver PLAN.md, Etapa 6): minifb no
    // expone una forma portable de hacerlo, así que el cursor puede llegar
    // al borde de la ventana, pero la rotación sigue funcionando mientras
    // el mouse se siga moviendo.
    if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Pass) {
        if let Some(prev_x) = *mouse_prev_x {
            player.a += (mouse_x - prev_x) * MOUSE_SENSITIVITY;
        }
        *mouse_prev_x = Some(mouse_x);
    }

    if forward != 0.0 {
        let radius = block_size * PLAYER_RADIUS_RATIO;
        let step = MOVE_SPEED * forward * dt;
        let dir = Vec2::new(player.a.cos(), player.a.sin());

        let try_x = Vec2::new(player.pos.x + dir.x * step, player.pos.y);
        if !collides(maze, try_x, radius, block_size) {
            player.pos.x = try_x.x;
        }

        let try_y = Vec2::new(player.pos.x, player.pos.y + dir.y * step);
        if !collides(maze, try_y, radius, block_size) {
            player.pos.y = try_y.y;
        }
    }
}
