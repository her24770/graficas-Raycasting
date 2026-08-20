use crate::font;
use crate::framebuffer::Framebuffer;
use crate::render::hud::format_time;
use crate::state::LEVELS;

const LABEL_COLOR: u32 = 0xD8B4A0;
const TITLE_COLOR: u32 = 0x5C3A2E;
const FAIL_COLOR: u32 = 0xB03A2E;
const ITEM_COLOR: u32 = 0xE8DCC8;
const HIGHLIGHT_COLOR: u32 = 0xF5EDE0;
const FOOTER_COLOR: u32 = 0xC9A98E;

/// Fondo compartido por las pantallas de estado (bienvenida, éxito):
/// degradado vertical de azul profundo a marrón cálido, un acento circular
/// tipo sol/luna, y capas de siluetas oscuras superpuestas cerca del fondo
/// para dar sensación de profundidad.
pub fn draw_background(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    // Paradas de color de arriba hacia abajo: azul profundo, malva
    // polvoriento, terracota cálido, casi negro.
    let stops: [(f32, (u8, u8, u8)); 4] = [
        (0.0, (0x1B, 0x3A, 0x5C)),
        (0.4, (0x7D, 0x64, 0x78)),
        (0.75, (0xB8, 0x7A, 0x63)),
        (1.0, (0x1A, 0x10, 0x0A)),
    ];

    for y in 0..height {
        let t = y as f32 / (height.max(1) - 1).max(1) as f32;
        let color = gradient_at(&stops, t);
        framebuffer.set_current_color(color);
        for x in 0..width {
            framebuffer.point(x, y);
        }
    }

    // Sol/luna: círculo cálido a media altura, ligeramente arriba del centro.
    let sun_cx = width as f32 / 2.0;
    let sun_cy = height as f32 * 0.5;
    let sun_r = height as f32 * 0.09;
    draw_filled_circle(framebuffer, sun_cx, sun_cy, sun_r, 0xF5EDE0);

    // Capas de siluetas (tipo cordillera / muros del laberinto), de más
    // clara y lejana a más oscura y cercana, cada una más abajo que la anterior.
    let layers: [(f32, u32); 3] = [(0.62, 0x4A3226), (0.74, 0x33201A), (0.86, 0x1C0F0A)];
    for &(base_y, color) in &layers {
        draw_ridge_silhouette(framebuffer, base_y, color);
    }
}

fn gradient_at(stops: &[(f32, (u8, u8, u8))], t: f32) -> u32 {
    for pair in stops.windows(2) {
        let (t0, c0) = pair[0];
        let (t1, c1) = pair[1];
        if t >= t0 && t <= t1 {
            let local_t = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            let r = lerp(c0.0 as f32, c1.0 as f32, local_t) as u32;
            let g = lerp(c0.1 as f32, c1.1 as f32, local_t) as u32;
            let b = lerp(c0.2 as f32, c1.2 as f32, local_t) as u32;
            return (r << 16) | (g << 8) | b;
        }
    }
    let (_, last) = stops[stops.len() - 1];
    ((last.0 as u32) << 16) | ((last.1 as u32) << 8) | last.2 as u32
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn draw_filled_circle(framebuffer: &mut Framebuffer, cx: f32, cy: f32, radius: f32, color: u32) {
    framebuffer.set_current_color(color);
    let r2 = radius * radius;
    let min_x = (cx - radius).max(0.0) as usize;
    let max_x = (cx + radius).min(framebuffer.width as f32 - 1.0) as usize;
    let min_y = (cy - radius).max(0.0) as usize;
    let max_y = (cy + radius).min(framebuffer.height as f32 - 1.0) as usize;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            if dx * dx + dy * dy <= r2 {
                framebuffer.point(x, y);
            }
        }
    }
}

fn draw_divider(framebuffer: &mut Framebuffer, center_x: usize, y: usize, half_width: usize, color: u32) {
    framebuffer.set_current_color(color);
    let start = center_x.saturating_sub(half_width);
    let end = (center_x + half_width).min(framebuffer.width - 1);
    for x in start..=end {
        framebuffer.point(x, y);
    }
}

