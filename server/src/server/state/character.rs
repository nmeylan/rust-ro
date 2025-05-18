use std::collections::HashSet;


use std::sync::Mutex;

use accessor::Setters;

use models::enums::item::ItemType;
use models::enums::look::LookType;
use models::item::EquippedItem;

use crate::repository::model::item_model::{InventoryItemModel, ItemModel};
use crate::server::model::action::{Attack, SkillInUse};
use crate::server::model::hotkey::Hotkey;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::{MapItem, MapItemSnapshot, MapItemType, ToMapItem, ToMapItemSnapshot};
use crate::server::model::movement::{Movable, Movement};
use crate::server::script::ScriptGlobalVariableStore;
use models::position::Position;
use models::status::Status;
use skills::Skill;


/// Character state
#[derive(Setters)]
pub struct Character {
    /// Name of the character, this value is never mutated
    #[set]
    pub name: String,
    /// Id of the character, this value is never mutated
    #[set]
    pub char_id: u32,
    /// Id of the account, this value is never mutated
    pub account_id: u32,
    pub status: Status,
    /// Character state only exist when player has join the game.
    /// Once a player join the game its character is always assigned to a map.
    /// map_instance_key is the key of the map instance on which character is currently assigned.
    pub map_instance_key: MapInstanceKey,
    pub loaded_from_client_side: bool,
    /// x position of the character on the map
    pub x: u16,
    /// y position of the character on the map
    pub y: u16,
    /// direction of the character on the map
    pub dir: u16,
    /// When a character is moving, it follow a path. It consist of a list of position to move at, at a given time, depending on its current speed.
    pub movements: Vec<Movement>,
    /// When a character is attacking, it has a target. It can repeat its attack, so we need also to keep track of the last attack, to know when next attack can occur.
    pub attack: Option<Attack>,
    /// When a character started using skill, it has probably target, cast time, cooldown, after cast walk delay... So we need to keep track of the skill lifecycle.
    pub skill_in_use: Option<SkillInUse>,
    /// Character inventory is a list of item. Their index in the Vec below is sent to client, client side inventory items identifier is the index in this Vec.
    /// When action are made in inventory in client side, client only send to the server the index of the item in the inventory.
    /// When an item is removed client side, index of all items do not change, then when another item is added to inventory, client expect we use the first free index.
    /// That is why we use a Vec of Option.
    pub inventory: Vec<Option<InventoryItemModel>>,
    /// Contains item being viewed by character, this set is updated at each tick of the game loop.
    pub map_view: HashSet<MapItem>,
    /// Some script can store global variable for the character
    pub script_variable_store: Mutex<ScriptGlobalVariableStore>,

    pub last_moved_at: u128,
    pub last_regen_hp_at: u128,
    pub last_regen_sp_at: u128,
    pub sit: bool,

    pub hotkeys: Vec<Hotkey>,
    // 1 male, 0 female
    pub sex: u8,
}

type InventoryIter<'a> = Box<dyn Iterator<Item=(usize, &'a InventoryItemModel)> + 'a>;

impl Movable for Character {
    fn movements_mut(&mut self) -> &mut Vec<Movement> {
        &mut self.movements
    }
    fn movements(&self) -> &Vec<Movement> {
        &self.movements
    }
    fn set_movement(&mut self, movements: Vec<Movement>) {
        self.movements = movements;
    }
}

impl Character {
    pub fn new(name: String, char_id: u32, account_id: u32, status: Status, x: u16, y: u16, dir: u16, last_map: String, sex: u8, hotkeys: Vec<Hotkey>) -> Self {
        Self {
            name,
            char_id,
            account_id,
            status,
            map_instance_key: MapInstanceKey::new(last_map, 0),
            loaded_from_client_side: false,
            x,
            y,
            dir,
            movements: vec![],
            attack: None,
            skill_in_use: None,
            inventory: vec![],
            map_view: Default::default(),
            script_variable_store: Default::default(),
            last_moved_at: 0,
            last_regen_hp_at: 0,
            last_regen_sp_at: 0,
            sit: false,
            hotkeys,
            sex,
        }
    }
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

    pub fn is_attacking(&self) -> bool {
        self.attack.is_some()
    }
    pub fn is_using_skill(&self) -> bool {
        self.skill_in_use.is_some()
    }

