use std::f32::consts::PI;
use std::fs::File;
use std::io::{BufRead, BufReader};

use nalgebra_glm::Vec2;

use crate::player::Player;

pub type Maze = Vec<Vec<char>>;

/// Carga un laberinto desde un archivo de texto. Heredado de la base del
/// profesor (rama 09-RC-03-MAZE-MOVEMENT); se irá extendiendo en etapas
/// posteriores para soportar múltiples niveles y validación de tamaño.
pub fn load_maze(filename: &str, block_size: usize) -> (Maze, Player) {
    let file = File::open(filename).expect("no se pudo abrir el archivo del laberinto");

    let reader = BufReader::new(file);

    let mut maze: Maze = Vec::new();

    let mut player_cell: Option<(usize, usize)> = None;

    for (row, line) in reader.lines().enumerate() {
        let line = line.expect("no se pudo leer una línea del laberinto");

        let mut cells: Vec<char> = Vec::new();

        for (col, character) in line.chars().enumerate() {
            if character == 'p' {
                player_cell = Some((row, col));
                cells.push(' ');
            } else {
                cells.push(character);
            }
        }

        maze.push(cells);
    }

    let (row, col) = player_cell.unwrap_or((0, 0));
    let pos = Vec2::new(
        (col * block_size + block_size / 2) as f32,
        (row * block_size + block_size / 2) as f32,
    );

    let player = Player {
        pos,
        // Se orienta automáticamente hacia un pasillo abierto en vez de un
        // ángulo fijo, para no depender de la forma de un laberinto en
        // particular (importante una vez que haya varios niveles, Etapa 10).
        a: facing_toward_open_cell(&maze, row, col),
    };

    (maze, player)
}

fn facing_toward_open_cell(maze: &Maze, row: usize, col: usize) -> f32 {
    let directions: [(i32, i32, f32); 4] = [
        (0, 1, 0.0),         // este
        (1, 0, PI / 2.0),    // sur
        (0, -1, PI),         // oeste
        (-1, 0, -PI / 2.0),  // norte
    ];

    for (dr, dc, angle) in directions {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nr < 0 || nc < 0 {
            continue;
        }

        if maze.get(nr as usize).and_then(|r| r.get(nc as usize)) == Some(&' ') {
            return angle;
        }
    }

    0.0
}
