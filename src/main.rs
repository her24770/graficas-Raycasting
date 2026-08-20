mod audio;
mod font;
mod framebuffer;
mod maze;
mod player;
mod raycaster;
mod render;
mod sprite;
mod textures;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::audio::AudioEngine;
use crate::framebuffer::Framebuffer;
use crate::maze::{load_maze, Maze};
use crate::player::process_events;
use crate::sprite::Sprite;
use crate::textures::TextureAtlas;

const BLOCK_SIZE: usize = 100;
const FOV: f32 = PI / 3.0;
const TARGET_FPS: f32 = 15.0;
const TORCH_COUNT: usize = 4;
/// Distancia recorrida (en unidades del mundo) entre cada sonido de paso.
const STEP_DISTANCE: f32 = 60.0;

fn cell_to_pos(row: usize, col: usize, block_size: f32) -> Vec2 {
    Vec2::new(
        col as f32 * block_size + block_size / 2.0,
        row as f32 * block_size + block_size / 2.0,
    )
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> i32 {
    (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
}

/// Elige `count` celdas abiertas del laberinto, repartidas por el mapa en
/// vez de amontonadas: arranca en `start` (la celda del jugador) y en cada
/// paso agrega la celda abierta que quede más lejos de todas las ya
/// elegidas (farthest point sampling), así los sprites quedan como algo
/// para ir descubriendo en distintos puntos del recorrido.
fn scatter_open_cells(maze: &Maze, start: (usize, usize), count: usize, block_size: f32) -> Vec<Vec2> {
    let open_cells: Vec<(usize, usize)> = maze
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == ' ')
                .map(move |(col, _)| (row, col))
        })
        .filter(|&cell| cell != start)
        .collect();

    let mut chosen: Vec<(usize, usize)> = Vec::new();
    let mut reference = vec![start];

    for _ in 0..count.min(open_cells.len()) {
        let next = open_cells
            .iter()
            .filter(|cell| !chosen.contains(cell))
            .max_by_key(|&&cell| reference.iter().map(|&r| manhattan(cell, r)).min().unwrap_or(0));

        match next {
            Some(&cell) => {
                chosen.push(cell);
                reference.push(cell);
            }
            None => break,
        }
    }

    chosen
        .into_iter()
        .map(|(row, col)| cell_to_pos(row, col, block_size))
        .collect()
}

fn main() {
    let window_width = 1000;
    let window_height = 600;
    let framebuffer_width = 1000;
    let framebuffer_height = 600;
    let target_frame_time = Duration::from_secs_f32(1.0 / TARGET_FPS);

    let (maze, mut player) = load_maze("./assets/levels/level1.txt", BLOCK_SIZE);
    let textures = TextureAtlas::load();

    let player_start_cell = (
        (player.pos.y / BLOCK_SIZE as f32) as usize,
        (player.pos.x / BLOCK_SIZE as f32) as usize,
    );
    let torch_positions = scatter_open_cells(&maze, player_start_cell, TORCH_COUNT, BLOCK_SIZE as f32);
    let mut sprites: Vec<Sprite> = torch_positions.into_iter().map(Sprite::torch).collect();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut z_buffer = vec![f32::MAX; framebuffer_width];

    let mut audio = AudioEngine::new();
    audio.play_music_loop("assets/audio/music/background.mp3");
    let mut distance_since_step = 0.0;

    let mut window = Window::new(
        "Raycasting",
        window_width,
        window_height,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let mut last_frame = Instant::now();
    let mut fps_smoothed = TARGET_FPS;
    let mut mouse_prev_x: Option<f32> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = Instant::now();
        // clamp: un frame anormalmente lento (alt-tab, disco, etc.) nunca
        // debe producir un salto de posición lo bastante grande como para
        // atravesar una pared entre dos frames.
        let dt = (frame_start - last_frame).as_secs_f32().min(0.1);
        last_frame = frame_start;

        // promedio móvil (exponencial) para que el número en pantalla no
        // tiemble frame a frame por pequeñas variaciones de tiempo.
        if dt > 0.0 {
            fps_smoothed = fps_smoothed * 0.9 + (1.0 / dt) * 0.1;
        }

        let pos_before = player.pos;
        process_events(&window, &mut player, &maze, BLOCK_SIZE as f32, dt, &mut mouse_prev_x);

        distance_since_step += (player.pos - pos_before).magnitude();
        if distance_since_step >= STEP_DISTANCE {
            distance_since_step = 0.0;
            audio.play_sfx("assets/audio/sfx/step.wav");
        }

        let i = player.pos.x as usize / BLOCK_SIZE;
        let j = player.pos.y as usize / BLOCK_SIZE;
        if maze.get(j).and_then(|row| row.get(i)) == Some(&'g') {
            println!("¡Meta alcanzada! Fin del juego.");
            audio.play_sfx("assets/audio/sfx/success.wav");
            // el sonido se reproduce en un hilo aparte; sin esta pausa el
            // proceso terminaría antes de que llegue a sonar (se reemplaza
            // por la pantalla de éxito real en la Etapa 10).
            std::thread::sleep(Duration::from_millis(1500));
            break;
        }

        for sprite in sprites.iter_mut() {
            sprite.update(dt);
        }

        render::walls::render(
            &mut framebuffer,
            &maze,
            &player,
            FOV,
            BLOCK_SIZE as f32,
            &textures,
            &mut z_buffer,
        );
        render::sprites::render(&mut framebuffer, &player, &sprites, FOV, BLOCK_SIZE as f32, &z_buffer);
        render::minimap::render(&mut framebuffer, &maze, &player, BLOCK_SIZE as f32);
        render::hud::draw_fps(&mut framebuffer, fps_smoothed);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        // throttle deliberado: sin esto el motor correría mucho más rápido
        // que 15fps; se descuenta el tiempo ya gastado en este frame para
        // apuntar al framerate objetivo real, no a un framerate objetivo
        // más el tiempo de trabajo encima.
        let elapsed = frame_start.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
    }
}
