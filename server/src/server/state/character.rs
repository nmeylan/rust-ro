use std::collections::HashSet;
use std::{io};
use std::io::Write;
use std::sync::{Arc, Mutex};
use accessor::Setters;
use enums::item::ItemType;
use crate::{get_job_config};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::core::action::Attack;
use crate::server::core::movement::Movement;
use crate::server::core::map_instance::{MapInstance, MapInstanceKey};
use crate::server::core::position::Position;
use crate::server::state::status::{LookType, Status};
use crate::server::core::map_item::{MapItem, MapItemSnapshot, MapItemType};
use crate::server::map_item::{ToMapItem, ToMapItemSnapshot};
use crate::server::script::ScriptGlobalVariableStore;

#[derive(Setters)]
pub struct Character {
    #[set]
    pub name: String,
    pub status: Status,
    #[set]
    pub char_id: u32,
    pub account_id: u32,
    pub map_instance_key: MapInstanceKey,
    pub loaded_from_client_side: bool,
    pub x: u16,
    pub y: u16,
    pub dir: u16,
    pub movements: Vec<Movement>,
    pub attack: Option<Attack>,
    pub inventory: Vec<Option<InventoryItemModel>>,
    pub map_view: HashSet<MapItem>,
    pub script_variable_store: Mutex<ScriptGlobalVariableStore>,
}

impl Character {
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    #[allow(dead_code)]
    pub fn dir(&self) -> u16 {
        self.dir
    }

    pub fn is_moving(&self) -> bool {
        !self.movements.is_empty()
    }
    pub fn is_attacking(&self) -> bool {
        self.attack.is_some()
    }

    pub fn pop_movement(&mut self) -> Option<Movement> {
        self.movements.pop()
    }
    pub fn peek_movement(&self) -> Option<&Movement> {
        self.movements.last()
    }
    pub fn peek_mut_movement(&mut self) -> Option<&mut Movement> {
        self.movements.last_mut()
    }

    pub fn set_movement(&mut self, movements: Vec<Movement>) {
        self.movements = movements;
    }
    pub fn clear_movement(&mut self) {
        self.movements = vec![];
    }
    pub fn set_attack(&mut self, target_id: u32, repeat: bool, tick: u128) {
        self.attack = Some(Attack {
            target: target_id,
            repeat,
            last_attack_tick: tick,
            last_attack_motion: 0,
        });
    }
    pub fn attack(&self) -> Attack {
        self.attack.unwrap()
    }
    pub fn update_last_attack_tick(&mut self, tick: u128) {
        self.attack.as_mut().unwrap().last_attack_tick = tick;
    }
    pub fn update_last_attack_motion(&mut self, attack_motion: u32) {
        self.attack.as_mut().unwrap().last_attack_motion = attack_motion;
    }
    pub fn clear_attack(&mut self) {
        self.attack = None;
    }