    pub fn set_attack(&mut self, target_id: u32, repeat: bool, tick: u128) {
        self.attack = Some(Attack {
            target: target_id,
            repeat,
            last_attack_tick: tick,
            last_attack_motion: 0,
        });
    }
    pub fn set_skill_in_use(&mut self, target_id: Option<u32>, start_skill_tick: u128, skill: Box<dyn Skill>) {
        self.skill_in_use = Some(SkillInUse {
            target: target_id,
            start_skill_tick,
            skill,
            used_at_tick: None,
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

    pub fn update_skill_used_at_tick(&mut self, tick: u128) {
        self.skill_in_use.as_mut().unwrap().used_at_tick = Some(tick);
    }

    pub fn skill_in_use(&self) -> &SkillInUse {
        self.skill_in_use.as_ref().unwrap()
    }
    pub fn skill_has_been_used(&self) -> bool {
        self.skill_in_use.as_ref().unwrap().used_at_tick.is_some()
    }

    pub fn clear_skill_in_use(&mut self) {
        self.skill_in_use = None;
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn set_last_moved_at(&mut self, tick: u128) {
        self.last_moved_at = tick;
    }

    pub fn set_current_map(&mut self, map_name: String, id: u8) {
        self.map_instance_key = MapInstanceKey::new(map_name, id);
    }

    pub fn set_current_map_with_key(&mut self, key: MapInstanceKey) {
        self.map_instance_key = key;
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

    pub fn is_map_item_in_fov(&self, id: u32) -> bool {
        self.map_view.iter().any(|item| item.id() == id)
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

    pub fn change_look(&mut self, look: LookType, value: u16) -> Option<String> {
        if self.status.look.is_none() {
            error!("Character has no look");
            return None;
        }
        let db_column = match look {
            LookType::Hair => {
                self.status.look.as_mut().unwrap().hair = value;
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
    pub fn get_job(&self) -> u32 {
        self.status.job
    }
    pub fn get_base_level(&self) -> u32 {
        self.status.base_level
    }
    pub fn get_base_class(&self) -> u32 {
        self.status.job
    }
    pub fn get_job_level(&self) -> u32 {
        self.status.job_level
    }
    pub fn get_skill_point(&self) -> u32 {
        self.status.skill_point
    }

    pub fn change_zeny(&mut self, value: u32) {
        self.status.zeny = value;
    }

    pub fn add_items(&mut self, items: Vec<InventoryItemModel>) -> Vec<(usize, InventoryItemModel)> {
        let mut added_items = vec![];
        for item in items {
            if item.item_type().is_stackable() {
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

    pub fn add_in_inventory(&mut self, item: InventoryItemModel) -> usize {
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

    pub fn takeoff_equip_item(&mut self, index: usize) -> Option<EquippedItem> {
        let res = if let Some(item) = self.get_item_from_inventory_mut(index) {
            let old_equip = item.equip;
            item.equip = 0;
            Some(EquippedItem { item_id: item.item_id, location: old_equip as u64, index })
        } else {
            None
        };
        let item = self.get_item_from_inventory(index);
        if let Some(item) = item {
            if matches!(item.item_type(), ItemType::Weapon) {
                self.status.takeoff_weapon(index);
            } else if matches!(item.item_type(), ItemType::Ammo) {
                self.status.takeoff_ammo();
            } else {
                self.status.takeoff_equipment(index);
            }
        }
        res
    }

    pub fn wear_equip_item(&mut self, index: usize, location: u64, item_to_equip_model: &ItemModel) -> Option<EquippedItem> {
        if location == 0 {
            return None
        }
        let res = if let Some(item) = self.get_item_from_inventory_mut(index) {
            item.equip = location as i32;
            Some(EquippedItem { item_id: item.item_id, location, index })
        } else {
            None
        };
        let item = self.get_item_from_inventory(index);
        if let Some(item) = item {
            if matches!(item.item_type(), ItemType::Weapon) {
                self.status.wear_weapon(item_to_equip_model.to_wear_weapon(index, location, item));
            } else if matches!(item.item_type(), ItemType::Ammo) {
                self.status.wear_ammo(item_to_equip_model.to_wear_ammo(index));
            } else {
                self.status.wear_equipment(item_to_equip_model.to_wear_gear(index, location, item));
            }
        }
        res
    }

    pub fn del_item_from_inventory(&mut self, index: usize, amount: i16) -> u16 {
        if let Some(inventory_slot) = self.inventory.get_mut(index) {
            if inventory_slot.is_some() {
                let item = inventory_slot.as_mut().unwrap();
                if item.amount - amount > 0 {
                    item.amount -= amount;
                    return item.amount as u16;
                } else {
                    let _ = self.inventory[index].take();
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

    pub fn inventory_equip(&self) -> Vec<(usize, &InventoryItemModel)> {
        self.inventory_iter()
            .filter(|(_, item)| item.item_type().is_equipment())
            .collect()
    }
    pub fn inventory_wearable(&self) -> Vec<(usize, &InventoryItemModel)> {
        self.inventory_iter()
            .filter(|(_, item)| item.item_type().is_wearable())
            .collect()
    }

    pub fn inventory_normal(&self) -> Vec<(usize, &InventoryItemModel)> {
        self.inventory_iter()
            .filter(|(_, item)| !item.item_type().is_equipment())
            .collect()
    }

    pub(crate) fn inventory_iter(&self) -> InventoryIter {
        Box::new(self.inventory.iter().enumerate()
            .filter(|(_, item)| item.is_some())
            .map(|(index, item)| (index, item.as_ref().unwrap())))
    }
}

impl ToMapItem for Character {
    fn to_map_item(&self) -> MapItem {
        let client_item_class = self.status.job as i16;
        MapItem::new(self.char_id, client_item_class, MapItemType::Character)
    }
}

impl ToMapItemSnapshot for Character {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot {
        MapItemSnapshot::new(
            self.to_map_item(),
            Position { x: self.x, y: self.y, dir: self.dir }
        )
    }
}