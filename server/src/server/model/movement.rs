use models::position::Position;

use crate::server::model::path::PathNode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movement {
    position: Position,
    is_diagonal: bool,
    move_at: u128,
}

impl Movement {
    /// Used in tests
    pub fn new(x: u16, y: u16, move_at: u128) -> Self {
        Self {
            position: Position { x, y, dir: 0 },
            is_diagonal: false,
            move_at,
        }
    }

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
            let position = Position {
                x: path_node.x,
                y: path_node.y,
                dir: 0,
            };
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
        if is_diagonal { (speed as f64 / 0.7) as u128 } else { speed as u128 }
    }
}

pub trait Movable {
    fn is_moving(&self) -> bool {
        !self.movements().is_empty()
    }
    fn pop_movement(&mut self) -> Option<Movement> {
        self.movements_mut().pop()
    }
    fn movements_mut(&mut self) -> &mut Vec<Movement>;
    fn movements(&self) -> &Vec<Movement>;
    fn peek_movement(&self) -> Option<&Movement> {
        self.movements().last()
    }
    fn peek_mut_movement(&mut self) -> Option<&mut Movement> {
        self.movements_mut().last_mut()
    }
    fn shift_movement(&mut self, movement: Movement) {
        self.movements_mut().insert(0, movement);
    }
    fn push_movement(&mut self, movement: Movement) {
        self.movements_mut().push(movement);
    }

    fn set_movement(&mut self, movements: Vec<Movement>);
    fn clear_movement(&mut self) {
        *self.movements_mut() = vec![];
    }
}
