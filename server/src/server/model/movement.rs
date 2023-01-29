use crate::server::model::path::PathNode;
use crate::server::model::position::Position;

#[derive(Clone, Copy, Debug)]
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

    fn set_movement(&mut self, movements: Vec<Movement>);
    fn clear_movement(&mut self) {
        *self.movements_mut() = vec![];
    }
}