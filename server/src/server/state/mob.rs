
use std::collections::HashMap;



use rand::Rng;
use packets::packets::PacketZcNotifyMove;


use crate::server::core::position::Position;
use crate::server::core::map::Map;
use crate::server::core::map_instance::{MapInstanceKey};
use crate::server::state::status::Status;
use crate::server::core::map_item::{MapItem, MapItemType};
use crate::server::map_item::ToMapItem;
use crate::util::tick::{get_tick_client};

#[derive(Setters)]
pub struct Mob {
    pub id: u32,
    pub name: String,
    pub mob_id: i16,
    pub spawn_id: u32,
    pub status: Status,
    #[set]
    pub x: u16,
    #[set]
    pub y: u16,
    pub current_map: MapInstanceKey,
    pub map_view: Vec<MapItem>,
    pub is_view_char: bool,
}

impl Mob {
    pub fn new(id: u32, x: u16, y: u16, mob_id: i16, spawn_id: u32, name: String, current_map: MapInstanceKey, status: Status) -> Mob {
        Mob {
            id,
            x,
            y,
            mob_id,
            spawn_id,
            status,
            name,
            map_view: vec![],
            current_map,
            is_view_char: false
        }
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn update_map_view(&mut self, map_items: Vec<MapItem>) {
        self.is_view_char = !map_items.is_empty();
        self.map_view = map_items;
    }

    pub fn action_move(&mut self, cells: &Vec<u16>, x_size: u16, y_size: u16) -> HashMap<MapItem, PacketZcNotifyMove> {
        let mut rng = rand::thread_rng();
        let mut character_packets_map: HashMap<MapItem, PacketZcNotifyMove> = HashMap::new();
        let rand = rng.gen_range(0..=100);
        let should_move = if self.is_view_char {
            rand <= 80
        } else {
            rand <= 10
        };

        if should_move {
            let rand_distance = rng.gen_range(2..=8);
            let current_x = self.x;
            let current_y = self.y;
            let (x, y) = Map::find_random_walkable_cell_in_max_range(cells, x_size, y_size, current_x, current_y, rand_distance);
            // Todo: implement server side movement, to avoid desync between client and server
            self.x = x;
            self.y = y;
            if self.is_view_char {
                let from = Position {
                    x: current_x,
                    y: current_y,
                    dir: 0
                };
                let to = Position {
                    x,
                    y,
                    dir: 0
                };
                self.map_view.iter()
                    .filter(|map_item| map_item.object_type_value() == MapItemType::Character.value())
                    .for_each(|map_item| {
                        let mut packet_zc_notify_move = PacketZcNotifyMove::default();
                        packet_zc_notify_move.set_gid(self.id);
                        packet_zc_notify_move.move_data = from.to_move_data(&to);
                        let start_time = get_tick_client();
                        packet_zc_notify_move.set_move_start_time(start_time);
                        packet_zc_notify_move.fill_raw();
                        character_packets_map.insert(map_item.clone(), packet_zc_notify_move);
                    })
            }
        }
        character_packets_map
    }
}

impl ToMapItem for Mob {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.mob_id, MapItemType::Mob)
    }
}