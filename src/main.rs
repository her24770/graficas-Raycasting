mod audio;
mod font;
mod framebuffer;
mod maze;
mod player;
mod raycaster;
mod render;
mod sprite;
mod state;
mod textures;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

use crate::audio::AudioEngine;
use crate::framebuffer::Framebuffer;
use crate::player::process_events;
use crate::state::{GameState, PlayingState, LEVELS};
use crate::textures::TextureAtlas;

const BLOCK_SIZE: usize = 100;
const FOV: f32 = PI / 3.0;
const TARGET_FPS: f32 = 15.0;
/// Distancia recorrida (en unidades del mundo) entre cada sonido de paso.
const STEP_DISTANCE: f32 = 60.0;

fn main() {
    let window_width = 1000;
    let window_height = 600;
    let framebuffer_width = 1000;
    let framebuffer_height = 600;
    let target_frame_time = Duration::from_secs_f32(1.0 / TARGET_FPS);

    let textures = TextureAtlas::load();
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut audio = AudioEngine::new();

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
    let mut state = GameState::Welcome { selected: 0 };

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

        let confirm_pressed = window.is_key_pressed(Key::Enter, KeyRepeat::No)
            || window.is_key_pressed(Key::Space, KeyRepeat::No);

        // Las transiciones de estado se juntan acá en vez de reasignar
        // `state` dentro del match: así no hay conflicto de préstamos con
        // los bindings (`selected`, `ps`, `level_index`) que vienen de
        // matchear sobre `&mut state`.
        let mut next_state: Option<GameState> = None;

        match &mut state {
            GameState::Welcome { selected } => {
                if window.is_key_pressed(Key::W, KeyRepeat::No) || window.is_key_pressed(Key::Up, KeyRepeat::No) {
                    *selected = (*selected + LEVELS.len() - 1) % LEVELS.len();
                }
                if window.is_key_pressed(Key::S, KeyRepeat::No) || window.is_key_pressed(Key::Down, KeyRepeat::No) {
                    *selected = (*selected + 1) % LEVELS.len();
                }

                let chosen = *selected;
                render::screen::draw_welcome(&mut framebuffer, chosen);

                if confirm_pressed {
                    let playing = PlayingState::new(chosen, BLOCK_SIZE, framebuffer_width);
                    audio.play_music_loop("assets/audio/music/background.mp3");
                    next_state = Some(GameState::Playing(playing));
                }
            }
            GameState::Playing(ps) => {
                let pos_before = ps.player.pos;
                let blocked = process_events(&window, &mut ps.player, &ps.maze, BLOCK_SIZE as f32, dt, &mut ps.mouse_prev_x);

                // solo cuenta el golpe cuando arranca el contacto, no cada
                // cuadro que se sigue empujando contra la misma pared.
                if blocked && !ps.was_blocked {
                    ps.collisions += 1;
                }
                ps.was_blocked = blocked;

                ps.distance_since_step += (ps.player.pos - pos_before).magnitude();
                if ps.distance_since_step >= STEP_DISTANCE {
                    ps.distance_since_step = 0.0;
                    audio.play_sfx("assets/audio/sfx/step.mp3");
                }

                ps.time_left = (ps.time_left - dt).max(0.0);

                for sprite in ps.sprites.iter_mut() {
                    sprite.update(dt);
                }

                render::walls::render(
                    &mut framebuffer,
                    &ps.maze,
                    &ps.player,
                    FOV,
                    BLOCK_SIZE as f32,
                    &textures,
                    &mut ps.z_buffer,
                );
                render::sprites::render(&mut framebuffer, &ps.player, &ps.sprites, FOV, BLOCK_SIZE as f32, &ps.z_buffer);
                render::minimap::render(&mut framebuffer, &ps.maze, &ps.player, BLOCK_SIZE as f32);
                render::hud::draw_fps(&mut framebuffer, fps_smoothed);
                render::hud::draw_timer(&mut framebuffer, ps.time_left);
                render::hud::draw_collisions(&mut framebuffer, ps.collisions);

                if ps.reached_goal(BLOCK_SIZE) {
                    audio.play_sfx("assets/audio/sfx/success.mp3");
                    next_state = Some(GameState::Success {
                        level_index: ps.level_index,
                        collisions: ps.collisions,
                        time_used: ps.time_used(),
                    });
                } else if ps.time_left <= 0.0 {
                    next_state = Some(GameState::TimeUp {
                        level_index: ps.level_index,
                        collisions: ps.collisions,
                    });
                }
            }
            GameState::Success { level_index, collisions, time_used } => {
                render::screen::draw_success(&mut framebuffer, *level_index, *collisions, *time_used);

                if confirm_pressed {
                    next_state = Some(GameState::Welcome { selected: *level_index });
                }
            }
            GameState::TimeUp { level_index, collisions } => {
                render::screen::draw_time_up(&mut framebuffer, *level_index, *collisions);

                if confirm_pressed {
                    next_state = Some(GameState::Welcome { selected: *level_index });
                }
            }
        }

        if let Some(ns) = next_state {
            state = ns;
        }

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
