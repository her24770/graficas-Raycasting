use nalgebra_glm::Vec2;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::sprite::Sprite;

/// Dibuja sprites tipo billboard (siempre de frente a cámara), respetando
/// el z-buffer que llenó `render::walls::render` para recortarse
/// correctamente contra las paredes más cercanas.
pub fn render(
    framebuffer: &mut Framebuffer,
    player: &Player,
    sprites: &[Sprite],
    fov: f32,
    block_size: f32,
    z_buffer: &[f32],
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;

    // Misma distancia al plano de proyección que usa `walls::render`, para
    // que un sprite y una pared a la misma distancia se vean del mismo
    // tamaño en pantalla.
    let dist_to_projection_plane = (height / 2.0) / (fov / 2.0).tan();

    let dir = Vec2::new(player.a.cos(), player.a.sin());
    let plane_len = (fov / 2.0).tan();
    let plane = Vec2::new(-dir.y * plane_len, dir.x * plane_len);

    for sprite in sprites {
        let d = sprite.pos - player.pos;

        let inv_det = 1.0 / (plane.x * dir.y - dir.x * plane.y);
        // transform_x: numerador para la posición horizontal en pantalla.
        // transform_y: la profundidad real (perpendicular a la cámara) —
        // esta es la que hay que usar para tamaño y z-buffer, no transform_x.
        let transform_x = inv_det * (dir.y * d.x - dir.x * d.y);
        let transform_y = inv_det * (-plane.y * d.x + plane.x * d.y);

        if transform_y <= 1.0 {
            continue; // detrás de la cámara o pegado a ella
        }

        let screen_x = (width / 2.0) * (1.0 + transform_x / transform_y);
        // Mismo tipo de fórmula que `wall_height` en walls.rs: cuántas
        // "celdas de distancia" hay, multiplicado por la distancia al
        // plano de proyección.
        let sprite_size = (block_size / transform_y) * dist_to_projection_plane;
        let half_size = sprite_size / 2.0;

        if screen_x + half_size < 0.0 || screen_x - half_size > width {
            continue; // completamente fuera de pantalla
        }

        let draw_start_x = (screen_x - half_size).max(0.0) as usize;
        let draw_end_x = (screen_x + half_size).min(width - 1.0) as usize;

        let center_y = height / 2.0;
        let draw_start_y = (center_y - half_size).max(0.0) as usize;
        let draw_end_y = (center_y + half_size).min(height - 1.0) as usize;

        let frame = sprite.current_frame();

        for x in draw_start_x..=draw_end_x {
            if transform_y >= z_buffer[x] {
                continue; // hay una pared más cerca en esta columna
            }

            let u = (x as f32 - (screen_x - half_size)) / sprite_size;

            for y in draw_start_y..=draw_end_y {
                let v = (y as f32 - (center_y - half_size)) / sprite_size;

                if let Some(color) = frame.sample(u, v) {
                    framebuffer.set_current_color(color);
                    framebuffer.point(x, y);
                }
            }
        }
    }
}
