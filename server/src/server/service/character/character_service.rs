use std::collections::HashSet;
use std::{io};
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

use packets::packets::{Packet, PacketZcAttackRange, PacketZcItemDisappear, PacketZcItemEntry, PacketZcLongparChange, PacketZcNotifyStandentry7, PacketZcNotifyVanish, PacketZcNpcackMapmove, PacketZcParChange, PacketZcSpriteChange2, PacketZcStatusValues};
use crate::repository::model::item_model::InventoryItemModel;
use crate::repository::{CharacterRepository};
use crate::server::model::events::game_event::{CharacterLook, CharacterZeny, GameEvent};

use crate::server::model::map_item::{MapItem, MapItemType};
use crate::server::model::path::manhattan_distance;

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::persistence_event::{PersistenceEvent, SavePositionUpdate, StatusUpdate};
use crate::server::model::events::persistence_event::PersistenceEvent::SaveCharacterPosition;
use crate::server::{PLAYER_FOV, Server};
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::position::Position;
use crate::server::model::tasks_queue::TasksQueue;

use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;


use crate::server::state::character::Character;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::server::ServerState;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;

static mut SERVICE_INSTANCE: Option<CharacterService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct CharacterService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<dyn CharacterRepository + Sync>,
    configuration_service: &'static GlobalConfigService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

