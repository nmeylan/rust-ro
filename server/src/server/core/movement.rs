use crate::server::core::path::PathNode;
use crate::server::core::position::Position;

#[derive(Clone, Copy)]
pub struct Movement {
    position: Position,
    is_diagonal: bool,
    move_at: u128,
}

impl Movement {
    pub fn move_at(&self) -> u128 {
        self.move_at
    }
    pub fn set_move_at(&mut self, tick: u128) {
        self.move_at = tick
    }
    pub fn is_diagonal(&self) -> bool {
        self.is_diagonal
    }
    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn from_path(path: Vec<PathNode>, start_at: u128) -> Vec<Movement> {
        let mut movements = vec![];
        for path_node in path.iter() {
            let position = Position { x: path_node.x, y: path_node.y, dir: 0 };
            movements.push(Movement {
                position,
                is_diagonal: path_node.is_diagonal,
                move_at: start_at, // Will be re-set in game loop to take current player speed
            });
        }
        movements.reverse();
        movements
    }

    pub fn delay(speed: u16, is_diagonal: bool) -> u128 {
        if is_diagonal {
            (speed as f64 / 0.6) as u128
        } else {
            speed as u128
        }
    }
}
