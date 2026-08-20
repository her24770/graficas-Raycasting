mod font;
mod framebuffer;
mod maze;
mod player;
mod raycaster;
mod render;
mod textures;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::process_events;
use crate::textures::TextureAtlas;

const BLOCK_SIZE: usize = 100;
const FOV: f32 = PI / 3.0;
const TARGET_FPS: f32 = 15.0;

fn main() {
    let window_width = 1000;
    let window_height = 600;
    let framebuffer_width = 1000;
    let framebuffer_height = 600;
    let target_frame_time = Duration::from_secs_f32(1.0 / TARGET_FPS);

    let (maze, mut player) = load_maze("./assets/levels/level1.txt", BLOCK_SIZE);
    let textures = TextureAtlas::load();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

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

        process_events(&window, &mut player, &maze, BLOCK_SIZE as f32, dt, &mut mouse_prev_x);

        let i = player.pos.x as usize / BLOCK_SIZE;
        let j = player.pos.y as usize / BLOCK_SIZE;
        if maze.get(j).and_then(|row| row.get(i)) == Some(&'g') {
            println!("¡Meta alcanzada! Fin del juego.");
            break;
        }

        render::walls::render(&mut framebuffer, &maze, &player, FOV, BLOCK_SIZE as f32, &textures);
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
