use crate::server::core::map::MapItem;
use crate::server::core::position::Position;
use crate::server::enums::map_item::MapItemType;
use crate::server::server::Server;

impl Server {
    #[inline]
    pub fn map_item_x_y(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Position> {
        match map_item.object_type() {
            MapItemType::Character => {
                let characters = self.characters.borrow();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(Position{ x: character.x(), y: character.y(), dir: 3 }); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(mob) = map_instance.get_mob(map_item.id()) {
                        return Some(Position{ x: mob.x(), y: mob.y(), dir: 3 }); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(Position{ x: warp.x(), y: warp.y(), dir: 0 });
                    }
                }
                None
            }
            MapItemType::Unknown => {
                None
            }
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(Position{ x: script.x(), y: script.y(), dir: script.dir() });
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item_name(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<String> {
        match map_item.object_type() {
            MapItemType::Character => {
                let characters = self.characters.borrow();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(character.name.clone()); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(mob) = map_instance.get_mob(map_item.id()) {
                        return Some(mob.name.clone()); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(warp.name.clone());
                    }
                }
                None
            }
            MapItemType::Unknown => {
                None
            }
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script.name().clone());
                    }
                }
                None
            }
        }
    }
}