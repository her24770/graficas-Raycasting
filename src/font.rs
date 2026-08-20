use crate::framebuffer::Framebuffer;

const GLYPH_WIDTH: usize = 3;
const GLYPH_HEIGHT: usize = 5;

/// Fuente de bitmap mínima (3x5 píxeles por carácter), pensada para texto
/// corto de HUD y pantallas (FPS, títulos, menús), no para texto extenso.
fn glyph_for(c: char) -> [&'static str; GLYPH_HEIGHT] {
    match c {
        '0' => ["###", "#.#", "#.#", "#.#", "###"],
        '1' => [".#.", "##.", ".#.", ".#.", "###"],
        '2' => ["###", "..#", "###", "#..", "###"],
        '3' => ["###", "..#", "###", "..#", "###"],
        '4' => ["#.#", "#.#", "###", "..#", "..#"],
        '5' => ["###", "#..", "###", "..#", "###"],
        '6' => ["###", "#..", "###", "#.#", "###"],
        '7' => ["###", "..#", "..#", "..#", "..#"],
        '8' => ["###", "#.#", "###", "#.#", "###"],
        '9' => ["###", "#.#", "###", "..#", "###"],
        'A' => [".#.", "#.#", "###", "#.#", "#.#"],
        'B' => ["##.", "#.#", "##.", "#.#", "##."],
        'C' => [".##", "#..", "#..", "#..", ".##"],
        'D' => ["##.", "#.#", "#.#", "#.#", "##."],
        'E' => ["###", "#..", "##.", "#..", "###"],
        'F' => ["###", "#..", "###", "#..", "#.."],
        'G' => [".##", "#..", "#.#", "#.#", ".##"],
        'H' => ["#.#", "#.#", "###", "#.#", "#.#"],
        'I' => ["###", ".#.", ".#.", ".#.", "###"],
        'J' => ["..#", "..#", "..#", "#.#", ".#."],
        'K' => ["#.#", "#.#", "##.", "#.#", "#.#"],
        'L' => ["#..", "#..", "#..", "#..", "###"],
        'M' => ["#.#", "###", "#.#", "#.#", "#.#"],
        'N' => ["#.#", "##.", "###", ".##", "#.#"],
        'O' => [".#.", "#.#", "#.#", "#.#", ".#."],
        'P' => ["##.", "#.#", "##.", "#..", "#.."],
        'Q' => [".#.", "#.#", "#.#", ".#.", "..#"],
        'R' => ["##.", "#.#", "##.", "#.#", "#.#"],
        'S' => [".##", "#..", ".#.", "..#", "##."],
        'T' => ["###", ".#.", ".#.", ".#.", ".#."],
        'U' => ["#.#", "#.#", "#.#", "#.#", ".#."],
        'V' => ["#.#", "#.#", "#.#", "#.#", ".#."],
        'W' => ["#.#", "#.#", "#.#", "###", "#.#"],
        'X' => ["#.#", "#.#", ".#.", "#.#", "#.#"],
        'Y' => ["#.#", "#.#", ".#.", ".#.", ".#."],
        'Z' => ["###", "..#", ".#.", "#..", "###"],
        ':' => ["...", ".#.", "...", ".#.", "..."],
        '/' => ["..#", "..#", ".#.", "#..", "#.."],
        '>' => ["#..", ".#.", "..#", ".#.", "#.."],
        '<' => ["..#", ".#.", "#..", ".#.", "..#"],
        '-' => ["...", "...", "###", "...", "..."],
        '!' => [".#.", ".#.", ".#.", "...", ".#."],
        _ => ["...", "...", "...", "...", "..."],
    }
}

/// Dibuja `text` en (x, y) usando la fuente de bitmap, escalada `scale`
/// veces. No hace wrap de línea ni maneja texto largo.
pub fn draw_text(framebuffer: &mut Framebuffer, x: usize, y: usize, text: &str, color: u32, scale: usize) {
    framebuffer.set_current_color(color);

    let mut cursor_x = x;
    for c in text.chars() {
        let glyph = glyph_for(c.to_ascii_uppercase());

        for (row, line) in glyph.iter().enumerate() {
            for (col, pixel) in line.chars().enumerate() {
                if pixel != '#' {
                    continue;
                }

                for sy in 0..scale {
                    for sx in 0..scale {
                        framebuffer.point(cursor_x + col * scale + sx, y + row * scale + sy);
                    }
                }
            }
        }

        cursor_x += (GLYPH_WIDTH + 1) * scale;
    }
}

/// Ancho total en píxeles que ocupará `text` dibujado con `draw_text`.
pub fn text_width(text: &str, scale: usize) -> usize {
    text.chars().count() * (GLYPH_WIDTH + 1) * scale
}

/// Como `draw_text`, pero centrado horizontalmente alrededor de `center_x`.
pub fn draw_text_centered(framebuffer: &mut Framebuffer, center_x: usize, y: usize, text: &str, color: u32, scale: usize) {
    let width = text_width(text, scale);
    let x = center_x.saturating_sub(width / 2);
    draw_text(framebuffer, x, y, text, color, scale);
}
