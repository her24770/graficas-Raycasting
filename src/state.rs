use nalgebra_glm::Vec2;

use crate::maze::{load_maze, Maze};
use crate::player::Player;
use crate::sprite::Sprite;

pub struct LevelInfo {
    pub name: &'static str,
    pub path: &'static str,
}

pub const LEVELS: [LevelInfo; 3] = [
    LevelInfo { name: "NIVEL 1", path: "assets/levels/level1.txt" },
    LevelInfo { name: "NIVEL 2", path: "assets/levels/level2.txt" },
    LevelInfo { name: "NIVEL 3", path: "assets/levels/level3.txt" },
];

const TORCH_COUNT: usize = 4;
/// Tiempo límite por intento, en segundos. Si se agota antes de llegar a
/// la meta, el nivel se pierde.
pub const TIME_LIMIT: f32 = 60.0;
/// Cuánto dura el parpadeo rojo de pantalla al chocar contra una pared.
pub const FLASH_DURATION: f32 = 0.4;

/// Todo el estado que solo existe mientras se está jugando un nivel. Se
/// crea de cero cada vez que se entra a Playing (desde Welcome), así que
/// no hay estado viejo de una partida anterior que sobreviva por error.
pub struct PlayingState {
    pub level_index: usize,
    pub maze: Maze,
    pub player: Player,
    pub sprites: Vec<Sprite>,
    pub z_buffer: Vec<f32>,
    pub mouse_prev_x: Option<f32>,
    pub collisions: u32,
    /// Si el jugador ya estaba chocando contra una pared el frame pasado,
    /// para contar golpes (choques nuevos), no cuadros de contacto seguido.
    pub was_blocked: bool,
    pub time_left: f32,
    /// Cuenta regresiva del parpadeo rojo; 0.0 significa apagado.
    pub collision_flash: f32,
}

pub enum GameState {
    Welcome { selected: usize },
    Playing(PlayingState),
    Success { level_index: usize, collisions: u32, time_used: f32 },
    TimeUp { level_index: usize, collisions: u32 },
}

fn cell_to_pos(row: usize, col: usize, block_size: f32) -> Vec2 {
    Vec2::new(
        col as f32 * block_size + block_size / 2.0,
        row as f32 * block_size + block_size / 2.0,
    )
}

fn manhattan(a: (usize, usize), b: (usize, usize)) -> i32 {
    (a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()
}

/// Elige `count` celdas abiertas del laberinto, repartidas por el mapa en
/// vez de amontonadas: arranca en `start` (la celda del jugador) y en cada
/// paso agrega la celda abierta que quede más lejos de todas las ya
/// elegidas (farthest point sampling).
fn scatter_open_cells(maze: &Maze, start: (usize, usize), count: usize, block_size: f32) -> Vec<Vec2> {
    let open_cells: Vec<(usize, usize)> = maze
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|&(_, &cell)| cell == ' ')
                .map(move |(col, _)| (row, col))
        })
        .filter(|&cell| cell != start)
        .collect();

    let mut chosen: Vec<(usize, usize)> = Vec::new();
    let mut reference = vec![start];

    for _ in 0..count.min(open_cells.len()) {
        let next = open_cells
            .iter()
            .filter(|cell| !chosen.contains(cell))
            .max_by_key(|&&cell| reference.iter().map(|&r| manhattan(cell, r)).min().unwrap_or(0));

        match next {
            Some(&cell) => {
                chosen.push(cell);
                reference.push(cell);
            }
            None => break,
        }
    }

    chosen
        .into_iter()
        .map(|(row, col)| cell_to_pos(row, col, block_size))
        .collect()
}

/// Busca la celda de meta ('g'/'G') en el laberinto, para ubicar ahí el
/// sprite del cofre.
fn find_goal_cell(maze: &Maze, block_size: f32) -> Option<Vec2> {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 'g' || cell == 'G' {
                return Some(cell_to_pos(row, col, block_size));
            }
        }
    }
    None
}

impl PlayingState {
    pub fn new(level_index: usize, block_size: usize, framebuffer_width: usize) -> Self {
        let level = &LEVELS[level_index];
        let (maze, player) = load_maze(level.path, block_size);

        let player_start_cell = (
            (player.pos.y / block_size as f32) as usize,
            (player.pos.x / block_size as f32) as usize,
        );
        let torch_positions = scatter_open_cells(&maze, player_start_cell, TORCH_COUNT, block_size as f32);
        let mut sprites: Vec<Sprite> = torch_positions.into_iter().map(Sprite::torch).collect();

        if let Some(goal_pos) = find_goal_cell(&maze, block_size as f32) {
            sprites.push(Sprite::chest(goal_pos));
        }

        PlayingState {
            level_index,
            maze,
            player,
            sprites,
            z_buffer: vec![f32::MAX; framebuffer_width],
            mouse_prev_x: None,
            collisions: 0,
            was_blocked: false,
            time_left: TIME_LIMIT,
            collision_flash: 0.0,
        }
    }

    /// True si el jugador está parado sobre la celda de meta ('g').
    pub fn reached_goal(&self, block_size: usize) -> bool {
        let i = self.player.pos.x as usize / block_size;
        let j = self.player.pos.y as usize / block_size;
        self.maze.get(j).and_then(|row| row.get(i)) == Some(&'g')
    }

    /// Tiempo que tardó el intento hasta ahora (para mostrar al terminar).
    pub fn time_used(&self) -> f32 {
        TIME_LIMIT - self.time_left.max(0.0)
    }
}
