use std::collections::HashSet;
use std::{io, mem};
use std::io::Write;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Once};
use tokio::runtime::Runtime;

use enums::class::JobName;
use enums::item::EquipmentLocation;
use enums::look::LookType;
use enums::status::StatusTypes;

use crate::enums::EnumWithNumberValue;
use crate::enums::EnumWithMaskValueU64;

use packets::packets::{Packet, PacketZcLongparChange, PacketZcNotifyStandentry7, PacketZcNotifyVanish, PacketZcNpcackMapmove, PacketZcParChange, PacketZcSpriteChange2};
use crate::repository::model::item_model::InventoryItemModel;
use crate::repository::{CharacterRepository, Repository};
use crate::server::model::events::game_event::{CharacterChangeMap, CharacterLook, CharacterZeny};
use crate::server::model::map::{MAP_EXT};
use crate::server::model::map_item::{MapItem, MapItemType};
use crate::server::model::path::manhattan_distance;

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::persistence_event::{PersistenceEvent, SavePositionUpdate, StatusUpdate};
use crate::server::model::events::persistence_event::PersistenceEvent::SaveCharacterPosition;
use crate::server::{PLAYER_FOV, Server};

use crate::server::service::global_config_service::GlobalConfigService;



use crate::server::state::character::Character;
use crate::server::state::map_instance::MapInstanceState;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;

static mut SERVICE_INSTANCE: Option<CharacterService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct CharacterService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<dyn CharacterRepository>,
    configuration_service: &'static GlobalConfigService
}

