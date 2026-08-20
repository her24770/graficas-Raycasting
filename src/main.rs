mod framebuffer;
mod maze;
mod player;
mod raycaster;
mod render;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::process_events;

const BLOCK_SIZE: usize = 100;
const FOV: f32 = PI / 3.0;

fn main() {
    let window_width = 1000;
    let window_height = 600;
    let framebuffer_width = 1000;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let (maze, mut player) = load_maze("./assets/levels/level1.txt", BLOCK_SIZE);

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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        // clamp: un frame anormalmente lento (alt-tab, disco, etc.) nunca
        // debe producir un salto de posición lo bastante grande como para
        // atravesar una pared entre dos frames.
        let dt = (now - last_frame).as_secs_f32().min(0.1);
        last_frame = now;

        process_events(&window, &mut player, &maze, BLOCK_SIZE as f32, dt);

        let i = player.pos.x as usize / BLOCK_SIZE;
        let j = player.pos.y as usize / BLOCK_SIZE;
        if maze.get(j).and_then(|row| row.get(i)) == Some(&'g') {
            println!("¡Meta alcanzada! Fin del juego.");
            break;
        }

        render::walls::render(&mut framebuffer, &maze, &player, FOV, BLOCK_SIZE as f32);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