    pub fn join_and_set_map(&mut self, map_instance: Arc<MapInstance>) {
        self.set_current_map(map_instance.clone());
        map_instance.insert_item(self.to_map_item());
        self.movements = vec![];
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn set_current_map(&mut self, current_map: Arc<MapInstance>) {
        self.map_instance_key = MapInstanceKey::new(current_map.name.clone(), current_map.id);
    }

    pub fn current_map_name(&self) -> &String {
        self.map_instance_key.map_name()
    }

    pub fn current_map_instance(&self) -> u8 {
        self.map_instance_key.map_instance()
    }

    pub fn clear_map_view(&mut self) {
        self.map_view = Default::default();
    }


    pub fn get_look(&self, look_type: LookType) -> u32 {
        if self.status.look.is_none() {
            error!("Character has no look");
            return 0;
        }
        let look = self.status.look.as_ref().unwrap();
        match look_type {
            LookType::Hair => look.hair as u32,
            LookType::HairColor => look.hair_color,
            LookType::ClothesColor => look.clothes_color,
            LookType::Body => look.body,
            LookType::Weapon => look.weapon,
            LookType::Shield => look.shield,
            LookType::HeadBottom => look.head_bottom,
            LookType::HeadTop => look.head_top,
            LookType::HeadMid => look.head_middle,
            LookType::Robe => look.robe,
            _ => look.robe, // TODO
        }
    }

    pub fn change_look(&mut self, look: LookType, value: u32) -> Option<String> {
        if self.status.look.is_none() {
            error!("Character has no look");
            return None;
        }
        let db_column = match look {
            LookType::Hair => {
                self.status.look.as_mut().unwrap().hair = value as u16;
                "hair"
            }
            LookType::HairColor => {
                self.status.look.as_mut().unwrap().hair_color = value as u32;
                "hair_color"
            }
            LookType::ClothesColor => {
                self.status.look.as_mut().unwrap().clothes_color = value as u32;
                "clothes_color"
            }
            LookType::Body => {
                self.status.look.as_mut().unwrap().body = value as u32;
                "body"
            }
            LookType::Weapon => {
                self.status.look.as_mut().unwrap().weapon = value as u32;
                "weapon"
            }
            LookType::Shield => {
                self.status.look.as_mut().unwrap().shield = value as u32;
                "shield"
            }
            LookType::HeadBottom => {
                self.status.look.as_mut().unwrap().head_bottom = value as u32;
                "head_bottom"
            }
            LookType::HeadTop => {
                self.status.look.as_mut().unwrap().head_top = value as u32;
                "head_top"
            }
            LookType::HeadMid => {
                self.status.look.as_mut().unwrap().head_middle = value as u32;
                "head_mid"
            }
            LookType::Robe => {
                self.status.look.as_mut().unwrap().robe = value as u32;
                "robe"
            }
            _ => { "shoes" }
        };
        Some(db_column.to_string())
    }

    pub fn get_zeny(&self) -> u32 {
        self.status.zeny
    }

    pub fn change_zeny(&mut self, value: u32) {
        self.status.zeny = value;
    }

    pub fn add_items(&mut self, items: Vec<InventoryItemModel>) -> Vec<(usize, InventoryItemModel)> {
        let mut added_items = vec![];
        for item in items {
            if item.item_type.is_stackable() {
                if let Some((index, item_in_inventory)) = self.inventory.iter_mut().enumerate().filter(|(_, i)| i.is_some()).map(|(index, i)| (index, i.as_mut().unwrap())).find(|(_index, i)| i.item_id == item.item_id) {
                    item_in_inventory.amount += item.amount;
                    added_items.push((index, item.clone()));
                    continue;
                }
            }
            added_items.push((self.add_in_inventory(item.clone()), item));
        }
        added_items
    }

    fn add_in_inventory(&mut self, item: InventoryItemModel) -> usize {
        if let Some(position) = self.inventory.iter().position(|e| e.is_none()) {
            let _ = std::mem::replace(&mut self.inventory[position], Some(item));
            position
        } else {
            self.inventory.push(Some(item));
            self.inventory.len() - 1
        }
    }

    pub fn get_item_from_inventory(&self, index: usize) -> Option<&InventoryItemModel> {
        if let Some(inventory_slot) = self.inventory.get(index) {
            if inventory_slot.is_some() {
                return Some(inventory_slot.as_ref().unwrap());
            }
        }
        None
    }

    fn get_item_from_inventory_mut(&mut self, index: usize) -> Option<&mut InventoryItemModel> {
        if let Some(inventory_slot) = self.inventory.get_mut(index) {
            if inventory_slot.is_some() {
                return Some(inventory_slot.as_mut().unwrap());
            }
        }
        None
    }

    pub fn equip_item(&mut self, index: usize) -> Option<Vec<(usize, i32)>> {
        if let Some(item) = self.get_item_from_inventory(index) {
            let location = item.location;
            let id = item.id;
            let mut equipped_take_off_items = vec![];
            if item.item_type.is_equipment() {
                equipped_take_off_items.push(((index, location)));
                self.inventory.iter_mut().enumerate()
                    // Get only the items that are equipment and already equipped to same location
                    // TODO handle accessories should not use 2 slots
                    .filter(|(_, i)|  if let Some(j) = i { j.item_type.is_equipment() && (j.equip & location != 0) } else { false })
                    .for_each(|(item_index, item)| {
                        let item = item.as_mut().unwrap();
                        item.equip = 0;
                        equipped_take_off_items.push((item_index, item.location));
                    });
                self.get_item_from_inventory_mut(index).unwrap().equip = location;
                return Some(equipped_take_off_items);
            }
        }
        None
    }

    pub fn del_item_from_inventory(&mut self, index: usize, amount: i16) -> u16 {
        if let Some(inventory_slot) = self.inventory.get_mut(index) {
            if inventory_slot.is_some() {
                let item = inventory_slot.as_mut().unwrap();
                if item.amount - amount >= 0 {
                    item.amount -= amount;
                    return item.amount as u16;
                } else {
                    let _ = std::mem::replace(&mut self.inventory[index], None);
                    return 0;
                }
            }
        }
        0
    }

    pub fn weight(&self) -> u32 {
        self.inventory.iter()
            .filter(|item| item.is_some())
            .map(|item| {
                let item = item.as_ref().unwrap();
                item.weight as u32 * item.amount as u32
            })
            .sum()
    }
    pub fn max_weight(&self) -> u32 {
        let base_weight = get_job_config(self.status.class).base_weight();
        base_weight + (self.status.str * 300) as u32
    }

    pub fn check_weight(&self, additional_weight: u32) -> bool {
        (self.max_weight() as f32 * 0.9) as u32 > (self.weight() + additional_weight)
    }

    pub fn weapon_delay(&self) -> u32 {
        let weapon = "fist"; // TODO use equipped weapon
        *get_job_config(self.status.class).base_aspd().get(weapon).unwrap_or(&2000)
    }

    pub fn print(&self) {
        let mut stdout = io::stdout();
        writeln!(stdout, "************** {} - {} ****************", self.name, self.char_id).unwrap();
        writeln!(stdout, "Status:").unwrap();
        writeln!(stdout, "  str: {}", self.status.str).unwrap();
        writeln!(stdout, "  agi: {}", self.status.agi).unwrap();
        writeln!(stdout, "  vit: {}", self.status.vit).unwrap();
        writeln!(stdout, "  int: {}", self.status.int).unwrap();
        writeln!(stdout, "  dex: {}", self.status.dex).unwrap();
        writeln!(stdout, "  luk: {}", self.status.luk).unwrap();
        writeln!(stdout, "  speed: {}", self.status.speed).unwrap();
        writeln!(stdout, "  hp: {}/{}", self.status.hp, self.status.max_hp).unwrap();
        writeln!(stdout, "  sp: {}/{}", self.status.sp, self.status.max_sp).unwrap();
        writeln!(stdout, "  zeny: {}", self.status.zeny).unwrap();
        writeln!(stdout, "  weight: {}/{}", self.weight(), self.max_weight()).unwrap();
        writeln!(stdout, "Inventory:").unwrap();
        let mut inventory_print = |predicate: Box<dyn Fn(&(usize, &InventoryItemModel)) -> bool>| {
            self.inventory_iter()
                .filter(predicate)
                .for_each(|(index, item)| writeln!(stdout, " [{}] {} - {} ({})", index, item.name_english, item.item_id, item.amount).unwrap());
        };
        inventory_print(Box::new(|(_, item)| item.item_type.is_consumable()));
        inventory_print(Box::new(|(_, item)| item.item_type.is_equipment()));
        inventory_print(Box::new(|(_, item)| item.item_type.is_etc()));
        stdout.flush().unwrap();
    }

    pub fn inventory_equip(&self) -> Vec<(usize, &InventoryItemModel)> {
        self.inventory_iter()
            .filter(|(_, item)| item.item_type.is_equipment())
            .collect()
    }

    pub fn inventory_equipped(&self) -> Vec<(usize, &InventoryItemModel)> {
        self.inventory_iter()
            .filter(|(_, item)| item.item_type.is_equipment() && item.equip != 0)
            .collect()
    }

    pub fn inventory_normal(&self) -> Vec<(usize, &InventoryItemModel)> {
        self.inventory_iter()
            .filter(|(_, item)| !item.item_type.is_equipment())
            .collect()
    }

    fn inventory_iter(&self) -> Box<dyn Iterator<Item=(usize, &InventoryItemModel)> + '_> {
        Box::new(self.inventory.iter().enumerate()
            .filter(|(_, item)| item.is_some())
            .map(|(index, item)| (index, item.as_ref().unwrap())))
    }
}

impl ToMapItem for Character {
    fn to_map_item(&self) -> MapItem {
        let client_item_class = self.status.class as i16;
        MapItem::new(self.char_id, client_item_class, MapItemType::Character)
    }
}

impl ToMapItemSnapshot for Character {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot {
        let client_item_class = self.status.class as i16;
        MapItemSnapshot::new(
            MapItem::new(self.char_id, client_item_class, MapItemType::Character),
            Position { x: self.x, y: self.y, dir: self.dir },
        )
    }
}