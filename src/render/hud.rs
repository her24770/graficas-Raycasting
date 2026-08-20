use crate::font::draw_text;
use crate::framebuffer::Framebuffer;

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
