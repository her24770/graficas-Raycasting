use crate::font::draw_text;
use crate::framebuffer::Framebuffer;

/// Dibuja el contador de FPS en la esquina superior izquierda. `fps` ya
/// debe venir suavizado (promedio móvil) para que el número no tiemble
/// frame a frame por pequeñas variaciones de tiempo.
pub fn draw_fps(framebuffer: &mut Framebuffer, fps: f32) {
    let text = format!("FPS:{}", fps.round() as i32);
    draw_text(framebuffer, 10, 10, &text, 0xFFFF00, 3);
}
