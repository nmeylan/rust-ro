use std::sync::Once;
use regex::internal::Char;
use enums::status::StatusTypes;
use packets::packets::{PacketZcAttackRange, PacketZcParChange, PacketZcStatusValues};
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::Server;
use crate::server::service::battle_service::BattleService;
use crate::server::state::character::Character;
use crate::util::packet::chain_packets;

static mut SERVICE_INSTANCE: Option<StatusService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct StatusService {}

impl StatusService {
    pub fn instance() -> &'static StatusService {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(StatusService::new());
        });
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    fn new() -> Self {
        StatusService {}
    }

    pub fn calculate_status(&self, server_ref: &Server, character: &Character) {
        let mut packet_str = PacketZcStatusValues::new();
        packet_str.set_status_type(StatusTypes::Str.value());
        packet_str.set_default_status(character.status.str as i32);
        packet_str.fill_raw();
        let mut packet_agi = PacketZcStatusValues::new();
        packet_agi.set_status_type(StatusTypes::Agi.value());
        packet_agi.set_default_status(character.status.agi as i32);
        packet_agi.fill_raw();
        let mut packet_dex = PacketZcStatusValues::new();
        packet_dex.set_status_type(StatusTypes::Dex.value());
        packet_dex.set_default_status(character.status.dex as i32);
        packet_dex.fill_raw();
        let mut packet_int = PacketZcStatusValues::new();
        packet_int.set_status_type(StatusTypes::Int.value());
        packet_int.set_default_status(character.status.int as i32);
        packet_int.fill_raw();
        let mut packet_luk = PacketZcStatusValues::new();
        packet_luk.set_status_type(StatusTypes::Luk.value());
        packet_luk.set_default_status(character.status.luk as i32);
        packet_luk.fill_raw();
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
        let aspd = BattleService::instance().aspd(character);
        packet_aspd.set_count(BattleService::instance().client_aspd(aspd));
        packet_aspd.fill_raw();
        let mut packet_atk = PacketZcParChange::new();
        packet_atk.set_var_id(StatusTypes::Atk1.value() as u16);
        packet_atk.set_count(BattleService::instance().base_atk(character) as i32);
        packet_atk.fill_raw();
        let mut packet_atk2 = PacketZcParChange::new();
        packet_atk2.set_var_id(StatusTypes::Atk2.value() as u16);
        packet_atk2.set_count(BattleService::instance().atk2(character) as i32);
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

        let final_response_packet: Vec<u8> = chain_packets(vec![
            &packet_str, &packet_agi, &packet_dex, &packet_int, &packet_luk,
            &packet_hit, &packet_flee, &packet_aspd, &packet_atk, &packet_atk2, &packet_def,
            &packet_flee2, &packet_crit, &packet_matk, &packet_matk2,
            &packet_mdef2, &packet_attack_range, &packet_maxhp, &packet_maxsp, &packet_hp,
            &packet_sp, &packet_speed,
        ]);
        server_ref.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, final_response_packet)))
            .expect("Fail to send client notification");
    }
}