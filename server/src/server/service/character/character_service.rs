use std::collections::HashSet;
use std::{io};
use std::io::Write;
use std::sync::mpsc::SyncSender;
use std::sync::{Arc, Once};
use tokio::runtime::Runtime;

use models::enums::class::{JOB_BASE_MASK, JobName};
use models::enums::effect::Effect;

use models::enums::look::LookType;
use models::enums::status::StatusTypes;

use models::enums::EnumWithNumberValue;
use models::enums::skill_enums::SkillEnum;


use packets::packets::{Packet, PacketZcAttackRange, PacketZcItemDisappear, PacketZcItemEntry, PacketZcLongparChange, PacketZcNotifyEffect, PacketZcNotifyStandentry7, PacketZcNotifyVanish, PacketZcNpcackMapmove, PacketZcParChange, PacketZcSpriteChange2, PacketZcStatusChangeAck, PacketZcStatusValues, PacketZcNotifyMove};
use crate::repository::model::item_model::InventoryItemModel;
use crate::repository::{CharacterRepository};
use crate::server::model::events::game_event::{CharacterKillMonster, CharacterLook, CharacterUpdateStat, CharacterZeny, GameEvent};

use crate::server::model::map_item::{MapItem, MapItemType};
use crate::server::model::path::manhattan_distance;

use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::persistence_event::{IncreaseSkillLevel, PersistenceEvent, ResetSkills, SavePositionUpdate, StatusUpdate};
use crate::server::model::events::persistence_event::PersistenceEvent::SaveCharacterPosition;
use crate::server::{PLAYER_FOV, Server};
use crate::server::model::events::map_event::{MapEvent, MobDropItems};
use crate::server::model::map_instance::{MapInstance, MapInstanceKey};
use models::position::Position;
use models::status::{KnownSkill, Status};
use crate::server::model::movement::Movable;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::skill_tree_service::SkillTreeService;

use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;


use crate::server::state::character::Character;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::server::ServerState;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;
use crate::util::tick::get_tick_client;

static mut SERVICE_INSTANCE: Option<CharacterService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct CharacterService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    repository: Arc<dyn CharacterRepository + Sync>,
    configuration_service: &'static GlobalConfigService,
    skill_tree_service: SkillTreeService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    status_service: &'static StatusService,
}