impl CharacterService {
    pub fn instance() -> &'static CharacterService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<dyn CharacterRepository + Sync>, configuration_service: &'static GlobalConfigService, server_task_queue: Arc<TasksQueue<GameEvent>>) -> Self {
        Self { client_notification_sender, persistence_event_sender, repository, configuration_service, server_task_queue }
    }
    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<dyn CharacterRepository + Sync>, configuration_service: &'static GlobalConfigService, server_task_queue: Arc<TasksQueue<GameEvent>>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(CharacterService { client_notification_sender, persistence_event_sender, repository, configuration_service, server_task_queue });
        });
    }

    pub fn max_weight(&self, character: &Character) -> u32 {
        let base_weight = self.configuration_service.get_job_config(character.status.job).base_weight();
        base_weight + (character.status.str * 300) as u32
    }

    pub fn can_carry_weight(&self, character: &Character, additional_weight: u32) -> bool {
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
        type PredicateClosure = Box<dyn Fn(&(usize, &InventoryItemModel)) -> bool>;
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

    pub fn change_map(&self, new_map_instance_key: &MapInstanceKey, new_position: Position, character: &mut Character) {
        character.set_current_map_with_key(new_map_instance_key.clone());
        character.movements = vec![];
        let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

        let mut new_current_map: [char; 16] = [0 as char; 16];
        new_map_instance_key.map_name().fill_char_array(new_current_map.as_mut());
        packet_zc_npcack_mapmove.set_map_name(new_current_map);
        packet_zc_npcack_mapmove.set_x_pos(new_position.x as i16);
        packet_zc_npcack_mapmove.set_y_pos(new_position.y as i16);
        packet_zc_npcack_mapmove.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_npcack_mapmove.raw_mut()))))
            .expect("Failed to send notification event with PacketZcNpcackMapmove");

        character.update_position(new_position.x, new_position.y);
        character.clear_map_view();
        character.loaded_from_client_side = false;
        self.persistence_event_sender.send(SaveCharacterPosition(SavePositionUpdate { account_id: character.account_id, char_id: character.char_id, map_name: new_map_instance_key.map_name().clone(), x: character.x(), y: character.y() }))
            .expect("Fail to send persistence notification");
    }

    pub fn change_look(&self, character_look: CharacterLook, character: &mut Character) {
        let db_column = character.change_look(character_look.look_type, character_look.look_value);
        if let Some(db_column) = db_column {
            self.change_sprite(character, character_look.look_type, character_look.look_value, 0);
            self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character_look.char_id, db_column, value: character_look.look_value as u32 })).expect("Fail to send persistence notification");
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
            packet: packets,
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

    pub fn notify_weight(&self, character: &Character) {
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, self.weight_update_packets(character)))).expect("Fail to send client notification");
    }

    fn weight_update_packets(&self, character: &Character) -> Vec<u8> {
        let mut packet_weight = PacketZcParChange::new();
        packet_weight.set_var_id(StatusTypes::Weight.value() as u16);
        packet_weight.set_count(character.weight() as i32);
        packet_weight.fill_raw();
        let mut packet_max_weight = PacketZcParChange::new();
        packet_max_weight.set_var_id(StatusTypes::Maxweight.value() as u16);
        packet_max_weight.set_count(self.max_weight(character) as i32);
        packet_max_weight.fill_raw();
        chain_packets(vec![&packet_weight, &packet_max_weight])
    }

    pub fn update_base_level(&self, character: &mut Character, maybe_new_base_level: Option<u32>, maybe_level_delta: Option<i32>) -> i32 {
        let old_base_level = character.status.base_level;
        let new_base_level = if let Some(new_base_level) = maybe_new_base_level {
            new_base_level.min(self.configuration_service.config().game.max_base_level).max(1)
        } else if let Some(add_level) = maybe_level_delta {
            ((old_base_level as i32 + add_level).min(self.configuration_service.config().game.max_base_level as i32).max(1)) as u32
        } else {
            old_base_level
        };
        character.status.base_level = new_base_level;
        self.update_status_point(character, (character.status.status_point as i32 + self.calculate_status_point_delta(old_base_level, new_base_level)).max(0) as u32);
        if self.should_reset_stats(character) {
            self.reset_stats(character);
        }
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Baselevel, new_base_level);
        new_base_level as i32 - old_base_level as i32
    }

    pub fn update_job_level(&self, character: &mut Character, maybe_new_base_level: Option<u32>, maybe_level_delta: Option<i32>) -> i32 {
        let old_job_level = character.status.job_level;
        let new_job_level = if let Some(new_job_level) = maybe_new_base_level {
            new_job_level.min(self.configuration_service.config().game.max_job_level).max(1)
        } else if let Some(add_level) = maybe_level_delta {
            ((old_job_level as i32 + add_level).min(self.configuration_service.config().game.max_job_level as i32).max(1)) as u32
        } else {
            old_job_level
        };
        character.status.job_level = new_job_level;
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Joblevel, new_job_level);
        new_job_level as i32 - old_job_level as i32
    }

    pub fn change_job(&self, character: &mut Character, job: JobName) {
        character.status.job = job.value() as u32;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "class".to_string(), value: character.status.job })).expect("Fail to send persistence notification");
        self.change_sprite(character, LookType::Job, character.status.job as u16, 0);
    }

    pub fn get_status_point_count_for_level(&self, character: &Character) -> u32 {
        let status_point_count: u32 = if JobName::from_value(character.status.job as usize).is_rebirth() {
            100
        } else {
            48
        };
        status_point_count + self.calculate_status_point_delta(1, character.status.base_level) as u32
    }

    pub fn get_spent_status_point(&self, character: &Character) -> u32 {
        let mut status_point_count: u32 = 0;
        status_point_count += self.stat_raising_cost(character.status.str, "str");
        status_point_count += self.stat_raising_cost(character.status.dex, "dex");
        status_point_count += self.stat_raising_cost(character.status.agi, "agi");
        status_point_count += self.stat_raising_cost(character.status.int, "int");
        status_point_count += self.stat_raising_cost(character.status.vit, "vit");
        status_point_count += self.stat_raising_cost(character.status.luk, "luk");
        status_point_count
    }

    pub fn stat_raising_cost(&self, stat: u16, stat_name: &str) -> u32 {
        let mut status_point_count: u32 = 0;
        for i in 2..=stat {
            status_point_count += self.stat_raising_cost_for_next_level(i, stat_name)
        }
        status_point_count
    }

    fn stat_raising_cost_for_next_level(&self, level: u16, stat_name: &str) -> u32 {
        self.configuration_service.config().game.status_point_raising_cost.iter().find(|status_point_raising_cost| status_point_raising_cost.level_min <= level && level <= status_point_raising_cost.level_max)
            .map(|status_point_raising_cost| {
                debug!("{} in range {}..{} cost {}", level, status_point_raising_cost.level_min, status_point_raising_cost.level_max, status_point_raising_cost.raising_cost);
                status_point_raising_cost.raising_cost as u32
            }).unwrap_or_else(|| {
            warn!("No status point cost defined for {} level {}", level, stat_name);
            1000
        })
    }

    pub fn update_status_point(&self, character: &mut Character, status_point: u32) {
        character.status.status_point = status_point;
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Statuspoint, status_point);
        self.server_task_queue.add_to_first_index(GameEvent::CharacterCalculateStats(character.char_id));
    }

    pub fn stat_value(&self, character: &Character, status_type: StatusTypes) -> u16 {
        match status_type {
            StatusTypes::Str => {
                character.status.str
            }
            StatusTypes::Agi => {
                character.status.agi
            }
            StatusTypes::Vit => {
                character.status.vit
            }
            StatusTypes::Int => {
                character.status.int
            }
            StatusTypes::Dex => {
                character.status.dex
            }
            StatusTypes::Luk => {
                character.status.luk
            }
            _ => {
                error!("Can't read stat of type {:?}, not handled yet!", status_type);
               0
            }
        }
    }

    pub fn increase_stat(&self, character: &mut Character, status_type: StatusTypes, value_to_add: u16) -> bool {
        let mut null_stat = 0;
        let stat = match status_type {
            StatusTypes::Str => {
                &mut character.status.str
            }
            StatusTypes::Agi => {
                &mut character.status.agi
            }
            StatusTypes::Vit => {
                &mut character.status.vit
            }
            StatusTypes::Int => {
                &mut character.status.int
            }
            StatusTypes::Dex => {
                &mut character.status.dex
            }
            StatusTypes::Luk => {
                &mut character.status.luk
            }
            _ => {
                error!("Can't update stat of type {:?}, not handled yet!", status_type);
                &mut null_stat
            }
        };
        if *stat + value_to_add > self.configuration_service.config().game.max_stat_level {
            return false;
        }
        // With this calculation method, when value_to_add is > 1, like 10 and there is in theory enough status point available to add 3 points to the stat,
        // stat won't be updated at all
        let mut raising_cost = 0;
        for i in 1..=value_to_add {
            raising_cost += self.stat_raising_cost_for_next_level(*stat + i - 1, format!("{status_type:?}").as_str());
        }
        if character.status.status_point < raising_cost {
            return false;
        }
        *stat += value_to_add;
        character.status.status_point -= raising_cost;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate {
            char_id: character.char_id,
            db_column: status_type.to_column().unwrap_or_else(|| panic!("no db column name for status of type {status_type:?}")).to_string(),
            value: *stat as u32,
        })).expect("Fail to send persistence notification");
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "status_point".to_string(), value: character.status.status_point })).expect("Fail to send persistence notification");
        self.server_task_queue.add_to_first_index(GameEvent::CharacterCalculateStats(character.char_id));
        true
    }

    pub fn gain_exp(&self, character: &mut Character, exp: u32) {
        character.status.base_exp += exp;
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Baseexp, character.status.base_exp);
    }
    pub fn gain_job_exp(&self, character: &mut Character, exp: u32) {
        character.status.job_exp += exp;
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Jobexp, character.status.job_exp);
    }

    pub fn calculate_status_point_delta(&self, from_level: u32, to_level: u32) -> i32 {
        let mut status_point_count: i32 = 0;
        let (start, end, multiplier) = if from_level > to_level {
            (to_level, from_level, -1)
        } else {
            (from_level, to_level, 1)
        };
        for i in start..end {
            status_point_count += self.configuration_service.config().game.status_point_rewards.iter().find(|status_point_reward| status_point_reward.level_min as u32 <= i && i <= status_point_reward.level_max as u32)
                .map(|status_point_reward| {
                    debug!("{} in range {}..{} give reward {}", i, status_point_reward.level_min, status_point_reward.level_max, status_point_reward.reward);
                    status_point_reward.reward as i32
                }).unwrap_or_else(|| {
                warn!("No status point reward defined for level {}", i);
                0
            });
        }
        status_point_count * multiplier
    }

    pub fn reset_stats(&self, character: &mut Character) {
        character.status.str = 1;
        character.status.dex = 1;
        character.status.agi = 1;
        character.status.vit = 1;
        character.status.int = 1;
        character.status.luk = 1;
        self.update_status_point(character, self.get_status_point_count_for_level(character) - self.get_spent_status_point(character));
        for (column_name, _) in &[("str", StatusTypes::Str.value() as u16), ("agi", StatusTypes::Agi.value() as u16), ("dex", StatusTypes::Dex.value() as u16), ("vit", StatusTypes::Vit.value() as u16), ("int", StatusTypes::Int.value() as u16), ("luk", StatusTypes::Luk.value() as u16)] {
            self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: column_name.to_string(), value: 1 })).expect("Fail to send persistence notification");
        }
    }

    pub fn should_reset_stats(&self, character: &Character) -> bool {
        self.get_spent_status_point(character) > self.get_status_point_count_for_level(character)
    }

    pub fn calculate_status(&self, server_ref: &Server, character: &Character) {
        let mut packet_str = PacketZcStatusValues::new();
        packet_str.set_status_type(StatusTypes::Str.value() as u32);
        packet_str.set_default_status(character.status.str as i32);
        packet_str.fill_raw();
        let mut packet_agi = PacketZcStatusValues::new();
        packet_agi.set_status_type(StatusTypes::Agi.value() as u32);
        packet_agi.set_default_status(character.status.agi as i32);
        packet_agi.fill_raw();
        let mut packet_dex = PacketZcStatusValues::new();
        packet_dex.set_status_type(StatusTypes::Dex.value() as u32);
        packet_dex.set_default_status(character.status.dex as i32);
        packet_dex.fill_raw();
        let mut packet_int = PacketZcStatusValues::new();
        packet_int.set_status_type(StatusTypes::Int.value() as u32);
        packet_int.set_default_status(character.status.int as i32);
        packet_int.fill_raw();
        let mut packet_vit = PacketZcStatusValues::new();
        packet_vit.set_status_type(StatusTypes::Vit.value() as u32);
        packet_vit.set_default_status(character.status.vit as i32);
        packet_vit.fill_raw();
        let mut packet_luk = PacketZcStatusValues::new();
        packet_luk.set_status_type(StatusTypes::Luk.value() as u32);
        packet_luk.set_default_status(character.status.luk as i32);
        packet_luk.fill_raw();
        let mut packet_str_increase_cost = PacketZcParChange::new();
        packet_str_increase_cost.set_var_id(StatusTypes::StrNextLevelIncreaseCost.value() as u16);
        packet_str_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.str, "str") as i32);
        packet_str_increase_cost.fill_raw();
        let mut packet_agi_increase_cost = PacketZcParChange::new();
        packet_agi_increase_cost.set_var_id(StatusTypes::AgiNextLevelIncreaseCost.value() as u16);
        packet_agi_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.agi, "agi") as i32);
        packet_agi_increase_cost.fill_raw();
        let mut packet_dex_increase_cost = PacketZcParChange::new();
        packet_dex_increase_cost.set_var_id(StatusTypes::DexNextLevelIncreaseCost.value() as u16);
        packet_dex_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.dex, "dex") as i32);
        packet_dex_increase_cost.fill_raw();
        let mut packet_vit_increase_cost = PacketZcParChange::new();
        packet_vit_increase_cost.set_var_id(StatusTypes::VitNextLevelIncreaseCost.value() as u16);
        packet_vit_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.vit, "vit") as i32);
        packet_vit_increase_cost.fill_raw();
        let mut packet_int_increase_cost = PacketZcParChange::new();
        packet_int_increase_cost.set_var_id(StatusTypes::IntNextLevelIncreaseCost.value() as u16);
        packet_int_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.int, "int") as i32);
        packet_int_increase_cost.fill_raw();
        let mut packet_luk_increase_cost = PacketZcParChange::new();
        packet_luk_increase_cost.set_var_id(StatusTypes::LukNextLevelIncreaseCost.value() as u16);
        packet_luk_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.luk, "luk") as i32);
        packet_luk_increase_cost.fill_raw();
        let mut packet_status_point = PacketZcParChange::new();
        packet_status_point.set_var_id(StatusTypes::Statuspoint.value() as u16);
        packet_status_point.set_count(character.status.status_point as i32);
        packet_status_point.fill_raw();

        let mut packet_hit = PacketZcParChange::new();
        packet_hit.set_var_id(StatusTypes::Hit.value() as u16);
        packet_hit.set_count(character.status.hit as i32);
        packet_hit.fill_raw();
        let mut packet_flee = PacketZcParChange::new();
        packet_flee.set_var_id(StatusTypes::Flee1.value() as u16);
        packet_flee.set_count(character.status.flee as i32);
        packet_flee.fill_raw();
        let mut packet_aspd = PacketZcParChange::new();
        packet_aspd.set_var_id(StatusTypes::Aspd.value() as u16);
        let aspd = StatusService::instance().aspd(character);
        packet_aspd.set_count(StatusService::instance().client_aspd(aspd));
        packet_aspd.fill_raw();
        let mut packet_atk = PacketZcParChange::new();
        packet_atk.set_var_id(StatusTypes::Atk1.value() as u16);
        packet_atk.set_count(StatusService::instance().status_atk_left_side(character));
        packet_atk.fill_raw();
        let mut packet_atk2 = PacketZcParChange::new();
        packet_atk2.set_var_id(StatusTypes::Atk2.value() as u16);
        packet_atk2.set_count(StatusService::instance().status_atk_right_side(character));
        packet_atk2.fill_raw();
        let mut packet_def = PacketZcParChange::new();
        packet_def.set_var_id(StatusTypes::Def1.value() as u16);
        packet_def.set_count(character.status.def as i32);
        packet_def.fill_raw();
        let mut packet_flee2 = PacketZcParChange::new();
        packet_flee2.set_var_id(StatusTypes::Flee2.value() as u16);
        packet_flee2.set_count(character.status.flee as i32);
        packet_flee2.fill_raw();
        let mut packet_crit = PacketZcParChange::new();
        packet_crit.set_var_id(StatusTypes::Critical.value() as u16);
        packet_crit.set_count(character.status.crit as i32);
        packet_crit.fill_raw();
        let mut packet_matk = PacketZcParChange::new();
        packet_matk.set_var_id(StatusTypes::Matk1.value() as u16);
        packet_matk.set_count(character.status.matk_min as i32);
        packet_matk.fill_raw();
        let mut packet_matk2 = PacketZcParChange::new();
        packet_matk2.set_var_id(StatusTypes::Matk2.value() as u16);
        packet_matk2.set_count(character.status.matk_max as i32);
        packet_matk2.fill_raw();
        let mut packet_mdef2 = PacketZcParChange::new();
        packet_mdef2.set_var_id(StatusTypes::Mdef2.value() as u16);
        packet_mdef2.set_count(character.status.mdef as i32);
        packet_mdef2.fill_raw();
        let mut packet_attack_range = PacketZcAttackRange::new();
        packet_attack_range.set_current_att_range(1);
        packet_attack_range.fill_raw();
        let mut packet_maxhp = PacketZcParChange::new();
        packet_maxhp.set_var_id(StatusTypes::Maxhp.value() as u16);
        packet_maxhp.set_count(character.status.max_hp as i32);
        packet_maxhp.fill_raw();
        let mut packet_maxsp = PacketZcParChange::new();
        packet_maxsp.set_var_id(StatusTypes::Maxsp.value() as u16);
        packet_maxsp.set_count(character.status.max_sp as i32);
        packet_maxsp.fill_raw();
        let mut packet_hp = PacketZcParChange::new();
        packet_hp.set_var_id(StatusTypes::Hp.value() as u16);
        packet_hp.set_count(character.status.hp as i32);
        packet_hp.fill_raw();
        let mut packet_sp = PacketZcParChange::new();
        packet_sp.set_var_id(StatusTypes::Sp.value() as u16);
        packet_sp.set_count(character.status.sp as i32);
        packet_sp.fill_raw();
        let mut packet_speed = PacketZcParChange::new();
        packet_speed.set_var_id(StatusTypes::Speed.value() as u16);
        packet_speed.set_count(character.status.speed as i32);
        packet_speed.fill_raw();

        let mut final_response_packet: Vec<u8> = chain_packets(vec![
            &packet_str, &packet_agi, &packet_dex, &packet_int, &packet_luk, &packet_vit,
            &packet_status_point, &packet_str_increase_cost, &packet_agi_increase_cost, &packet_vit_increase_cost,
            &packet_dex_increase_cost, &packet_int_increase_cost, &packet_luk_increase_cost,
            &packet_hit, &packet_flee, &packet_aspd, &packet_atk, &packet_atk2, &packet_def,
            &packet_flee2, &packet_crit, &packet_matk, &packet_matk2,
            &packet_mdef2, &packet_attack_range, &packet_maxhp, &packet_maxsp, &packet_hp,
            &packet_sp, &packet_speed,
        ]);
        final_response_packet.extend(self.weight_update_packets(character));
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, final_response_packet)))
            .expect("Fail to send client notification");
    }

    pub fn load_units_in_fov(&self, server_state: &ServerState, character: &mut Character, map_instance_state: &MapInstanceState) {
        let mut new_map_view: HashSet<MapItem> = HashSet::with_capacity(2048);
        for (_, item) in map_instance_state.map_items().iter() {
            if let Some(position) = server_state.map_item_x_y(item, character.current_map_name(), character.current_map_instance()) {
                if item.id() != character.char_id && manhattan_distance(character.x(), character.y(), position.x, position.y) <= PLAYER_FOV {
                    // info!("seeing {}", item.object_type());
                    new_map_view.insert(*item);
                }
            }
        }

        let mut packets = vec![];
        for map_item in new_map_view.iter() {
            if !character.map_view.contains(map_item) {
                let default_name = "unknown".to_string();
                let map_item_name = server_state.map_item_name(map_item, character.current_map_name(), character.current_map_instance()).unwrap_or(default_name);
                let position = server_state.map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()).unwrap();
                debug!("See map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                let mut name = [0 as char; 24];
                map_item_name.fill_char_array(name.as_mut());
                if matches!(map_item.object_type(), MapItemType::DroppedItem) {
                    if let Some(item) = map_instance_state.get_dropped_item(map_item.id()) {
                        let mut packet_zc_item_entry = PacketZcItemEntry::default();
                        packet_zc_item_entry.set_itid(item.item_id as u16);
                        packet_zc_item_entry.set_itaid(item.map_item_id);
                        packet_zc_item_entry.set_x_pos(item.location.x as i16);
                        packet_zc_item_entry.set_y_pos(item.location.y as i16);
                        packet_zc_item_entry.set_sub_x(item.sub_location.x as u8);
                        packet_zc_item_entry.set_sub_y(item.sub_location.y as u8);
                        packet_zc_item_entry.set_count(item.amount as i16);
                        packet_zc_item_entry.fill_raw();
                        packets.extend(packet_zc_item_entry.raw);
                    }
                } else {
                    let mut packet_zc_notify_standentry = PacketZcNotifyStandentry7::new();
                    packet_zc_notify_standentry.set_job(map_item.client_item_class());
                    packet_zc_notify_standentry.set_packet_length(PacketZcNotifyStandentry7::base_len(self.configuration_service.packetver()) as i16);
                    // packet_zc_notify_standentry.set_name(name);
                    packet_zc_notify_standentry.set_pos_dir(position.to_pos());
                    packet_zc_notify_standentry.set_objecttype(map_item.object_type_value() as u8);
                    packet_zc_notify_standentry.set_aid(map_item.id());
                    packet_zc_notify_standentry.set_gid(map_item.id());
                    if matches!(map_item.object_type(), MapItemType::Mob) {
                        if let Some(mob) = map_instance_state.get_mob(map_item.id()) {
                            packet_zc_notify_standentry.set_clevel(3);
                            packet_zc_notify_standentry.set_speed(mob.status.speed as i16);
                            packet_zc_notify_standentry.set_hp(mob.status.hp);
                            packet_zc_notify_standentry.set_max_hp(mob.status.max_hp);
                        }
                    }
                    packet_zc_notify_standentry.fill_raw_with_packetver(Some(self.configuration_service.packetver()));
                    packets.extend(packet_zc_notify_standentry.raw);
                }
            }
        }
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets))).expect("Failed to send notification to client");

        let mut packets = vec![];
        for map_item in character.map_view.iter() {
            if !new_map_view.contains(map_item) {
                if let Some(position) = server_state.map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()) {
                    debug!("Vanish map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                    if matches!(map_item.object_type(), MapItemType::DroppedItem) {
                        let mut packet_zc_item_disappear = PacketZcItemDisappear::new();
                        packet_zc_item_disappear.set_itaid(map_item.id());
                        packet_zc_item_disappear.fill_raw();
                        packets.extend(packet_zc_item_disappear.raw);
                    } else {
                        let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                        packet_zc_notify_vanish.set_gid(map_item.id());
                        packet_zc_notify_vanish.fill_raw();
                        packets.extend(packet_zc_notify_vanish.raw);
                    }
                }
            }
        }
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets))).expect("Failed to send notification to client");
        character.map_view = new_map_view;
    }

    fn send_status_update_and_defer_db_update(&self, char_id: u32, status_type: StatusTypes, new_value: u32) {
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id, db_column: status_type.to_column().unwrap_or_else(|| panic!("no db column name for status of type {status_type:?}")).to_string(), value: new_value })).expect("Fail to send persistence notification");
        let mut packet_base_level = PacketZcParChange::new();
        packet_base_level.set_var_id(status_type.value() as u16);
        packet_base_level.set_count(new_value as i32);
        packet_base_level.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(char_id, packet_base_level.raw))).expect("Fail to send client notification");
    }
}