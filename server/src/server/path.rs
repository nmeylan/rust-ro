use crate::server::map::Map;
use crate::server::movement::Position;
use std::collections::LinkedList;
use std::time::Instant;

// Coming from herculesWS
pub static MOVE_COST: u16 = 10;
pub static MOVE_DIAGONAL_COST: u16 = 10;

static DIR_NORTH: u8 = 1;
static DIR_WEST: u8 = 2;
static DIR_SOUTH: u8 = 4;
static DIR_EAST: u8 = 8;

struct Path {}

#[derive(Copy, Clone, Debug)]
pub struct PathNode {
    pub id: u32,
    pub parent_id: u32,
    pub x: u16,
    pub y: u16,
    pub g_cost: u16,
    pub f_cost: u16, // g_cost + heuristic
}

impl PathNode {
    pub fn set_parent_id(&mut self, parent_id: u32) {
        self.parent_id = parent_id;
    }
    pub fn set_gcost(&mut self, g_cost: u16) {
        self.g_cost = g_cost;
    }
    pub fn set_fcost(&mut self, f_cost: u16) {
        self.f_cost = f_cost;
    }
}

// Coming from herculesWS
#[inline]
fn client_side_heuristic(x0: u16, y0: u16, x1: u16, y1: u16) -> u16 {
    MOVE_COST * (i16::abs((x1 as i16 - x0 as i16)) as u16 + i16::abs((y1 as i16 - y0 as i16)) as u16) // Manhattan distance
}

#[inline]
fn is_direction(allowed_dir: u8, direction: u8) -> bool {
    allowed_dir & direction == direction
}

#[inline]
fn node_id(x: u16, y: u16, x_size: u16) -> u32 {
    x  as u32 + y  as u32 * x_size as u32
}

/**
* Client use a A* algorithm for path finding.
*/
pub fn path_search_client_side_algorithm(map: &Map, source: &Position, destination: &Position) -> Vec<PathNode> {
    let start = Instant::now();
    let max_x = (map.x_size - 1);
    let max_y = (map.y_size - 1);
    let mut start_node = PathNode {
        id: node_id(source.x, source.y, max_x),
        parent_id: node_id(source.x, source.y, max_x),
        x: source.x as u16,
        y: source.y as u16,
        g_cost: 0,
        f_cost: client_side_heuristic(source.x, source.y, destination.x, destination.y),
    };
    let mut open_set = vec![start_node];
    let mut discovered_nodes = vec![start_node];
    let mut current_node = start_node;
    while !open_set.is_empty() {
        let mut open_set_iter = open_set.iter();
        let init_node = open_set_iter.next().unwrap();
        let mut current: (usize, &PathNode) = open_set_iter.enumerate().fold((0, init_node), |(min_node_index, min_node), (cur_index, cur_node)| {
            if cur_node.f_cost <= min_node.f_cost {
                return (cur_index, cur_node)
            }
            (min_node_index, min_node)
        }); // current node is the node with minimum f_cost
        current_node = current.1.clone();
        let current_index = current.0;
        if current_node.x == destination.x && current_node.y == destination.y {
            break;
        }
        open_set.remove(current_index);

        let mut allowed_dirs: u8 = 0;
        if current_node.y < max_y && map.is_cell_walkable(current_node.x, current_node.y + 1) {
            allowed_dirs |= DIR_NORTH;
        }
        if current_node.y > 0 && map.is_cell_walkable(current_node.x, current_node.y - 1) {
            allowed_dirs |= DIR_SOUTH;
        }
        if current_node.x < max_x && map.is_cell_walkable(current_node.x + 1, current_node.y) {
            allowed_dirs |= DIR_EAST;
        }
        if current_node.x > 0 && map.is_cell_walkable(current_node.x - 1, current_node.y) {
            allowed_dirs |= DIR_WEST;
        }
        /**
        For all allowed neighbor cells
        **/
        if is_direction(allowed_dirs, DIR_SOUTH | DIR_EAST) && map.is_cell_walkable(current_node.x + 1, current_node.y - 1) {
            add_neighbor(current_node.x + 1, current_node.y - 1, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_EAST) {
            add_neighbor(current_node.x + 1, current_node.y, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
        if is_direction(allowed_dirs, DIR_NORTH | DIR_EAST) && map.is_cell_walkable(current_node.x + 1, current_node.y + 1) {
            add_neighbor(current_node.x + 1, current_node.y + 1, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_NORTH) {
            add_neighbor(current_node.x, current_node.y + 1, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
        if is_direction(allowed_dirs, DIR_NORTH | DIR_WEST) && map.is_cell_walkable(current_node.x - 1, current_node.y + 1) {
            add_neighbor(current_node.x - 1, current_node.y + 1, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_WEST) {
            add_neighbor(current_node.x - 1, current_node.y, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
        if is_direction(allowed_dirs, DIR_SOUTH | DIR_WEST) && map.is_cell_walkable(current_node.x - 1, current_node.y - 1) {
            add_neighbor(current_node.x - 1, current_node.y - 1, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_SOUTH) {
            add_neighbor(current_node.x, current_node.y - 1, destination, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
    }

    let mut final_path : Vec<PathNode> = vec![];
    while current_node.id != current_node.parent_id {
        final_path.push(current_node);
        let current_node_option = discovered_nodes.iter().find(|node| node.id == current_node.parent_id);
        if current_node_option.is_none() {
            break;
        }
        current_node = *current_node_option.unwrap();
    }
    final_path.reverse();
    final_path
}

fn add_neighbor(x: u16, y: u16, destination: &Position, max_x: u16, open_set: &mut Vec<PathNode>, discovered_nodes: &mut Vec<PathNode>, current_node: &PathNode, move_cost: u16) {
    let neighbor_option = discovered_nodes.iter().find(|node| node.x == x && node.y == y);
    let tentative_gcost = current_node.g_cost + move_cost;
    let h_cost = client_side_heuristic(x, y, destination.x, destination.y);
    let mut neighbor: PathNode;
    if neighbor_option.is_some() {
        neighbor = *neighbor_option.unwrap();
        if tentative_gcost < neighbor.g_cost {
            neighbor.set_parent_id(current_node.id);
            neighbor.set_gcost(tentative_gcost);
            neighbor.set_fcost(tentative_gcost + h_cost);
            if open_set.iter().any(|node| node.x == neighbor.x && node.y == neighbor.y) {
                open_set.push(neighbor);
            }
        }
    } else {
        neighbor = PathNode {
            id: node_id(x, y, max_x),
            parent_id: current_node.id,
            x,
            y,
            g_cost: tentative_gcost,
            f_cost: tentative_gcost + h_cost
        };
        open_set.push(neighbor);
        discovered_nodes.push(neighbor);
    }
}
