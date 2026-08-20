use nalgebra_glm::Vec2;

use crate::maze::Maze;

pub struct RayHit {
    /// Distancia euclidiana cruda (en píxeles de mundo) desde el origen
    /// hasta el punto de impacto, medida a lo largo del rayo. La corrección
    /// de fisheye (proyectar sobre la dirección de vista del jugador) se
    /// aplica en el renderer, no aquí, porque aquí no conocemos el ángulo
    /// de referencia de la cámara, solo el del rayo individual.
    pub distance: f32,
    pub wall_type: char,
    /// 0 = se golpeó una cara vertical (se avanzó en x), 1 = horizontal (se avanzó en y).
    pub side: u8,
    /// Posición fraccional (0.0..1.0) a lo largo de la cara golpeada, para
    /// mapear la coordenada horizontal de una textura más adelante.
    pub tex_x: f32,
}

/// Lanza un rayo desde `origin` (en píxeles de mundo) en la dirección `angle`,
/// usando DDA por celda sobre la grilla del laberinto. Devuelve `None` si el
/// rayo sale del laberinto sin golpear nada (no debería pasar en un mapa
/// cerrado, pero así nunca hay un loop infinito ni un panic por índice).
pub fn cast_ray(maze: &Maze, origin: Vec2, angle: f32, block_size: f32) -> Option<RayHit> {
    let dir_x = angle.cos();
    let dir_y = angle.sin();

    let pos_x = origin.x / block_size;
    let pos_y = origin.y / block_size;

    let mut map_x = pos_x.floor() as i32;
    let mut map_y = pos_y.floor() as i32;

    let delta_dist_x = if dir_x == 0.0 { f32::MAX } else { (1.0 / dir_x).abs() };
    let delta_dist_y = if dir_y == 0.0 { f32::MAX } else { (1.0 / dir_y).abs() };

    let (step_x, mut side_dist_x) = if dir_x < 0.0 {
        (-1, (pos_x - map_x as f32) * delta_dist_x)
    } else {
        (1, (map_x as f32 + 1.0 - pos_x) * delta_dist_x)
    };

    let (step_y, mut side_dist_y) = if dir_y < 0.0 {
        (-1, (pos_y - map_y as f32) * delta_dist_y)
    } else {
        (1, (map_y as f32 + 1.0 - pos_y) * delta_dist_y)
    };

    let max_rows = maze.len();
    let max_cols = maze.iter().map(|row| row.len()).max().unwrap_or(0);
    let max_steps = (max_rows + max_cols) * 2 + 16;

    let mut side;
    let wall_type;
    let mut steps = 0;

    loop {
        steps += 1;
        if steps > max_steps {
            return None;
        }

        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0u8;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1u8;
        }

        if map_x < 0 || map_y < 0 {
            return None;
        }

        match maze.get(map_y as usize).and_then(|row| row.get(map_x as usize)) {
            None => return None,
            Some(' ') => continue,
            Some(&c) => {
                wall_type = c;
                break;
            }
        }
    }

    let raw_distance = if side == 0 {
        side_dist_x - delta_dist_x
    } else {
        side_dist_y - delta_dist_y
    };

    let tex_x = if side == 0 {
        let wall_y = pos_y + raw_distance * dir_y;
        wall_y - wall_y.floor()
    } else {
        let wall_x = pos_x + raw_distance * dir_x;
        wall_x - wall_x.floor()
    };

    Some(RayHit {
        distance: raw_distance * block_size,
        wall_type,
        side,
        tex_x,
    })
}
