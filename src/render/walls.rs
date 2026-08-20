use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::raycaster::cast_ray;

const CEILING_COLOR: u32 = 0x404060;
const FLOOR_COLOR: u32 = 0x303030;

/// Color por tipo de pared. Placeholder: la textura real por material
/// llega en la Etapa 4 (Texturizado de paredes) del PLAN.md.
fn wall_color(wall_type: char, side: u8) -> u32 {
    let base = match wall_type {
        '1' => 0x888888,
        '2' => 0xAA4433,
        '3' => 0x8B5A2B,
        '4' => 0xB0B8C0,
        'g' | 'G' => 0x00FF00,
        _ => 0xFFDDDD,
    };

    // Sombrea distinto según la orientación de la cara golpeada, para dar
    // sensación de volumen aunque todavía no haya texturas reales.
    if side == 1 {
        shade(base, 0.7)
    } else {
        base
    }
}

fn shade(color: u32, factor: f32) -> u32 {
    let r = (((color >> 16) & 0xFF) as f32 * factor) as u32;
    let g = (((color >> 8) & 0xFF) as f32 * factor) as u32;
    let b = ((color & 0xFF) as f32 * factor) as u32;
    (r << 16) | (g << 8) | b
}

/// Renderiza la vista en primera persona: un rayo por columna de píxeles,
/// proyectado a una franja vertical cuya altura depende de la distancia
/// perpendicular corregida (para evitar el efecto fisheye).
pub fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, fov: f32, block_size: f32) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    framebuffer.set_current_color(CEILING_COLOR);
    for y in 0..height / 2 {
        for x in 0..width {
            framebuffer.point(x, y);
        }
    }

    framebuffer.set_current_color(FLOOR_COLOR);
    for y in height / 2..height {
        for x in 0..width {
            framebuffer.point(x, y);
        }
    }

    // Se usa la altura (no el ancho) porque esta distancia escala la altura
    // proyectada de las paredes en pantalla, no su extensión horizontal.
    let dist_to_projection_plane = (height as f32 / 2.0) / (fov / 2.0).tan();

    for x in 0..width {
        let camera_fraction = x as f32 / width as f32;
        let ray_angle = player.a - fov / 2.0 + fov * camera_fraction;

        let hit = match cast_ray(maze, player.pos, ray_angle, block_size) {
            Some(hit) => hit,
            None => continue,
        };

        // Corrección de fisheye: se proyecta la distancia cruda sobre la
        // dirección de vista del jugador, no se usa la distancia radial tal cual.
        let corrected = (hit.distance * (ray_angle - player.a).cos()).max(1.0);

        let wall_height = (block_size / corrected) * dist_to_projection_plane;

        let half = height as f32 / 2.0;
        let draw_start = (half - wall_height / 2.0).max(0.0) as usize;
        let draw_end = (half + wall_height / 2.0).min(height as f32 - 1.0) as usize;

        framebuffer.set_current_color(wall_color(hit.wall_type, hit.side));
        for y in draw_start..=draw_end {
            framebuffer.point(x, y);
        }
    }
}
