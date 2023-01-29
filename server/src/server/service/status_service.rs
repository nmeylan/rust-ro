use std::sync::{Once};
use std::sync::mpsc::SyncSender;



use enums::status::StatusTypes;
use enums::weapon::WeaponType;
use crate::enums::EnumWithStringValue;
use packets::packets::{PacketZcAttackRange, PacketZcParChange, PacketZcStatusValues};



use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::persistence_event::PersistenceEvent;

use crate::server::Server;
use crate::server::service::global_config_service::GlobalConfigService;

use crate::server::state::character::Character;
use crate::util::packet::chain_packets;

static mut SERVICE_INSTANCE: Option<StatusService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct StatusService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    configuration_service: &'static GlobalConfigService,
}

impl StatusService {
    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, configuration_service: &'static GlobalConfigService) -> StatusService {
        StatusService { client_notification_sender, persistence_event_sender, configuration_service }
    }
    pub fn instance() -> &'static StatusService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(StatusService::new(client_notification_sender, persistence_event_sender, configuration_service));
        });
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
        let aspd = StatusService::instance().aspd(character);
        packet_aspd.set_count(StatusService::instance().client_aspd(aspd));
        packet_aspd.fill_raw();
        let mut packet_atk = PacketZcParChange::new();
        packet_atk.set_var_id(StatusTypes::Atk1.value() as u16);
        packet_atk.set_count(StatusService::instance().status_atk_left_side(character) as i32);
        packet_atk.fill_raw();
        let mut packet_atk2 = PacketZcParChange::new();
        packet_atk2.set_var_id(StatusTypes::Atk2.value() as u16);
        packet_atk2.set_count(StatusService::instance().status_atk_right_side(character) as i32);
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

    pub fn attack_per_seconds(&self, aspd: f32) -> f32 {
        50_f32 / (200_f32 - aspd.min(199.0))
    }

    pub fn attack_motion(&self, character: &Character) -> u32 {
        let aspd = self.aspd(character);
        (1000.0 / self.attack_per_seconds(aspd)).round() as u32
    }

    pub fn client_aspd(&self, aspd: f32) -> i32 {
        ((200_f32 - aspd.min(199.0)) * 10.0).round() as i32
    }

    ///  PRE-RE formula: 200-(WD-([WD*AGI/25]+[WD*DEX/100])/10)*(1-SM)  https://irowiki.org/classic/ASPD
    /// [] - Square brackets hold the same priority as normal brackets, but indicate that the value of the contents should be rounded down to the nearest whole number (integer) once calculated.
    /// http://calc.free-ro.com/
    pub fn aspd(&self, character: &Character) -> f32 {
        let weapon_delay = self.weapon_delay(character) as f32 / 10.0;
        let speed_modifier = 0_f32;
        200.0 - (weapon_delay - ((((weapon_delay * (character.status.agi as f32)) / 25.0).floor() + ((weapon_delay * (character.status.dex as f32)) / 100.0).floor()) / 10.0) * (1.0 - speed_modifier))
    }

    pub fn weapon_delay(&self, character: &Character) -> u32 {
        let weapon = self.right_hand_weapon_type(character);
        *self.configuration_service.get_job_config(character.status.job).base_aspd().get(weapon.as_str()).unwrap_or(&2000)
    }

    pub fn right_hand_weapon_type(&self, character: &Character) -> WeaponType {
        character.right_hand_weapon()
            .map(|(_, weapon)| self.configuration_service.get_item(weapon.item_id).weapon_type.expect("Expected weapon to have subtype"))
            .unwrap_or(WeaponType::Fist)
    }

    /// PRE-RE https://irowiki.org/classic/Attacks
    /// UI left side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    ///
    /// Atk stands for Attack and gives an indication of how much damage you will do when you hit something.
    ///The visible components of the Atk score are your Strength plus the Atk of the weapon you are using on the left and the damage bonus from any pluses the weapon might have on the right.
    ///The real value on the left of your Atk score includes hidden bonuses from Strength, Dexterity and Luck.
    ///For fists, the true value is equal to: STR + [STR/10]^2 + [DEX/5] + [LUK/5] where [] indicates you round the value inside down before continuing and ^2 indicates squaring.
    ///For weapons, the true value is equal to: STR + [STR/10]^2 + [DEX/5] + [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you round the value inside down before continuing and ^2 indicates squaring.
    ///For missile weapons, the true value is equal to: DEX + [DEX/10]^2 + [STR/5] + [LUK/5] + WeaponAtk + AtkBonusCards where [] indicates you round the value inside down before continuing and ^2 indicates squaring.
    ///Not counting the value of WeaponAtk and AtkBonusCards, this true value is often referred to as the base damage. This base damage is basically the your Atk with bare fists.
    pub fn status_atk_left_side(&self, character: &Character) -> i32 {
        let imposito_magnus = 0;
        let _upgrade_damage = 0;
        let _atk_cards = 0;
        (self.fist_atk(character) + self.weapon_atk(character) + imposito_magnus + self.weapon_upgrade_damage(character) + self.atk_cards(character)) as i32
    }

    pub fn fist_atk(&self, character: &Character) -> u32 {
        let mut str;
        let dex;
        let mut is_ranged_weapon = false;

        let weapon_type = self.right_hand_weapon_type(character);
        is_ranged_weapon = weapon_type.is_ranged();
        if is_ranged_weapon {
            str = character.status.dex;
            dex = character.status.str;
        } else {
            str = character.status.str;
            dex = character.status.dex;
        }
        // For homunculus
        // dstr = str / 10;
        // str += dstr*dstr;
        let dstr = str / 10;
        str += dstr * dstr;
        str += dex / 5 + character.status.luk / 5;
        str as u32
    }

    pub fn atk_cards(&self, _character: &Character) -> u32 {
        0
    }

    pub fn weapon_upgrade_damage(&self, _character: &Character) -> u32 {
        0
    }

    pub fn weapon_atk(&self, character: &Character) -> u32 {
        let right_hand_weapon_atk: u32 = if let Some((_, right_hand_weapon)) = character.right_hand_weapon() {
            self.configuration_service.get_item(right_hand_weapon.item_id).attack.unwrap_or(0) as u32
        } else {
            0
        };
        right_hand_weapon_atk
    }

    pub fn weapon_lvl(&self, character: &Character) -> Option<i16> {
        if let Some((_, right_hand_weapon)) = character.right_hand_weapon() {
            self.configuration_service.get_item(right_hand_weapon.item_id).weapon_level
        } else {
            None
        }
    }

    /// UI right side atk in status info panel
    /// https://web.archive.org/web/20060717223009/http://rodatazone.simgaming.net/mechanics/substats.php
    /// https://web.archive.org/web/20060717222819/http://rodatazone.simgaming.net/items/upgrading.php
    pub fn status_atk_right_side(&self, _character: &Character) -> i32 {
        // TODO: it is refinement damage. do not mix with refinement bonus which refers to random additional atk for over upgrade
        // refinement
        //    Weapon Lv. 1 - Every +1 upgrade gives +2 ATK (+1~3 ATK for every overupgrade).
        //     Weapon Lv. 2 - Every +1 upgrade gives +3 ATK (+1~5 ATK for every overupgrade).
        //     Weapon Lv. 3 - Every +1 upgrade gives +5 ATK (+1~7 ATK for every overupgrade).
        //     Weapon Lv. 4 - Every +1 upgrade gives +7 ATK (+1~13(?) ATK for every overupgrade).
        //    Weapon Lv. 1 - Safety Level +7
        //     Weapon Lv. 2 - Safety Level +6
        //     Weapon Lv. 3 - Safety Level +5
        //     Weapon Lv. 4 - Safety Level +4
        0
    }

    /// VIT + rnd(0,[VIT/20]^2-1).
    pub fn mob_vit_def(&self, vit: u32) -> u32 {
        let rng = fastrand::Rng::new();
        vit + rng.u32(0..(((vit as f32 / 20.0).floor() as u32) ^ 2) as u32 - 1)
    }
    /// [VIT*0.5] + rnd([VIT*0.3], max([VIT*0.3],[VIT^2/150]-1)).
    pub fn character_vit_def(&self, _vit: u32) -> u32 {
        0
    }
}
