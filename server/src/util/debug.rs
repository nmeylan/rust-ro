use std::collections::HashMap;
use std::fmt::Display;
use std::sync::mpsc::SyncSender;
use models::enums::EnumWithMaskValueU64;
use models::enums::item::EquipmentLocation;
use models::item::{WearAmmo, WearGear, WearWeapon};
use packets::packets::{PacketZcNotifyChat};

use packets::packets::Packet;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;

#[allow(dead_code)]
pub fn debug_in_game_chat(notification_sender: SyncSender<Notification>, character: &Character, text: String) {
    let mut zc_notify_chat = PacketZcNotifyChat::new(GlobalConfigService::instance().packetver());
    zc_notify_chat.set_gid(character.account_id);
    zc_notify_chat.set_packet_length((text.len() + 8) as i16);
    zc_notify_chat.set_msg(text);
    zc_notify_chat.fill_raw();
    notification_sender.send(
        Notification::Char(CharNotification::new(character.char_id, std::mem::take(zc_notify_chat.raw_mut())))
    ).unwrap();
}


pub struct WearWeaponForDisplay<'a> {
    pub wear_weapon: &'a WearWeapon,
    pub config_service: &'static GlobalConfigService,
}
pub struct WearGearForDisplay<'a> {
    pub wear_gear: &'a WearGear,
    pub config_service: &'static GlobalConfigService,
}
pub struct WearAmmoForDisplay<'a> {
    pub wear_ammo: &'a WearAmmo,
    pub config_service: &'static GlobalConfigService,
}

impl<'a> WearWeaponForDisplay<'a> {
    pub fn new(weapon: &'a WearWeapon, config_service: &'static GlobalConfigService) -> Self {
        Self {
            wear_weapon: weapon,
            config_service,
        }
    }
}
impl<'a> WearGearForDisplay<'a> {
    pub fn new(gear: &'a WearGear, config_service: &'static GlobalConfigService) -> Self {
        Self {
            wear_gear: gear,
            config_service,
        }
    }
}
impl<'a> WearAmmoForDisplay<'a> {
    pub fn new(ammo: &'a WearAmmo, config_service: &'static GlobalConfigService) -> Self {
        Self {
            wear_ammo: ammo,
            config_service,
        }
    }
}

impl<'a> Display for WearWeaponForDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cards = vec![];
        if self.wear_weapon.card0 > 0 {
            cards.push(self.config_service.get_item(self.wear_weapon.card0 as i32).name_english.clone());
        }
        if self.wear_weapon.card1 > 0 {
            cards.push(self.config_service.get_item(self.wear_weapon.card1 as i32).name_english.clone());
        }
        if self.wear_weapon.card2 > 0 {
            cards.push(self.config_service.get_item(self.wear_weapon.card2 as i32).name_english.clone());
        }
        if self.wear_weapon.card3 > 0 {
            cards.push(self.config_service.get_item(self.wear_weapon.card3 as i32).name_english.clone());
        }
        write!(f, "+{} {}({:?}), atk: {}, lvl: {}, element: {:?}", self.wear_weapon.refine(),
               self.config_service.get_item(self.wear_weapon.item_id()).name_english.clone(),
               self.wear_weapon.weapon_type(), self.wear_weapon.attack(), self.wear_weapon.level(),
               self.wear_weapon.element(),
        )?;
        if !cards.is_empty() {
            write!(f, ", cards: [{}]", cards.join(","))?;
        };
        write!(f, ".")
    }
}


impl<'a> Display for WearGearForDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let item_name = self.config_service.get_item(self.wear_gear.item_id).name_english.clone();
        write!(f, "+{} {}({:?}), def: {}, lvl: {}", self.wear_gear.refine(),
               &item_name,
               EquipmentLocation::try_from_flag(self.wear_gear.location()).expect(format!("Unable to get equipment location with value {}, for item {}", self.wear_gear.location(), item_name).as_str()), self.wear_gear.def(), self.wear_gear.level(),
        )?;
        if self.wear_gear.card0 > 0 {
            write!(f, ", cards [{}]", self.config_service.get_item(self.wear_gear.card0 as i32).name_english.clone())?;
        };
        write!(f, ".")
    }
}

impl<'a> Display for WearAmmoForDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?}), atk: {}, element: {:?}", self.config_service.get_item(self.wear_ammo.item_id).name_english.clone(),
               self.wear_ammo.ammo_type, self.wear_ammo.attack, self.wear_ammo.element,
        )
    }
}