/// Pantalla de bienvenida: título, lista de niveles con el seleccionado
/// resaltado, e instrucciones de control.
pub fn draw_welcome(framebuffer: &mut Framebuffer, selected: usize) {
    draw_background(framebuffer);

    let width = framebuffer.width;
    let height = framebuffer.height;
    let cx = width / 2;

    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.08) as usize, "RAYCASTING", LABEL_COLOR, 2);
    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.14) as usize, "SELECCIONA NIVEL", TITLE_COLOR, 4);
    draw_divider(framebuffer, cx, (height as f32 * 0.23) as usize, width / 8, TITLE_COLOR);

    let list_start_y = (height as f32 * 0.32) as usize;
    let spacing = (height as f32 * 0.07) as usize;

    for (i, level) in LEVELS.iter().enumerate() {
        let y = list_start_y + i * spacing;
        if i == selected {
            let text = format!("> {} <", level.name);
            font::draw_text_centered(framebuffer, cx, y, &text, HIGHLIGHT_COLOR, 3);
        } else {
            font::draw_text_centered(framebuffer, cx, y, level.name, ITEM_COLOR, 3);
        }
    }

    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.90) as usize, "W/S: ELEGIR", FOOTER_COLOR, 2);
    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.94) as usize, "ENTER: JUGAR", FOOTER_COLOR, 2);
}

/// Fondo + título + líneas de información, compartido entre la pantalla de
/// éxito y la de tiempo agotado (mismo layout, distinto color de acento).
fn draw_end_screen(framebuffer: &mut Framebuffer, label: &str, title: &str, info_lines: &[String], accent: u32) {
    draw_background(framebuffer);

    let width = framebuffer.width;
    let height = framebuffer.height;
    let cx = width / 2;

    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.24) as usize, label, LABEL_COLOR, 2);
    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.30) as usize, title, accent, 4);
    draw_divider(framebuffer, cx, (height as f32 * 0.39) as usize, width / 8, accent);

    let mut y = (height as f32 * 0.48) as usize;
    for line in info_lines {
        font::draw_text_centered(framebuffer, cx, y, line, ITEM_COLOR, 3);
        y += (height as f32 * 0.07) as usize;
    }

    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.90) as usize, "ENTER: MENU", FOOTER_COLOR, 2);
    font::draw_text_centered(framebuffer, cx, (height as f32 * 0.94) as usize, "ESC: SALIR", FOOTER_COLOR, 2);
}

/// Pantalla de éxito al completar un nivel.
pub fn draw_success(framebuffer: &mut Framebuffer, level_index: usize, collisions: u32, time_used: f32) {
    let level_name = LEVELS.get(level_index).map(|l| l.name).unwrap_or("NIVEL");
    let lines = [
        format!("TIEMPO {}", format_time(time_used)),
        format!("GOLPES {collisions}"),
    ];
    draw_end_screen(framebuffer, level_name, "COMPLETADO", &lines, TITLE_COLOR);
}

/// Pantalla de fin de nivel cuando se agota el tiempo sin llegar a la meta.
pub fn draw_time_up(framebuffer: &mut Framebuffer, level_index: usize, collisions: u32) {
    let level_name = LEVELS.get(level_index).map(|l| l.name).unwrap_or("NIVEL");
    let lines = [format!("GOLPES {collisions}")];
    draw_end_screen(framebuffer, level_name, "TIEMPO AGOTADO", &lines, FAIL_COLOR);
}

/// Silueta jagged (tipo cresta de montaña / muros irregulares) rellena
/// desde `base_y` (fracción de la altura) hasta el fondo de la pantalla.
fn draw_ridge_silhouette(framebuffer: &mut Framebuffer, base_y: f32, color: u32) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let base = (height as f32 * base_y) as usize;

    // Perfil pseudoaleatorio pero determinístico (sin dependencias extra),
    // basado en una suma de senos con distinta frecuencia por capa.
    let seed = (base_y * 997.0) as u32;

    framebuffer.set_current_color(color);
    for x in 0..width {
        let fx = x as f32 / width as f32;
        let wobble = ((fx * 11.0 + seed as f32).sin() * 0.5
            + (fx * 23.0 + seed as f32 * 1.7).sin() * 0.3)
            * (height as f32 * 0.05);
        let top = (base as f32 + wobble).max(0.0) as usize;

        for y in top..height {
            framebuffer.point(x, y);
        }
    }
}
