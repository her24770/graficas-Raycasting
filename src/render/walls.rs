use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::raycaster::cast_ray;
use crate::textures::TextureAtlas;

const CEILING_COLOR: u32 = 0x404060;
const FLOOR_COLOR: u32 = 0x303030;

/// Color plano de respaldo por tipo de pared, usado cuando no hay textura
/// cargada para ese material (archivo faltante, o todavía no se agregó).
/// También la reutiliza el minimapa para pintar cada celda.
pub fn fallback_color(wall_type: char) -> u32 {
    match wall_type {
        '1' => 0x888888,
        '2' => 0xAA4433,
        '3' => 0x8B5A2B,
        '4' => 0xB0B8C0,
        'g' | 'G' => 0x00FF00,
        _ => 0xFFDDDD,
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
/// perpendicular corregida (para evitar el efecto fisheye), texturizada por
/// tipo de pared con `tex_x` (horizontal) y un `tex_y` interpolado por fila.
pub fn render(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    fov: f32,
    block_size: f32,
    textures: &TextureAtlas,
) {
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

        let shade_factor = if hit.side == 1 { 0.7 } else { 1.0 };

        match textures.get(hit.wall_type) {
            Some(texture) => {
                // Cuánto avanza la coordenada vertical de la textura por
                // cada píxel de pantalla; si la pared está recortada arriba
                // o abajo (más alta que la ventana), se arranca desde el
                // punto de la textura que le corresponde a ese recorte.
                let tex_step = texture.height as f32 / wall_height;
                let clipped_top = half - wall_height / 2.0;
                let mut tex_pos = (draw_start as f32 - clipped_top) * tex_step;

                for y in draw_start..=draw_end {
                    let v = (tex_pos / texture.height as f32).fract();
                    tex_pos += tex_step;

                    let color = shade(texture.sample(hit.tex_x, v), shade_factor);
                    framebuffer.set_current_color(color);
                    framebuffer.point(x, y);
                }
            }
            None => {
                let color = shade(fallback_color(hit.wall_type), shade_factor);
                framebuffer.set_current_color(color);
                for y in draw_start..=draw_end {
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
