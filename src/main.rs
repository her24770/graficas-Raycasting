mod caster;
mod framebuffer;
mod maze;
mod player;

use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{load_maze, Maze};
use crate::player::{process_events, Player};

const BLOCK_SIZE: usize = 100;
const NUM_RAYS: usize = 5;
const FOV: f32 = PI / 3.0;

// Placeholder heredado de la base del profesor: vista 2D top-down con un
// abanico de rayos dibujado sobre el mapa. Etapa 0 (Setup) del PLAN.md deja
// esto compilando y corriendo; se reemplaza en etapas posteriores por la
// vista en primera persona real, texturas, colisiones, HUD, minimapa, etc.

fn cell_color(cell: char) -> u32 {
    match cell {
        '1' => 0x888888, // material 1 (tema por definir en la Etapa 4)
        '2' => 0xAA4433, // material 2
        '3' => 0x8B5A2B, // material 3
        '4' => 0xB0B8C0, // material 4
        'g' | 'G' => 0x00FF00,
        _ => 0xFFDDDD,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(cell_color(cell));

    for x in xo..xo + BLOCK_SIZE {
        for y in yo..yo + BLOCK_SIZE {
            framebuffer.point(x, y);
        }
    }
}

fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, row * BLOCK_SIZE, cell);
        }
    }

    framebuffer.set_current_color(0xFFFF00);

    let px = player.pos.x as usize;
    let py = player.pos.y as usize;

    for x in px.saturating_sub(3)..=px + 3 {
        for y in py.saturating_sub(3)..=py + 3 {
            framebuffer.point(x, y);
        }
    }

    for i in 0..NUM_RAYS {
        let ray_fraction = i as f32 / (NUM_RAYS - 1) as f32;
        let angle = player.a - FOV / 2.0 + FOV * ray_fraction;
        cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE);
    }
}

fn main() {
    let window_width = 1000;
    let window_height = 600;
    let framebuffer_width = 1000;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let (maze, mut player) = load_maze("./assets/levels/level1.txt", BLOCK_SIZE);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(0x333355);

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

        framebuffer.clear();

        render(&mut framebuffer, &maze, &player);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