impl CharacterService {
    pub fn instance() -> &'static CharacterService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<Repository>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(CharacterService{ client_notification_sender, persistence_event_sender, repository, configuration_service });
        });
    }

    pub fn max_weight(&self, character: &Character) -> u32 {
        let base_weight = self.configuration_service.get_job_config(character.status.job).base_weight();
        base_weight + (character.status.str * 300) as u32
    }

    pub fn check_weight(&self, character: &Character, additional_weight: u32) -> bool {
        (self.max_weight(character) as f32 * 0.9) as u32 > (character.weight() + additional_weight)
    }

    pub fn print(&self, character: &Character) {
        let mut stdout = io::stdout();
        writeln!(stdout, "************** {} - {} ****************", character.name, character.char_id).unwrap();
        writeln!(stdout, "Status:").unwrap();
        writeln!(stdout, "  str: {}", character.status.str).unwrap();
        writeln!(stdout, "  agi: {}", character.status.agi).unwrap();
        writeln!(stdout, "  vit: {}", character.status.vit).unwrap();
        writeln!(stdout, "  int: {}", character.status.int).unwrap();
        writeln!(stdout, "  dex: {}", character.status.dex).unwrap();
        writeln!(stdout, "  luk: {}", character.status.luk).unwrap();
        writeln!(stdout, "  speed: {}", character.status.speed).unwrap();
        writeln!(stdout, "  hp: {}/{}", character.status.hp, character.status.max_hp).unwrap();
        writeln!(stdout, "  sp: {}/{}", character.status.sp, character.status.max_sp).unwrap();
        writeln!(stdout, "  zeny: {}", character.status.zeny).unwrap();
        writeln!(stdout, "  weight: {}/{}", character.weight(), self.max_weight(character)).unwrap();
        writeln!(stdout, "Inventory:").unwrap();
        type PredicateClosure =  Box<dyn Fn(&(usize, &InventoryItemModel)) -> bool>;
        let mut inventory_print = |predicate: PredicateClosure| {
            character.inventory_iter()
                .filter(predicate)
                .for_each(|(index, item)| writeln!(stdout, " [{}] {} - {} ({})", index, item.name_english, item.item_id, item.amount).unwrap());
        };
        inventory_print(Box::new(|(_, item)| item.item_type.is_consumable()));
        inventory_print(Box::new(|(_, item)| item.item_type.is_equipment()));
        inventory_print(Box::new(|(_, item)| item.item_type.is_etc()));
        writeln!(stdout, "Equipped items:").unwrap();
        character.inventory_equipped().for_each(|(index, item)| writeln!(stdout, " [{}] {} - {} ({:?}) at {:?}", index,
                                                                    item.name_english, item.item_id, item.item_type, EquipmentLocation::from_flag(item.equip as u64)).unwrap());
        stdout.flush().unwrap();
    }

    pub fn change_map(&self, map_instance_state: &MapInstanceState, event: &CharacterChangeMap, character: &mut Character) {
        character.set_current_map_with_key(map_instance_state.key().clone());
        character.movements = vec![];
        let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

        let mut new_current_map: [char; 16] = [0 as char; 16];
        let map_name = format!("{}{}", event.new_map_name, MAP_EXT);
        map_name.fill_char_array(new_current_map.as_mut());
        packet_zc_npcack_mapmove.set_map_name(new_current_map);
        let new_position = event.new_position.unwrap();
        packet_zc_npcack_mapmove.set_x_pos(new_position.x as i16);
        packet_zc_npcack_mapmove.set_y_pos(new_position.y as i16);
        packet_zc_npcack_mapmove.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_npcack_mapmove.raw_mut()))))
            .expect("Failed to send notification event with PacketZcNpcackMapmove");

        character.update_position(new_position.x, new_position.y);
        character.clear_map_view();
        character.loaded_from_client_side = false;
        self.persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: character.current_map_name().clone(), x: character.x(), y: character.y() }))
            .expect("Fail to send persistence notification");

    }

    pub fn change_look(&self, character_look: CharacterLook, character: &mut Character) {
        let db_column = character.change_look(character_look.look_type, character_look.look_value);
        if let Some(db_column) = db_column {
            self.change_sprite(character, character_look.look_type, character_look.look_value, 0);
            self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character_look.char_id, db_column, value: character_look.look_value as u32})).expect("Fail to send persistence notification");
        }
    }

    pub fn change_sprite(&self, character: &Character, look_type: LookType, look_value: u16, look_value2: u16) {
        let mut packet_zc_sprite_change = PacketZcSpriteChange2::new();
        packet_zc_sprite_change.set_gid(character.char_id);
        packet_zc_sprite_change.set_atype(look_type.value() as u8);
        packet_zc_sprite_change.set_value(look_value);
        packet_zc_sprite_change.set_value2(look_value2);
        packet_zc_sprite_change.fill_raw();
        self.send_area_notification_around_characters(character, packet_zc_sprite_change.raw);
    }

    pub fn send_area_notification_around_characters(&self, character: &Character, packets: Vec<u8>) {
        self.client_notification_sender.send(Notification::Area(AreaNotification {
            map_name: character.current_map_name().clone(),
            map_instance_id: character.current_map_instance(),
            range_type: AreaNotificationRangeType::Fov { x: character.x(), y: character.y(), exclude_id: None },
            packet: packets
        })).expect("Fail to send client notification");
    }

    pub fn update_zeny(&self, runtime: &Runtime, zeny_update: CharacterZeny, character: &mut Character) {
        let zeny = if let Some(zeny) = zeny_update.zeny {
            self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: zeny_update.char_id, value: zeny, db_column: "zeny".to_string() }))
                .expect("Fail to send persistence notification");
            zeny
        } else {
            runtime.block_on(async {
                self.repository.character_zeny_fetch(zeny_update.char_id).await.expect("failed to fetch zeny") as u32
            })
        };
        character.change_zeny(zeny);

        let mut packet_zc_longpar_change = PacketZcLongparChange::new();
        packet_zc_longpar_change.set_amount(character.get_zeny() as i32);
        packet_zc_longpar_change.set_var_id(StatusTypes::Zeny.value() as u16);
        packet_zc_longpar_change.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_longpar_change.raw_mut())))).expect("Fail to send client notification");
    }

    pub fn update_weight(&self, character: &mut Character) {
        let mut packet_weight = PacketZcParChange::new();
        packet_weight.set_var_id(StatusTypes::Weight.value() as u16);
        packet_weight.set_count(character.weight() as i32);
        packet_weight.fill_raw();
        let mut packet_max_weight = PacketZcParChange::new();
        packet_max_weight.set_var_id(StatusTypes::Maxweight.value() as u16);
        packet_max_weight.set_count(self.max_weight(character) as i32);
        packet_max_weight.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, chain_packets(vec![&packet_weight, &packet_max_weight])))).expect("Fail to send client notification");
    }

    pub fn update_base_level(&self, character: &mut Character, maybe_new_base_level: Option<u32>, maybe_level_delta: Option<i32>) -> i32 {
        let old_base_level = character.status.base_level;
        let new_base_level = if let Some(new_base_level) = maybe_new_base_level {
            new_base_level.min(self.configuration_service.config().game.max_base_level).max(1) as u32
        } else if let Some(add_level) = maybe_level_delta {
            ((old_base_level as i32 + add_level).min(self.configuration_service.config().game.max_base_level as i32).max(1)) as u32
        } else {
            old_base_level
        };
        character.status.base_level = new_base_level;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "base_level".to_string(), value: new_base_level})).expect("Fail to send persistence notification");
        let mut packet_base_level = PacketZcParChange::new();
        packet_base_level.set_var_id(StatusTypes::Baselevel.value() as u16);
        packet_base_level.set_count(new_base_level as i32);
        packet_base_level.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_base_level.raw))).expect("Fail to send client notification");
        (old_base_level as i32 - new_base_level as i32) as i32
    }

    pub fn update_job_level(&self, character: &mut Character, maybe_new_base_level: Option<u32>, maybe_level_delta: Option<i32>) -> i32 {
        let old_job_level = character.status.job_level;
        let new_job_level = if let Some(new_job_level) = maybe_new_base_level {
            new_job_level.min(self.configuration_service.config().game.max_job_level).max(1) as u32
        } else if let Some(add_level) = maybe_level_delta {
            ((old_job_level as i32 + add_level).min(self.configuration_service.config().game.max_job_level as i32).max(1)) as u32
        } else {
            old_job_level
        };
        character.status.job_level = new_job_level;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "job_level".to_string(), value: new_job_level})).expect("Fail to send persistence notification");
        let mut packet_job_level = PacketZcParChange::new();
        packet_job_level.set_var_id(StatusTypes::Joblevel.value() as u16);
        packet_job_level.set_count(new_job_level as i32);
        packet_job_level.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_job_level.raw))).expect("Fail to send client notification");
        (old_job_level as i32 - new_job_level as i32) as i32
    }

    pub fn change_job(&self, character: &mut Character, job: JobName) {
        character.status.job = job.value() as u32;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "class".to_string(), value: character.status.job})).expect("Fail to send persistence notification");
        self.change_sprite(character, LookType::Job, character.status.job as u16, 0);
    }

    pub fn load_units_in_fov(&self, server: &Server, character: &mut Character, map_instance_state: &MapInstanceState) {
        let mut new_map_view: HashSet<MapItem> = HashSet::with_capacity(2048);
        for (_, item) in map_instance_state.map_items().iter() {
            if let Some(position) = server.state().map_item_x_y(item, character.current_map_name(), character.current_map_instance()) {
                if item.id() != character.char_id && manhattan_distance(character.x(), character.y(), position.x, position.y) <= PLAYER_FOV {
                    // info!("seeing {}", item.object_type());
                    new_map_view.insert(*item);
                }
            }
        }

        for map_item in new_map_view.iter() {
            if !character.map_view.contains(map_item) {
                let default_name = "unknown".to_string();
                let map_item_name = server.state().map_item_name(map_item, character.current_map_name(), character.current_map_instance()).unwrap_or(default_name);
                let position = server.state().map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()). unwrap();
                debug!("See map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                let mut name = [0 as char; 24];
                map_item_name.fill_char_array(name.as_mut());
                let mut packet_zc_notify_standentry = PacketZcNotifyStandentry7::new();
                packet_zc_notify_standentry.set_job(map_item.client_item_class());
                packet_zc_notify_standentry.set_packet_length(PacketZcNotifyStandentry7::base_len(server.packetver()) as i16);
                // packet_zc_notify_standentry.set_name(name);
                packet_zc_notify_standentry.set_pos_dir(position.to_pos());
                packet_zc_notify_standentry.set_objecttype(map_item.object_type_value() as u8);
                packet_zc_notify_standentry.set_aid(map_item.id());
                packet_zc_notify_standentry.set_gid(map_item.id());
                match *map_item.object_type() {
                    MapItemType::Character => {}
                    MapItemType::Mob => {
                        if let Some(mob) = map_instance_state.get_mob(map_item.id()) {
                            packet_zc_notify_standentry.set_clevel(3);
                            packet_zc_notify_standentry.set_speed(mob.status.speed as i16);
                            packet_zc_notify_standentry.set_hp(mob.status.hp);
                            packet_zc_notify_standentry.set_max_hp(mob.status.max_hp);
                        }
                    }
                    MapItemType::Warp => {}
                    MapItemType::Unknown => {}
                    MapItemType::Npc => {}
                    MapItemType::DroppedItem => {}
                }
                packet_zc_notify_standentry.fill_raw_with_packetver(Some(server.packetver()));
                self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_notify_standentry.raw_mut())))).expect("Failed to send notification to client");
            }
        }

        for map_item in character.map_view.iter() {
            if !new_map_view.contains(map_item) {
                if let Some(position) = server.state().map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()) {
                    debug!("Vanish map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                    let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                    packet_zc_notify_vanish.set_gid(map_item.id());
                    packet_zc_notify_vanish.fill_raw();
                    self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_notify_vanish.raw_mut())))).expect("Failed to send notification to client");
                }
            }
        }
        character.map_view = new_map_view;
    }


}