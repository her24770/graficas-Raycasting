use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::render::walls::fallback_color;

const BOX_SIZE: usize = 160;
const MARGIN: usize = 12;
const BACKGROUND_COLOR: u32 = 0x101018;
const BORDER_COLOR: u32 = 0xCCCCCC;
const PLAYER_COLOR: u32 = 0xFFFF00;

/// Dibuja el minimapa como overlay en la esquina superior derecha, encima
/// de lo que ya se haya renderizado (vista 3D + HUD). Por eso se llama al
/// final del frame, nunca antes.
pub fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, block_size: f32) {
    let rows = maze.len();
    let cols = maze.iter().map(|row| row.len()).max().unwrap_or(0);
    if rows == 0 || cols == 0 {
        return;
    }

    let box_x = framebuffer.width.saturating_sub(BOX_SIZE + MARGIN);
    let box_y = MARGIN;

    framebuffer.set_current_color(BACKGROUND_COLOR);
    for y in box_y..box_y + BOX_SIZE {
        for x in box_x..box_x + BOX_SIZE {
            framebuffer.point(x, y);
        }
    }

    let cell_px = (BOX_SIZE as f32 / rows.max(cols) as f32).max(1.0);

    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == ' ' {
                continue;
            }

            framebuffer.set_current_color(fallback_color(cell));

            let cx = box_x as f32 + col as f32 * cell_px;
            let cy = box_y as f32 + row as f32 * cell_px;
            let x0 = cx as usize;
            let y0 = cy as usize;
            let x1 = ((cx + cell_px) as usize).max(x0 + 1);
            let y1 = ((cy + cell_px) as usize).max(y0 + 1);

            for y in y0..y1 {
                for x in x0..x1 {
                    framebuffer.point(x, y);
                }
            }
        }
    }

    framebuffer.set_current_color(BORDER_COLOR);
    for x in box_x..box_x + BOX_SIZE {
        framebuffer.point(x, box_y);
        framebuffer.point(x, box_y + BOX_SIZE - 1);
    }
    for y in box_y..box_y + BOX_SIZE {
        framebuffer.point(box_x, y);
        framebuffer.point(box_x + BOX_SIZE - 1, y);
    }

    let player_col = player.pos.x / block_size;
    let player_row = player.pos.y / block_size;
    let px = box_x as f32 + player_col * cell_px;
    let py = box_y as f32 + player_row * cell_px;

    // línea corta indicando hacia dónde mira el jugador
    framebuffer.set_current_color(PLAYER_COLOR);
    let dir_len = cell_px * 1.5;
    let steps = 10;
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = (px + player.a.cos() * dir_len * t) as usize;
        let y = (py + player.a.sin() * dir_len * t) as usize;
        framebuffer.point(x, y);
    }

    // punto del jugador
    let radius: i32 = 2;
    let pxu = px as i32;
    let pyu = py as i32;
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let x = pxu + dx;
            let y = pyu + dy;
            if x >= 0 && y >= 0 {
                framebuffer.point(x as usize, y as usize);
            }
        }
    }
}
