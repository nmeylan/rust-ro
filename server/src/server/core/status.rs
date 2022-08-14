use std::sync::atomic::{AtomicU16, AtomicU32};
use std::sync::atomic::Ordering::Relaxed;

use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::MobModel;
use crate::server::configuration::GameConfig;

#[derive(SettersAll, Debug, Clone)]
pub struct Status {
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
    pub base_atk: u32,
    pub matk_min: u32,
    pub matk_max: u32,
    pub speed: u16,
    pub attack_motion: u32,
    pub attack_delay: u32,
    pub delay_motion: u32,
    pub hit: u32,
    pub flee: u32,
    pub crit: u32,
    pub def: u32,
    pub mdef: u32,
    pub aspd: u32,
    pub look: Option<Look>,
}

#[derive(SettersAll, Debug)]
pub struct Look {
    pub hair: AtomicU16,
    pub hair_color: AtomicU32,
    pub clothes_color: AtomicU32,
    pub body: AtomicU32,
    pub weapon: AtomicU32,
    pub shield: AtomicU32,
    pub head_top: AtomicU32,
    pub head_middle: AtomicU32,
    pub head_bottom: AtomicU32,
    pub robe: AtomicU32,
}

impl Clone for Look {
    fn clone(&self) -> Self {
        Self {
            hair: AtomicU16::new(self.hair.load(Relaxed)),
            hair_color: AtomicU32::new(self.hair_color.load(Relaxed)),
            clothes_color: AtomicU32::new(self.clothes_color.load(Relaxed)),
            body: AtomicU32::new(self.body.load(Relaxed)),
            weapon: AtomicU32::new(self.weapon.load(Relaxed)),
            shield: AtomicU32::new(self.shield.load(Relaxed)),
            head_top: AtomicU32::new(self.head_top.load(Relaxed)),
            head_middle: AtomicU32::new(self.head_middle.load(Relaxed)),
            head_bottom: AtomicU32::new(self.head_bottom.load(Relaxed)),
            robe: AtomicU32::new(self.robe.load(Relaxed)),
        }
    }
}

#[derive(r#enum::WithNumberValue, Debug)]
pub enum LookType {
    Hair,
    Weapon,
    HeadBottom,
    HeadTop,
    HeadMid,
    HairColor,
    ClothesColor,
    Shield,
    Shoes,
    Robe,
    Body,
}

impl Status {
    pub fn from_char_model(char_model: &CharSelectModel, configuration: &GameConfig) -> Status {
        Status {
            hp: char_model.hp,
            sp: char_model.sp,
            max_hp: char_model.max_hp,
            max_sp: char_model.max_sp,
            str: char_model.str,
            agi: char_model.agi,
            vit: char_model.vit,
            int: char_model.int,
            dex: char_model.dex,
            luk: char_model.luk,
            base_atk: 0,
            matk_min: 0,
            matk_max: 0,
            speed: configuration.default_char_speed,
            attack_motion: 0,
            attack_delay: 0,
            delay_motion: 0,
            hit: 0,
            flee: 0,
            crit: 0,
            def: 0,
            mdef: 0,
            aspd: 0,
            look: Some(Look {
                hair: AtomicU16::new(char_model.hair),
                hair_color: AtomicU32::new(char_model.hair_color),
                clothes_color: AtomicU32::new(char_model.clothes_color),
                body: AtomicU32::new(char_model.body),
                weapon: AtomicU32::new(char_model.weapon),
                shield: AtomicU32::new(char_model.shield),
                head_top: AtomicU32::new(char_model.head_top),
                head_middle: AtomicU32::new(char_model.head_mid),
                head_bottom: AtomicU32::new(char_model.head_bottom),
                robe: AtomicU32::new(char_model.robe),
            }),
        }
    }
    pub fn from_mob_model(mob_model: &MobModel) -> Status {
        Status {
            hp: mob_model.hp,
            sp: mob_model.sp,
            max_hp: mob_model.hp,
            max_sp: mob_model.sp,
            str: mob_model.str,
            agi: mob_model.agi,
            vit: mob_model.vit,
            int: mob_model.int,
            dex: mob_model.dex,
            luk: mob_model.luk,
            base_atk: mob_model.atk1,
            matk_min: mob_model.atk1,
            matk_max: mob_model.atk2,
            speed: mob_model.speed,
            attack_motion: mob_model.atk_motion,
            attack_delay: mob_model.atk_delay,
            delay_motion: mob_model.damage_motion,
            hit: 0,
            flee: 0,
            crit: 0,
            def: mob_model.def,
            mdef: mob_model.mdef,
            aspd: 0,
            look: None,
        }
    }
}