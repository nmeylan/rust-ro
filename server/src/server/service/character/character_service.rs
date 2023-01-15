use std::collections::HashSet;
use std::mem;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Once};
use tokio::runtime::Runtime;
use enums::action::ActionType;
use enums::class::JobName;
use enums::look::LookType;
use enums::status::StatusTypes;
use crate::enums::EnumWithNumberValue;
use packets::packets::{Packet, PacketZcLongparChange, PacketZcNotifyAct, PacketZcNotifyStandentry7, PacketZcNotifyVanish, PacketZcNpcackMapmove, PacketZcParChange, PacketZcSpriteChange2};
use crate::repository::Repository;
use crate::server::events::game_event::{CharacterChangeMap, CharacterLook, CharacterRemoveFromMap, CharacterZeny, GameEvent};
use crate::server::core::map::{Map, MAP_EXT, RANDOM_CELL};
use crate::server::core::map_item::{MapItem, MapItemType};
use crate::server::core::path::manhattan_distance;
use crate::server::core::position::Position;
use crate::server::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::events::persistence_event::{PersistenceEvent, SavePositionUpdate, StatusUpdate};
use crate::server::events::persistence_event::PersistenceEvent::SaveCharacterPosition;
use crate::server::{PLAYER_FOV, Server};
use crate::server::core::configuration::Config;
use crate::server::core::map_instance::MapInstance;

use crate::server::service::status_service::StatusService;

use crate::server::state::character::Character;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;

static mut SERVICE_INSTANCE: Option<CharacterService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct CharacterService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<Repository>,
    configuration: &'static Config
}

impl CharacterService {
    pub fn instance() -> &'static CharacterService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<Repository>, configuration: &'static Config) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(CharacterService{ client_notification_sender, persistence_event_sender, repository, configuration });
        });
    }

    pub fn change_map(&self, map_instance: Arc<MapInstance>, event: &CharacterChangeMap, character: &mut Character) {
        character.join_and_set_map(map_instance);
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
            range_type: AreaNotificationRangeType::Fov { x: character.x(), y: character.y() },
            packet: packets
        })).expect("Fail to send client notification");
    }

    pub async fn update_zeny(&self, runtime: &Runtime, zeny_update: CharacterZeny, character: &mut Character) {
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
        packet_max_weight.set_count(character.max_weight() as i32);
        packet_max_weight.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, chain_packets(vec![&packet_weight, &packet_max_weight])))).expect("Fail to send client notification");
    }

    pub fn update_base_level(&self, character: &mut Character, maybe_new_base_level: Option<u32>, maybe_level_delta: Option<i32>) -> i32 {
        let old_base_level = character.status.base_level;
        let new_base_level = if let Some(new_base_level) = maybe_new_base_level {
            new_base_level.min(self.configuration.game.max_base_level).max(1) as u32
        } else if let Some(add_level) = maybe_level_delta {
            ((old_base_level as i32 + add_level).min(self.configuration.game.max_base_level as i32).max(1)) as u32
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
            new_job_level.min(self.configuration.game.max_job_level).max(1) as u32
        } else if let Some(add_level) = maybe_level_delta {
            ((old_job_level as i32 + add_level).min(self.configuration.game.max_job_level as i32).max(1)) as u32
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

    pub fn load_units_in_fov(&self, server: &Server, character: &mut Character) {
        let map_instance = server.get_map_instance(character.current_map_name(), character.current_map_instance());
        if map_instance.is_none() {
            return;
        }
        let map_instance = map_instance.unwrap();
        let mut new_map_view: HashSet<MapItem> = HashSet::with_capacity(2048);
        for (_, item) in map_instance.map_items.borrow().iter() {
            if let Some(position) = server.map_item_x_y(item, character.current_map_name(), character.current_map_instance()) {
                if item.id() != character.char_id && manhattan_distance(character.x(), character.y(), position.x, position.y) <= PLAYER_FOV {
                    // info!("seeing {}", item.object_type());
                    new_map_view.insert(*item);
                }
            }
        }

        for map_item in new_map_view.iter() {
            if !character.map_view.contains(map_item) {
                let default_name = "unknown".to_string();
                let map_item_name = server.map_item_name(map_item, character.current_map_name(), character.current_map_instance()).unwrap_or(default_name);
                let position = server.map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()). unwrap();
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
                        if let Some(mob) = map_instance.get_mob(map_item.id()) {
                            packet_zc_notify_standentry.set_clevel(3);
                            packet_zc_notify_standentry.set_speed(mob.status.speed as i16);
                            packet_zc_notify_standentry.set_hp(mob.status.hp);
                            packet_zc_notify_standentry.set_max_hp(mob.status.max_hp);
                        }
                    }
                    MapItemType::Warp => {}
                    MapItemType::Unknown => {}
                    MapItemType::Npc => {}
                }
                packet_zc_notify_standentry.fill_raw_with_packetver(Some(server.packetver()));
                self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_notify_standentry.raw_mut())))).expect("Failed to send notification to client");
            }
        }

        for map_item in character.map_view.iter() {
            if !new_map_view.contains(map_item) {
                let position = server.map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()).unwrap();
                debug!("Vanish map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(map_item.id());
                packet_zc_notify_vanish.fill_raw();
                self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_notify_vanish.raw_mut())))).expect("Failed to send notification to client");
            }
        }
        character.map_view = new_map_view;
    }

    pub fn attack(&self, server: &Server, character: &mut Character, tick: u128) {
        if !character.is_attacking() {
            return;
        }
        let attack = character.attack();
        if !attack.repeat { // one shot attack
            character.clear_attack();
        }
        let attack_motion = StatusService::instance().attack_motion(character);

        if tick < attack.last_attack_tick + attack_motion as u128 {
            return;
        }
        let map_item = server.map_item(attack.target, character.current_map_name(), character.current_map_instance());
        if let Some(map_item) = map_item {
            let _position = server.map_item_x_y(&map_item, character.current_map_name(), character.current_map_instance()).unwrap();
            // TODO: Check distance based on weapon range, handle too far target
            character.update_last_attack_tick(tick);
            character.update_last_attack_motion(attack_motion);
            let mut packet_zc_notify_act3 = PacketZcNotifyAct::new();
            packet_zc_notify_act3.set_target_gid(attack.target);
            packet_zc_notify_act3.set_action(ActionType::Attack.value() as u8);
            packet_zc_notify_act3.set_gid(character.char_id);
            packet_zc_notify_act3.set_attack_mt(attack_motion as i32);
            packet_zc_notify_act3.set_attacked_mt(attack_motion as i32);
            packet_zc_notify_act3.set_damage(2);
            packet_zc_notify_act3.set_count(1);
            packet_zc_notify_act3.fill_raw();
            self.client_notification_sender.send(Notification::Area(AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov {x: character.x, y: character.y}, mem::take(packet_zc_notify_act3.raw_mut())))).expect("Failed to send notification to client");
        } else {
            error!("Attack target {} not found", attack.target);
        }
    }
}