impl CharacterService {
    pub fn instance() -> &'static CharacterService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<dyn CharacterRepository + Sync>, configuration_service: &'static GlobalConfigService, skill_tree_service: SkillTreeService, status_service: &'static StatusService, server_task_queue: Arc<TasksQueue<GameEvent>>) -> Self {
        Self { client_notification_sender, persistence_event_sender, repository, configuration_service, skill_tree_service, server_task_queue, status_service }
    }
    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, repository: Arc<dyn CharacterRepository + Sync>, configuration_service: &'static GlobalConfigService, skill_tree_service: SkillTreeService, status_service: &'static StatusService, server_task_queue: Arc<TasksQueue<GameEvent>>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(CharacterService { client_notification_sender, persistence_event_sender, repository, configuration_service, skill_tree_service, server_task_queue, status_service });
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
        let status_snapshot = self.status_service.to_snapshot(&character.status);
        let mut stdout = io::stdout();
        writeln!(stdout, "************** {} - {} ****************", character.name, character.char_id).unwrap();
        writeln!(stdout, "Status:").unwrap();
        writeln!(stdout, "  str: {}", status_snapshot.str()).unwrap();
        writeln!(stdout, "  agi: {}", status_snapshot.agi()).unwrap();
        writeln!(stdout, "  vit: {}", status_snapshot.vit()).unwrap();
        writeln!(stdout, "  int: {}", status_snapshot.int()).unwrap();
        writeln!(stdout, "  dex: {}", status_snapshot.dex()).unwrap();
        writeln!(stdout, "  luk: {}", status_snapshot.luk()).unwrap();
        writeln!(stdout, "  speed: {}", status_snapshot.speed()).unwrap();
        writeln!(stdout, "  hp: {}/{}", status_snapshot.hp(), status_snapshot.max_hp()).unwrap();
        writeln!(stdout, "  sp: {}/{}", status_snapshot.sp(), status_snapshot.max_sp()).unwrap();
        writeln!(stdout, "  zeny: {}", status_snapshot.zeny()).unwrap();
        writeln!(stdout, "  weight: {}/{}", character.weight(), self.max_weight(character)).unwrap();
        writeln!(stdout, "Inventory:").unwrap();
        type PredicateClosure = Box<dyn Fn(&(usize, &InventoryItemModel)) -> bool>;
        let mut inventory_print = |predicate: PredicateClosure| {
            character.inventory_iter()
                .filter(predicate)
                .for_each(|(index, item)| writeln!(stdout, " [{}] {} - {} ({})", index, item.name_english, item.item_id, item.amount).unwrap());
        };
        inventory_print(Box::new(|(_, item)| item.item_type().is_consumable()));
        inventory_print(Box::new(|(_, item)| item.item_type().is_equipment()));
        inventory_print(Box::new(|(_, item)| item.item_type().is_etc()));
        writeln!(stdout, "Equipped items:").unwrap();
        character.status.equipped_gears().iter().for_each(|item| writeln!(stdout, " [{}] - {}  at {:?}", item.inventory_index(), item.item_id(), item.location()).unwrap());
        writeln!(stdout, "Equipped weapon:").unwrap();
        character.status.equipped_weapons().iter().for_each(|item| writeln!(stdout, " [{}] - {} ({:?}) at {:?}", item.inventory_index(), item.item_id(), item.weapon_type(), item.location()).unwrap());
        stdout.flush().unwrap();
    }

    pub fn change_map(&self, new_map_instance_key: &MapInstanceKey, new_position: Position, character: &mut Character) {
        character.set_current_map_with_key(new_map_instance_key.clone());
        character.movements = vec![];
        let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new(self.configuration_service.packetver());

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
        let mut packet_zc_sprite_change = PacketZcSpriteChange2::new(self.configuration_service.packetver());
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

        let mut packet_zc_longpar_change = PacketZcLongparChange::new(self.configuration_service.packetver());
        packet_zc_longpar_change.set_amount(character.get_zeny() as i32);
        packet_zc_longpar_change.set_var_id(StatusTypes::Zeny.value() as u16);
        packet_zc_longpar_change.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, std::mem::take(packet_zc_longpar_change.raw_mut())))).expect("Fail to send client notification");
    }

    pub fn notify_weight(&self, character: &Character) {
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, self.weight_update_packets(character)))).expect("Fail to send client notification");
    }

    fn weight_update_packets(&self, character: &Character) -> Vec<u8> {
        let mut packet_weight = PacketZcParChange::new(self.configuration_service.packetver());
        packet_weight.set_var_id(StatusTypes::Weight.value() as u16);
        packet_weight.set_count(character.weight() as i32);
        packet_weight.fill_raw();
        let mut packet_max_weight = PacketZcParChange::new(self.configuration_service.packetver());
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
        let mut packet_zc_notify_effect = PacketZcNotifyEffect::new(self.configuration_service.packetver());
        packet_zc_notify_effect.set_effect_id(Effect::BaseLevelUp.value() as i32);
        packet_zc_notify_effect.set_aid(character.char_id);
        packet_zc_notify_effect.fill_raw();
        self.send_area_notification_around_characters(character, packet_zc_notify_effect.raw);
        new_base_level as i32 - old_base_level as i32
    }

    pub fn get_job_level_max(&self, character: &mut Character) -> u32 {
        self.configuration_service.get_job_config(character.status.job).job_level().max_job_level() as u32
    }

    pub fn update_job_level(&self, character: &mut Character, maybe_new_base_level: Option<u32>, maybe_level_delta: Option<i32>) -> i32 {
        let old_job_level = character.status.job_level;
        let new_job_level = if let Some(new_job_level) = maybe_new_base_level {
            new_job_level.min(self.get_job_level_max(character)).max(1)
        } else if let Some(add_level) = maybe_level_delta {
            ((old_job_level as i32 + add_level).min(self.get_job_level_max(character) as i32).max(1)) as u32
        } else {
            old_job_level
        };
        character.status.job_level = new_job_level;
        if new_job_level > old_job_level {
            self.update_skill_point(character, character.status.skill_point + new_job_level - old_job_level, true);
        } else if old_job_level > new_job_level {
            if self.should_reset_skills(character) {
                self.reset_skills(character, false);
            }
            self.update_skill_point(character, (character.status.skill_point as i32 - (old_job_level as i32 - new_job_level as i32)).max(0) as u32, true);
        }
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Joblevel, new_job_level);
        let mut packet_zc_notify_effect = PacketZcNotifyEffect::new(self.configuration_service.packetver());
        packet_zc_notify_effect.set_effect_id(Effect::JobLevelUp.value() as i32);
        packet_zc_notify_effect.set_aid(character.char_id);
        packet_zc_notify_effect.fill_raw();
        self.send_area_notification_around_characters(character, packet_zc_notify_effect.raw);
        new_job_level as i32 - old_job_level as i32
    }

    pub fn change_job(&self, character: &mut Character, job: JobName, should_reset_skills: bool) {
        character.status.job = job.value() as u32;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "class".to_string(), value: character.status.job })).expect("Fail to send persistence notification");
        self.change_sprite(character, LookType::Job, character.status.job as u16, 0);
        if should_reset_skills {
            let skill_point;
            if job.is_first_class() {
                skill_point = 9 + character.status.job_level - 1;
            } else if job.is_second_class() || job.is_rebirth() {
                skill_point = 9 + 49 + character.status.job_level - 1;
            } else {
                skill_point = character.status.job_level - 1;
            }
            self.reset_skills(character, false);
            self.update_skill_point(character, skill_point, true);
        }
    }

    pub fn get_allocated_skills_point(&self, character: &Character) -> u8 {
        let mut count = 0;
        character.status.known_skills.iter().for_each(|skill| {
            if !skill.value.is_platinium() {
                count += skill.level;
            }
        });
        count
    }

    pub fn get_status_point_count_for_level(&self, character: &Character) -> u32 {
        let status_point_count: u32 = if JobName::from_value(character.status.job as usize).is_rebirth() {
            100
        } else {
            48
        };
        status_point_count + self.calculate_status_point_delta(1, character.status.base_level) as u32
    }

    pub fn get_skill_point_count_for_level(&self, character: &Character) -> u8 {
        let job = JobName::from_value(character.status.job as usize);
        let mut skill_points = (character.status.job_level as i32 - 1).max(0) as u8;
        if job.is_second_class() {
            let first_class_job = JobName::from_mask(job.mask() & JOB_BASE_MASK, true).unwrap();
            let first_class_job_config = self.configuration_service.get_job_config(first_class_job.value() as u32);
            skill_points += first_class_job_config.job_level().max_job_level() - 1;
        }
        if !job.is_novice() {
            skill_points += 9;
        }
        skill_points
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

    fn stat_raising_cost(&self, stat: u16, stat_name: &str) -> u32 {
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
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Statuspoint, character.status.status_point);
        self.server_task_queue.add_to_first_index(GameEvent::CharacterUpdateClientSideStats(character.char_id));
    }
    pub fn update_skill_point(&self, character: &mut Character, skill_point: u32, should_persist: bool) {
        character.status.skill_point = skill_point;
        if should_persist {
            self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Skillpoint, character.status.skill_point);
        }
    }

    pub fn stat_value(&self, status: &Status, status_type: &StatusTypes) -> u16 {
        match status_type {
            StatusTypes::Str => {
                status.str
            }
            StatusTypes::Agi => {
                status.agi
            }
            StatusTypes::Vit => {
                status.vit
            }
            StatusTypes::Int => {
                status.int
            }
            StatusTypes::Dex => {
                status.dex
            }
            StatusTypes::Luk => {
                status.luk
            }
            _ => {
                error!("Can't read stat of type {:?}, not handled yet!", status_type);
                0
            }
        }
    }

    pub fn stat_mut<'a>(&'a self, status: &'a mut Status, status_type: &StatusTypes) -> &mut u16 {
        match status_type {
            StatusTypes::Str => {
                &mut status.str
            }
            StatusTypes::Agi => {
                &mut status.agi
            }
            StatusTypes::Vit => {
                &mut status.vit
            }
            StatusTypes::Int => {
                &mut status.int
            }
            StatusTypes::Dex => {
                &mut status.dex
            }
            StatusTypes::Luk => {
                &mut status.luk
            }
            _ => {
                panic!("Can't read stat of type {status_type:?}, not handled yet!");
            }
        }
    }

    pub fn allocate_status_point(&self, character: &mut Character, status_type: StatusTypes, value_to_add: u16) -> bool {
        let status_point = character.status.status_point;
        let stat = self.stat_mut(&mut character.status, &status_type);
        if *stat + value_to_add > self.configuration_service.config().game.max_stat_level {
            return false;
        }
        // With this calculation method, when value_to_add is > 1, like 10 and there is in theory enough status point available to add 3 points to the stat,
        // stat won't be updated at all
        let mut raising_cost = 0;
        for i in 1..=value_to_add {
            raising_cost += self.stat_raising_cost_for_next_level(*stat + i - 1, format!("{status_type:?}").as_str());
        }
        if status_point < raising_cost {
            return false;
        }
        *stat += value_to_add;
        let value = *stat as u32;
        character.status.status_point -= raising_cost;
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate {
            char_id: character.char_id,
            db_column: status_type.to_column().unwrap_or_else(|| panic!("no db column name for status of type {status_type:?}")).to_string(),
            value,
        })).expect("Fail to send persistence notification");
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "status_point".to_string(), value: character.status.status_point })).expect("Fail to send persistence notification");
        self.server_task_queue.add_to_first_index(GameEvent::CharacterUpdateClientSideStats(character.char_id));
        true
    }

    pub fn allocate_skill_point(&self, character: &mut Character, skill: SkillEnum) -> bool {
        let skill_point = character.status.skill_point;
        if skill_point < 1 {
            return false
        }
        // Skill to allocate point to is not available in skill tree
        if !self.skill_tree_service.skill_tree(character).iter().any(|s| s.value == skill) {
            return false
        }
        character.status.skill_point -= 1;
        let increased_skill;
        if let Some(skill) = character.status.known_skills.iter_mut().find(|s| s.value == skill) {
            skill.level += 1;
            increased_skill = *skill;
        } else {
            increased_skill = KnownSkill { value: skill, level: 1 };
            character.status.known_skills.push(increased_skill)
        }
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Skillpoint, character.status.skill_point);
        self.persistence_event_sender.send(PersistenceEvent::IncreaseSkillLevel(IncreaseSkillLevel { char_id: character.char_id as i32, skill: increased_skill.value, increment: 1, })).expect("Fail to send persistence notification");
        self.skill_tree_service.send_skill_tree(character);
        true
    }

    pub fn gain_base_exp(&self, character: &mut Character, gain_exp: u32) {
        let mut gained_level = 0;
        let mut gain_exp = (gain_exp as f32 * self.configuration_service.config().game.base_exp_rate).ceil() as u32;
        let mut status_copy = Status::default();
        status_copy.base_exp = character.status.base_exp;
        status_copy.base_level = character.status.base_level;
        loop {
            let next_level_requirement = self.next_base_level_required_exp(&status_copy);
            if next_level_requirement == u32::MAX {
                character.status.base_exp = 0;
                gain_exp = 0;
                break;
            }
            // Currently multi leveling is allowed, so if gained exp permit to level up...
            if status_copy.base_exp + gain_exp >= next_level_requirement {
                gained_level += 1;
                // ... we gain new level...
                status_copy.base_level += 1;
                // ... removing from gained exp, the amount spent to level up...
                gain_exp = status_copy.base_exp + gain_exp - next_level_requirement;
                status_copy.base_exp = 0;
            } else {
                // ... until there not enough exp point to reach new level
                break;
            }
        }
        if gained_level > 0 {
            self.update_base_level(character, None, Some(gained_level));
        }
        character.status.base_exp = status_copy.base_exp + gain_exp;
        self.send_status_update_and_defer_db_update(character.char_id, StatusTypes::Baseexp, character.status.base_exp);
    }

    pub fn gain_job_exp(&self, character: &mut Character, gain_exp: u32) {
        let mut gained_level = 0;
        let mut gain_exp = (gain_exp as f32 * self.configuration_service.config().game.job_exp_rate).ceil() as u32;
        let mut status_copy = Status::default();
        status_copy.job_exp = character.status.job_exp;
        status_copy.job = character.status.job;
        status_copy.job_level = character.status.job_level;
        loop {
            let next_level_requirement = self.next_job_level_required_exp(&status_copy);
            if next_level_requirement == u32::MAX {
                character.status.job_exp = 0;
                gain_exp = 0;
                break;
            }
            // Currently multi leveling is allowed, so if gained exp permit to level up...
            if status_copy.job_exp + gain_exp >= next_level_requirement {
                gained_level += 1;
                // ... we gain new level...
                status_copy.job_level += 1;
                // ... removing from gained exp, the amount spent to level up...
                gain_exp = status_copy.job_exp + gain_exp - next_level_requirement;
                status_copy.job_exp = 0;
            } else {
                // ... until there not enough exp point to reach new level
                break;
            }
        }
        if gained_level > 0 {
            self.update_job_level(character, None, Some(gained_level));
        }
        character.status.job_exp = status_copy.job_exp + gain_exp;

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

    pub fn reset_skills(&self, character: &mut Character, should_persist_skill_points: bool) {
        let skill_points = self.get_allocated_skills_point(character);
        let skills_to_reset: Vec<i32> = character.status.known_skills.iter().filter(|skill| !skill.value.is_platinium()).map(|skill| skill.value.id() as i32).collect();
        character.status.known_skills.retain(|skill| skill.value.is_platinium());

        self.update_skill_point(character, skill_points as u32, should_persist_skill_points);
        self.persistence_event_sender.send(PersistenceEvent::ResetSkills(ResetSkills { char_id: character.char_id as i32, skills: skills_to_reset })).expect("Fail to send persistence notification");
        self.skill_tree_service.send_skill_tree(character);
    }

    pub fn should_reset_stats(&self, character: &Character) -> bool {
        self.get_spent_status_point(character) > self.get_status_point_count_for_level(character)
    }

    pub fn should_reset_skills(&self, character: &Character) -> bool {
        self.get_allocated_skills_point(character) + character.status.skill_point as u8 > self.get_skill_point_count_for_level(character)
    }

    pub fn next_base_level_required_exp(&self, status: &Status) -> u32 {
        let exp = if JobName::from_value(status.job as usize).is_rebirth() {
            &self.configuration_service.config().game.exp_requirements.base_next_level_requirement.transcendent
        } else {
            &self.configuration_service.config().game.exp_requirements.base_next_level_requirement.normal
        };
        if status.base_level > exp.len() as u32 {
            // TODO: define a formula in configuration for infinite leveling, which increase leveling curve, level after level.
            // For now disable exp gain when nothing is defined
            u32::MAX
        } else {
            *exp.get((status.base_level - 1) as usize).unwrap()
        }
    }

    pub fn next_job_level_required_exp(&self, status: &Status) -> u32 {
        let job_name = JobName::from_value(status.job as usize);
        let empty_exp = vec![];
        let exp = if job_name.is_taekwon() {
            &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.taekwon_class
        } else if job_name.is_gunslinger_ninja() {
            &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.gunslinger_class
        } else if job_name.is_rebirth() {
            if job_name.is_novice() {
                &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.transcended_novice
            } else if job_name.is_first_class() {
                &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.transcended_first_class
            } else if job_name.is_second_class() {
                &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.transcended_second_class
            } else {
                &empty_exp
            }
        } else if job_name.is_novice() {
            &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.novice
        } else if job_name.is_first_class() {
            &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.first_class
        } else if job_name.is_second_class() {
            &self.configuration_service.config().game.exp_requirements.job_next_level_requirement.second_class
        } else {
            &empty_exp
        };

        if status.job_level > exp.len() as u32 {
            // TODO: define a formula in configuration for infinite leveling, which increase leveling curve, level after level.
            // For now disable exp gain when nothing is defined
            u32::MAX
        } else {
            *exp.get((status.job_level - 1) as usize).unwrap()
        }
    }

    pub fn reload_client_side_status(&self, server_ref: &Server, character: &Character) {
        let character_status = self.status_service.to_snapshot(&character.status);
        let mut packet_str = PacketZcStatusValues::new(self.configuration_service.packetver());
        packet_str.set_status_type(StatusTypes::Str.value() as u32);
        packet_str.set_default_status(character_status.base_str() as i32);
        packet_str.fill_raw();
        let mut packet_agi = PacketZcStatusValues::new(self.configuration_service.packetver());
        packet_agi.set_status_type(StatusTypes::Agi.value() as u32);
        packet_agi.set_default_status(character_status.base_agi() as i32);
        packet_agi.fill_raw();
        let mut packet_dex = PacketZcStatusValues::new(self.configuration_service.packetver());
        packet_dex.set_status_type(StatusTypes::Dex.value() as u32);
        packet_dex.set_default_status(character_status.base_dex() as i32);
        packet_dex.fill_raw();
        let mut packet_int = PacketZcStatusValues::new(self.configuration_service.packetver());
        packet_int.set_status_type(StatusTypes::Int.value() as u32);
        packet_int.set_default_status(character_status.base_int() as i32);
        packet_int.fill_raw();
        let mut packet_vit = PacketZcStatusValues::new(self.configuration_service.packetver());
        packet_vit.set_status_type(StatusTypes::Vit.value() as u32);
        packet_vit.set_default_status(character_status.base_vit() as i32);
        packet_vit.fill_raw();
        let mut packet_luk = PacketZcStatusValues::new(self.configuration_service.packetver());
        packet_luk.set_status_type(StatusTypes::Luk.value() as u32);
        packet_luk.set_default_status(character_status.base_luk() as i32);
        packet_luk.fill_raw();
        let mut packet_str_increase_cost = PacketZcParChange::new(self.configuration_service.packetver());
        packet_str_increase_cost.set_var_id(StatusTypes::StrNextLevelIncreaseCost.value() as u16);
        packet_str_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.str, "str") as i32);
        packet_str_increase_cost.fill_raw();
        let mut packet_agi_increase_cost = PacketZcParChange::new(self.configuration_service.packetver());
        packet_agi_increase_cost.set_var_id(StatusTypes::AgiNextLevelIncreaseCost.value() as u16);
        packet_agi_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.agi, "agi") as i32);
        packet_agi_increase_cost.fill_raw();
        let mut packet_dex_increase_cost = PacketZcParChange::new(self.configuration_service.packetver());
        packet_dex_increase_cost.set_var_id(StatusTypes::DexNextLevelIncreaseCost.value() as u16);
        packet_dex_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.dex, "dex") as i32);
        packet_dex_increase_cost.fill_raw();
        let mut packet_vit_increase_cost = PacketZcParChange::new(self.configuration_service.packetver());
        packet_vit_increase_cost.set_var_id(StatusTypes::VitNextLevelIncreaseCost.value() as u16);
        packet_vit_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.vit, "vit") as i32);
        packet_vit_increase_cost.fill_raw();
        let mut packet_int_increase_cost = PacketZcParChange::new(self.configuration_service.packetver());
        packet_int_increase_cost.set_var_id(StatusTypes::IntNextLevelIncreaseCost.value() as u16);
        packet_int_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.int, "int") as i32);
        packet_int_increase_cost.fill_raw();
        let mut packet_luk_increase_cost = PacketZcParChange::new(self.configuration_service.packetver());
        packet_luk_increase_cost.set_var_id(StatusTypes::LukNextLevelIncreaseCost.value() as u16);
        packet_luk_increase_cost.set_count(self.stat_raising_cost_for_next_level(character.status.luk, "luk") as i32);
        packet_luk_increase_cost.fill_raw();
        let mut packet_status_point = PacketZcParChange::new(self.configuration_service.packetver());
        packet_status_point.set_var_id(StatusTypes::Statuspoint.value() as u16);
        packet_status_point.set_count(character.status.status_point as i32);
        packet_status_point.fill_raw();
        let mut packet_skill_point = PacketZcParChange::new(self.configuration_service.packetver());
        packet_skill_point.set_var_id(StatusTypes::Skillpoint.value() as u16);
        packet_skill_point.set_count(character.status.skill_point as i32);
        packet_skill_point.fill_raw();

        let mut packet_hit = PacketZcParChange::new(self.configuration_service.packetver());
        packet_hit.set_var_id(StatusTypes::Hit.value() as u16);
        packet_hit.set_count(character_status.hit() as i32);
        packet_hit.fill_raw();
        let mut packet_flee = PacketZcParChange::new(self.configuration_service.packetver());
        packet_flee.set_var_id(StatusTypes::Flee1.value() as u16);
        packet_flee.set_count(character_status.flee() as i32);
        packet_flee.fill_raw();
        let mut packet_aspd = PacketZcParChange::new(self.configuration_service.packetver());
        packet_aspd.set_var_id(StatusTypes::Aspd.value() as u16);
        let aspd = character_status.aspd();
        packet_aspd.set_count(StatusService::instance().client_aspd(aspd));
        packet_aspd.fill_raw();
        let mut packet_atk = PacketZcParChange::new(self.configuration_service.packetver());
        packet_atk.set_var_id(StatusTypes::Atk1.value() as u16);
        packet_atk.set_count(character_status.atk_left_side());
        packet_atk.fill_raw();
        let mut packet_atk2 = PacketZcParChange::new(self.configuration_service.packetver());
        packet_atk2.set_var_id(StatusTypes::Atk2.value() as u16);
        packet_atk2.set_count(character_status.atk_right_side());
        packet_atk2.fill_raw();
        let mut packet_def = PacketZcParChange::new(self.configuration_service.packetver());
        packet_def.set_var_id(StatusTypes::Def1.value() as u16);
        packet_def.set_count(character_status.def() as i32);
        packet_def.fill_raw();
        let mut packet_flee2 = PacketZcParChange::new(self.configuration_service.packetver());
        packet_flee2.set_var_id(StatusTypes::Flee2.value() as u16);
        packet_flee2.set_count(character_status.flee() as i32);
        packet_flee2.fill_raw();
        let mut packet_crit = PacketZcParChange::new(self.configuration_service.packetver());
        packet_crit.set_var_id(StatusTypes::Critical.value() as u16);
        packet_crit.set_count(character_status.crit() as i32);
        packet_crit.fill_raw();
        let mut packet_matk = PacketZcParChange::new(self.configuration_service.packetver());
        packet_matk.set_var_id(StatusTypes::Matk1.value() as u16);
        packet_matk.set_count(character_status.matk_min() as i32);
        packet_matk.fill_raw();
        let mut packet_matk2 = PacketZcParChange::new(self.configuration_service.packetver());
        packet_matk2.set_var_id(StatusTypes::Matk2.value() as u16);
        packet_matk2.set_count(character_status.matk_max() as i32);
        packet_matk2.fill_raw();
        let mut packet_mdef2 = PacketZcParChange::new(self.configuration_service.packetver());
        packet_mdef2.set_var_id(StatusTypes::Mdef2.value() as u16);
        packet_mdef2.set_count(character_status.mdef() as i32);
        packet_mdef2.fill_raw();
        let mut packet_attack_range = PacketZcAttackRange::new(self.configuration_service.packetver());
        packet_attack_range.set_current_att_range(character.status.attack_range() as i16);
        packet_attack_range.fill_raw();
        let mut packet_maxhp = PacketZcParChange::new(self.configuration_service.packetver());
        packet_maxhp.set_var_id(StatusTypes::Maxhp.value() as u16);
        packet_maxhp.set_count(character_status.max_hp() as i32);
        packet_maxhp.fill_raw();
        let mut packet_maxsp = PacketZcParChange::new(self.configuration_service.packetver());
        packet_maxsp.set_var_id(StatusTypes::Maxsp.value() as u16);
        packet_maxsp.set_count(character_status.max_sp() as i32);
        packet_maxsp.fill_raw();
        let mut packet_hp = PacketZcParChange::new(self.configuration_service.packetver());
        packet_hp.set_var_id(StatusTypes::Hp.value() as u16);
        packet_hp.set_count(character_status.hp() as i32);
        packet_hp.fill_raw();
        let mut packet_sp = PacketZcParChange::new(self.configuration_service.packetver());
        packet_sp.set_var_id(StatusTypes::Sp.value() as u16);
        packet_sp.set_count(character_status.sp() as i32);
        packet_sp.fill_raw();
        let mut packet_speed = PacketZcParChange::new(self.configuration_service.packetver());
        packet_speed.set_var_id(StatusTypes::Speed.value() as u16);
        packet_speed.set_count(character.status.speed as i32);
        packet_speed.fill_raw();
        let mut packet_exp_required_to_reach_next_base_level = PacketZcParChange::new(self.configuration_service.packetver());
        packet_exp_required_to_reach_next_base_level.set_var_id(StatusTypes::Nextbaseexp.value() as u16);
        packet_exp_required_to_reach_next_base_level.set_count(self.next_base_level_required_exp(&character.status) as i32);
        packet_exp_required_to_reach_next_base_level.fill_raw();
        let mut packet_exp_required_to_reach_next_job_level = PacketZcParChange::new(self.configuration_service.packetver());
        packet_exp_required_to_reach_next_job_level.set_var_id(StatusTypes::Nextjobexp.value() as u16);
        packet_exp_required_to_reach_next_job_level.set_count(self.next_job_level_required_exp(&character.status) as i32);
        packet_exp_required_to_reach_next_job_level.fill_raw();


        let mut final_response_packet: Vec<u8> = chain_packets(vec![
            &packet_str, &packet_agi, &packet_dex, &packet_int, &packet_luk, &packet_vit,
            &packet_status_point, &packet_skill_point, &packet_str_increase_cost, &packet_agi_increase_cost, &packet_vit_increase_cost,
            &packet_dex_increase_cost, &packet_int_increase_cost, &packet_luk_increase_cost,
            &packet_hit, &packet_flee, &packet_aspd, &packet_atk, &packet_atk2, &packet_def,
            &packet_flee2, &packet_crit, &packet_matk, &packet_matk2,
            &packet_mdef2, &packet_attack_range, &packet_maxhp, &packet_maxsp, &packet_hp,
            &packet_sp, &packet_speed,
            &packet_exp_required_to_reach_next_base_level, &packet_exp_required_to_reach_next_job_level,
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
                        let mut packet_zc_item_entry = PacketZcItemEntry::new(self.configuration_service.packetver());
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
                } else if matches!(map_item.object_type(), MapItemType::Mob) {
                    if let Some(mob) = map_instance_state.get_mob(map_item.id()) {
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry7::new(self.configuration_service.packetver());
                        packet_zc_notify_standentry.set_job(map_item.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(PacketZcNotifyStandentry7::base_len(self.configuration_service.packetver()) as i16);
                        // packet_zc_notify_standentry.set_name(name);
                        packet_zc_notify_standentry.set_pos_dir(position.to_pos());
                        packet_zc_notify_standentry.set_objecttype(map_item.object_type_value() as u8);
                        packet_zc_notify_standentry.set_aid(map_item.id());
                        packet_zc_notify_standentry.set_gid(map_item.id());
                        packet_zc_notify_standentry.set_clevel(3);
                        packet_zc_notify_standentry.set_speed(mob.status.speed() as i16);
                        packet_zc_notify_standentry.set_hp(mob.status.hp());
                        packet_zc_notify_standentry.set_max_hp(mob.status.max_hp());
                        packet_zc_notify_standentry.fill_raw_with_packetver(Some(self.configuration_service.packetver()));
                        packets.extend(packet_zc_notify_standentry.raw);
                        if mob.is_moving() {
                            let mut packet_zc_notify_move = PacketZcNotifyMove::new(self.configuration_service.packetver());
                            packet_zc_notify_move.set_gid(mob.id);
                            packet_zc_notify_move.move_data = mob.position().to_move_data(mob.movements.first().unwrap().position());
                            // packet_zc_notify_move.move_data = mob_movement.from.to_move_data(&mob_movement.to);
                            packet_zc_notify_move.set_move_start_time(get_tick_client());
                            packet_zc_notify_move.fill_raw();
                            #[cfg(feature = "debug_mob_movement")]
                            {
                                info!("A moving mob appeared! {} moving from {} to {}.", mob.id, mob.position(), mob.movements.first().unwrap().position());
                            }
                            packets.extend(packet_zc_notify_move.raw);
                        }
                    }
                } else {
                    let mut packet_zc_notify_standentry = PacketZcNotifyStandentry7::new(self.configuration_service.packetver());
                    packet_zc_notify_standentry.set_job(map_item.client_item_class());
                    packet_zc_notify_standentry.set_packet_length(PacketZcNotifyStandentry7::base_len(self.configuration_service.packetver()) as i16);
                    // packet_zc_notify_standentry.set_name(name);
                    packet_zc_notify_standentry.set_pos_dir(position.to_pos());
                    packet_zc_notify_standentry.set_objecttype(map_item.object_type_value() as u8);
                    packet_zc_notify_standentry.set_aid(map_item.id());
                    packet_zc_notify_standentry.set_gid(map_item.id());
                    packet_zc_notify_standentry.fill_raw_with_packetver(Some(self.configuration_service.packetver()));
                    packets.extend(packet_zc_notify_standentry.raw);
                }
            }
        }
        if !packets.is_empty() {
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets))).expect("Failed to send notification to client");
        }

        let mut packets = vec![];
        for map_item in character.map_view.iter() {
            if !new_map_view.contains(map_item) {
                if let Some(position) = server_state.map_item_x_y(map_item, character.current_map_name(), character.current_map_instance()) {
                    debug!("Vanish map_item {} at {},{}", map_item.object_type(), position.x(), position.y());
                    if matches!(map_item.object_type(), MapItemType::DroppedItem) {
                        let mut packet_zc_item_disappear = PacketZcItemDisappear::new(self.configuration_service.packetver());
                        packet_zc_item_disappear.set_itaid(map_item.id());
                        packet_zc_item_disappear.fill_raw();
                        packets.extend(packet_zc_item_disappear.raw);
                    } else {
                        let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new(self.configuration_service.packetver());
                        packet_zc_notify_vanish.set_gid(map_item.id());
                        packet_zc_notify_vanish.fill_raw();
                        packets.extend(packet_zc_notify_vanish.raw);
                    }
                }
            }
        }
        if !packets.is_empty() {
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packets))).expect("Failed to send notification to client");
        }
        character.map_view = new_map_view;
    }

    fn send_status_update_and_defer_db_update(&self, char_id: u32, status_type: StatusTypes, new_value: u32) {
        self.persistence_event_sender.send(PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id, db_column: status_type.to_column().unwrap_or_else(|| panic!("no db column name for status of type {status_type:?}")).to_string(), value: new_value })).expect("Fail to send persistence notification");
        let mut packet_status_change = PacketZcParChange::new(self.configuration_service.packetver());
        packet_status_change.set_var_id(status_type.value() as u16);
        packet_status_change.set_count(new_value as i32);
        packet_status_change.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(char_id, packet_status_change.raw))).expect("Fail to send client notification");
    }

    pub fn character_increase_stat(&self, character: &mut Character, character_update_stat: CharacterUpdateStat) {
        let result = self.allocate_status_point(character, StatusTypes::from_value(character_update_stat.stat_id as usize), character_update_stat.change_amount);
        let mut packet_zc_status_change_ack = PacketZcStatusChangeAck::new(self.configuration_service.packetver());
        packet_zc_status_change_ack.set_status_id(character_update_stat.stat_id);
        packet_zc_status_change_ack.set_result(result);
        packet_zc_status_change_ack.set_value(self.stat_value(&character.status, &StatusTypes::from_value(character_update_stat.stat_id as usize)) as u8);
        packet_zc_status_change_ack.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, packet_zc_status_change_ack.raw))).expect("Fail to send client notification");
    }

    pub fn character_kill_monster(&self, character: &mut Character, character_kill_monster: CharacterKillMonster, map_instance: &MapInstance) {
        self.gain_base_exp(character, character_kill_monster.mob_base_exp);
        self.gain_job_exp(character, character_kill_monster.mob_job_exp);
        // TODO check autoloot
        let autoloot = false;
        if autoloot {} else {
            map_instance.add_to_delayed_tick(MapEvent::MobDropItems(MobDropItems { owner_id: character_kill_monster.char_id, mob_id: character_kill_monster.mob_id, mob_x: character_kill_monster.mob_x, mob_y: character_kill_monster.mob_y }), 400);
        }
    }
}