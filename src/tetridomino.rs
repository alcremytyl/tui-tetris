use std::any::Any;

use ratatui::style::Color;

pub struct Tetridomino {
    color: Color,
    points: [[i8; 2]; 4],
}

pub const SHAPES: [Tetridomino; 7] = [
    Tetridomino {
        color: Color::from_u32(0x0027ddf1),
        points: [[0, 0], [0, 1], [0, 2], [0, 3]],
    }, // I
    Tetridomino {
        color: Color::from_u32(0x00f7e40a),
        points: [[0, 0], [1, 0], [0, 1], [1, 1]],
    }, // O-Tetromino
    Tetridomino {
        color: Color::from_u32(0x00ae59e3),
        points: [[0, 0], [1, 0], [2, 0], [1, 1]],
    }, // T-Tetromino
    Tetridomino {
        color: Color::from_u32(0x0032c75c),
        points: [[1, 0], [2, 0], [0, 1], [1, 1]],
    }, // S-Tetromino
    Tetridomino {
        color: Color::from_u32(0x00ed3b3b),
        points: [[0, 0], [1, 0], [1, 1], [2, 1]],
    }, // Z-Tetromino
    Tetridomino {
        color: Color::from_u32(0x00198ae1),
        points: [[0, 0], [1, 0], [2, 0], [2, 1]],
    }, // J-Tetromino
    Tetridomino {
        color: Color::from_u32(0x00ff9c33),
        points: [[0, 0], [1, 0], [2, 0], [0, 1]],
    }, // L-Tetromino
];

const ROT90: [[i8; 2]; 2] = [[0, -1], [1, 0]];
const ROT270: [[i8; 2]; 2] = [[0, 1], [-1, 0]];

pub enum Rotation {
    LEFT,
    RIGHT,
}

impl Tetridomino {
    pub fn rotate(self, rotation: Rotation) {
        let transform = match rotation {
            Rotation::LEFT => ROT270,
            Rotation::RIGHT => ROT90,
        };

        // TODO: do something with this value

        self.points
            .iter()
            .zip(transform.iter())
            .map(|(arr1, arr2)| arr1.iter().zip(arr2).map(|(x, y)| x * y))
            .collect();
    }
}
