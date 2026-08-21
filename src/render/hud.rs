use std::f32::consts::PI;

use crate::font::draw_text;
use crate::framebuffer::Framebuffer;

const FLASH_BLINKS: f32 = 3.0;
const FLASH_MAX_STRENGTH: f32 = 0.55;

/// Dibuja el contador de FPS en la esquina superior izquierda. `fps` ya
/// debe venir suavizado (promedio móvil) para que el número no tiemble
/// frame a frame por pequeñas variaciones de tiempo.
pub fn draw_fps(framebuffer: &mut Framebuffer, fps: f32) {
    let text = format!("FPS:{}", fps.round() as i32);
    draw_text(framebuffer, 10, 10, &text, 0xFFFF00, 3);
}

/// Formatea segundos como MM:SS, usado tanto por el cronómetro en vivo
/// como por las pantallas de fin de nivel.
pub fn format_time(seconds: f32) -> String {
    let total = seconds.max(0.0) as u32;
    format!("{:02}:{:02}", total / 60, total % 60)
}

/// Cronómetro de cuenta regresiva, debajo de los FPS. Se pone rojo en los
/// últimos 10 segundos como aviso visual de que se acaba el tiempo.
pub fn draw_timer(framebuffer: &mut Framebuffer, seconds_left: f32) {
    let text = format!("TIEMPO {}", format_time(seconds_left));
    let color = if seconds_left <= 10.0 { 0xFF4444 } else { 0xFFFF00 };
    draw_text(framebuffer, 10, 30, &text, color, 2);
}

/// Contador de choques contra paredes, debajo del cronómetro.
pub fn draw_collisions(framebuffer: &mut Framebuffer, collisions: u32) {
    let text = format!("GOLPES {collisions}");
    draw_text(framebuffer, 10, 46, &text, 0xFFAA55, 2);
}

/// Parpadeo rojo de pantalla completa al chocar contra una pared. `remaining`
/// cuenta hacia atrás desde `duration`; en 0.0 no dibuja nada. Se apaga solo
/// a medida que pasa el tiempo, sin que nadie tenga que resetearlo a mano.
pub fn draw_collision_flash(framebuffer: &mut Framebuffer, remaining: f32, duration: f32) {
    if remaining <= 0.0 {
        return;
    }

    let progress = 1.0 - (remaining / duration).clamp(0.0, 1.0);
    let envelope = 1.0 - progress; // se va apagando a medida que pasa el tiempo
    let blink = (progress * FLASH_BLINKS * 2.0 * PI).sin().abs(); // varios destellos, no uno solo
    let strength = envelope * blink * FLASH_MAX_STRENGTH;

    for pixel in framebuffer.buffer.iter_mut() {
        let r = (*pixel >> 16) & 0xFF;
        let g = (*pixel >> 8) & 0xFF;
        let b = *pixel & 0xFF;

        let nr = (r as f32 + (0xFF - r) as f32 * strength) as u32;
        let ng = (g as f32 * (1.0 - strength)) as u32;
        let nb = (b as f32 * (1.0 - strength)) as u32;

        *pixel = (nr << 16) | (ng << 8) | nb;
    }
}
