use std::sync::Arc;
use accessor::Setters;
use log::{debug};

use crate::server::core::map_instance::MapInstance;


// Coming from herculesWS
pub static MOVE_COST: u16 = 10;
pub static MOVE_DIAGONAL_COST: u16 = 14;

pub static DIR_NORTH: u8 = 1;
pub static DIR_WEST: u8 = 2;
pub static DIR_SOUTH: u8 = 4;
pub static DIR_EAST: u8 = 8;

#[derive(Copy, Clone, Debug, Setters)]
pub struct PathNode {
    pub id: u32,
    #[set]
    pub parent_id: u32,
    pub x: u16,
    pub y: u16,
    #[set]
    pub g_cost: u16,
    #[set]
    pub f_cost: u16, // g_cost + heuristic
    #[set]
    pub is_open: bool
}

// Coming from herculesWS
#[inline]
fn client_side_heuristic(x0: u16, y0: u16, x1: u16, y1: u16) -> u16 {
    MOVE_COST * manhattan_distance(x0, y0, x1, y1)
}

#[inline]
pub fn manhattan_distance(x0: u16, y0: u16, x1: u16, y1: u16) -> u16 {
    i16::abs(x1 as i16 - x0 as i16) as u16 + i16::abs(y1 as i16 - y0 as i16) as u16
}

#[inline]
pub fn is_direction(allowed_dir: u8, direction: u8) -> bool {
    (allowed_dir & direction) == direction
}

#[inline]
fn node_id(x: u16, y: u16, x_size: u16) -> u32 {
    x  as u32 + y  as u32 * x_size as u32
}

/**
* Client use a A* algorithm for path finding.
*/
pub fn path_search_client_side_algorithm(map: Arc<MapInstance>, source_x: u16, source_y: u16, destination_x: u16, destination_y: u16) -> Vec<PathNode> {
    let max_x = map.x_size - 1;
    let max_y = map.y_size - 1;
    let start_node = PathNode {
        id: node_id(source_x, source_y, max_x),
        parent_id: node_id(source_x, source_y, max_x),
        x: source_x as u16,
        y: source_y as u16,
        g_cost: 0,
        f_cost: client_side_heuristic(source_x, source_y, destination_x, destination_y),
        is_open: true,
    };
    let mut open_set = Vec::with_capacity(14 * 14);
    open_set.push(start_node);
    let mut discovered_nodes = Vec::with_capacity(14 * 14);
    discovered_nodes.push(start_node);
    let mut current_node = start_node;
    let mut i = 0;
    while !open_set.is_empty() {
        let open_set_iter = open_set.iter();
        let current: (usize, &PathNode) = open_set_iter.enumerate().reduce(|(min_node_index, min_node), (cur_index, cur_node)| {
            if cur_node.f_cost < min_node.f_cost {
                return (cur_index, cur_node)
            }
            (min_node_index, min_node)
        }).unwrap(); // current node is the node with minimum f_cost
        current_node = current.1.clone();
        let current_index = current.0;
        if current_node.x == destination_x && current_node.y == destination_y {
            // debug!("found destination");
            break;
        }
        if i > 100 {
            // debug!("Path finding stuck in infinite loop. Abort");
            return vec![]
        }
        // debug!("{:?}", open_set);
        // debug!("iteration: {} -> current_node({}) {},{}", i, current_index, current_node.x, current_node.y);
        open_set.swap_remove(current_index);
        // debug!("{:?}", open_set);
        current_node.set_is_open(false);
        i += 1;
        let allowed_dirs = allowed_dirs(max_x, max_y, current_node.x ,current_node.y);
        /*
        For all allowed neighbor cells
        */
        if is_direction(allowed_dirs, DIR_SOUTH | DIR_EAST) && map.is_cell_walkable(current_node.x + 1, current_node.y - 1) {
            add_neighbor(current_node.x + 1, current_node.y - 1, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_EAST)  && map.is_cell_walkable(current_node.x + 1, current_node.y){
            add_neighbor(current_node.x + 1, current_node.y, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
        if is_direction(allowed_dirs, DIR_NORTH | DIR_EAST) && map.is_cell_walkable(current_node.x + 1, current_node.y + 1) {
            add_neighbor(current_node.x + 1, current_node.y + 1, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_NORTH) && map.is_cell_walkable(current_node.x, current_node.y + 1) {
            add_neighbor(current_node.x, current_node.y + 1, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
        if is_direction(allowed_dirs, DIR_NORTH | DIR_WEST) && map.is_cell_walkable(current_node.x - 1, current_node.y + 1) {
            add_neighbor(current_node.x - 1, current_node.y + 1, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_WEST) && map.is_cell_walkable(current_node.x - 1, current_node.y) {
            add_neighbor(current_node.x - 1, current_node.y, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
        if is_direction(allowed_dirs, DIR_SOUTH | DIR_WEST) && map.is_cell_walkable(current_node.x - 1, current_node.y - 1) {
            add_neighbor(current_node.x - 1, current_node.y - 1, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_DIAGONAL_COST);
        }
        if is_direction(allowed_dirs, DIR_SOUTH) && map.is_cell_walkable(current_node.x, current_node.y - 1) {
            add_neighbor(current_node.x, current_node.y - 1, destination_x, destination_y, max_x, &mut open_set, &mut discovered_nodes, &current_node, MOVE_COST);
        }
    }

    let mut final_path : Vec<PathNode> = Vec::with_capacity(14 * 2);
    while current_node.id != current_node.parent_id {
        final_path.push(current_node);
        let current_node_option = discovered_nodes.iter().find(|node| node.id == current_node.parent_id);
        if current_node_option.is_none() {
            debug!("current_node_option.is_none()");
            break;
        }
        current_node = *current_node_option.unwrap();
    }
    final_path.reverse();
    final_path
}

pub fn allowed_dirs(max_x: u16, max_y: u16, x: u16, y: u16) -> u8 {
    let mut allowed_dirs: u8 = 0;
    if y < max_y {
        allowed_dirs |= DIR_NORTH;
    }
    if y > 0 {
        allowed_dirs |= DIR_SOUTH;
    }
    if x < max_x {
        allowed_dirs |= DIR_EAST;
    }
    if x > 0 {
        allowed_dirs |= DIR_WEST;
    }
    allowed_dirs
}

fn add_neighbor(x: u16, y: u16, destination_x: u16, destination_y: u16, max_x: u16, open_set: &mut Vec<PathNode>, discovered_nodes: &mut Vec<PathNode>, current_node: &PathNode, move_cost: u16) {
    let neighbor_option = discovered_nodes.iter().find(|node| node.x == x && node.y == y);
    let tentative_gcost = current_node.g_cost + move_cost;
    let h_cost = client_side_heuristic(x, y, destination_x, destination_y);
    let mut neighbor: PathNode;
    if neighbor_option.is_some() {
        neighbor = *neighbor_option.unwrap();
        if tentative_gcost < neighbor.g_cost {
            neighbor.set_parent_id(current_node.id);
            neighbor.set_g_cost(tentative_gcost);
            neighbor.set_f_cost(tentative_gcost + h_cost);
            if !neighbor.is_open {
                open_set.push(neighbor);
            }
            neighbor.set_is_open(true);
        }
    } else {
        // debug!("neighbor: {},{}", x, y);
        neighbor = PathNode {
            id: node_id(x, y, max_x),
            parent_id: current_node.id,
            x,
            y,
            g_cost: tentative_gcost,
            f_cost: tentative_gcost + h_cost,
            is_open: true
        };
        open_set.push(neighbor);
        discovered_nodes.push(neighbor);
    }
}
