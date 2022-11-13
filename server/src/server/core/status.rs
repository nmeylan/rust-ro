use std::sync::atomic::{AtomicU16, AtomicU32};
use std::sync::atomic::Ordering::Relaxed;

use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::MobModel;
use crate::server::configuration::GameConfig;

#[derive(SettersAll, Debug)]
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
    pub zeny: u32,
}

impl Clone for Status {
    fn clone(&self) -> Self {
        Self {
            hp: self.hp.clone(),
            sp: self.sp.clone(),
            max_hp: self.max_hp.clone(),
            max_sp: self.max_sp.clone(),
            str: self.str.clone(),
            agi: self.agi.clone(),
            vit: self.vit.clone(),
            int: self.int.clone(),
            dex: self.dex.clone(),
            luk: self.luk.clone(),
            base_atk: self.base_atk.clone(),
            matk_min: self.matk_min.clone(),
            matk_max: self.matk_max.clone(),
            speed: self.speed.clone(),
            attack_motion: self.attack_motion.clone(),
            attack_delay: self.attack_delay.clone(),
            delay_motion: self.delay_motion.clone(),
            hit: self.hit.clone(),
            flee: self.flee.clone(),
            crit: self.crit.clone(),
            def: self.def.clone(),
            mdef: self.mdef.clone(),
            aspd: self.aspd.clone(),
            look: self.look.clone(),
            zeny: self.zeny,
        }
    }
}

#[derive(SettersAll, Debug)]
pub struct Look {
    pub hair: u16,
    pub hair_color: u32,
    pub clothes_color: u32,
    pub body: u32,
    pub weapon: u32,
    pub shield: u32,
    pub head_top: u32,
    pub head_middle: u32,
    pub head_bottom: u32,
    pub robe: u32,
}

impl Clone for Look {
    fn clone(&self) -> Self {
        Self {
            hair: self.hair,
            hair_color: self.hair_color,
            clothes_color: self.clothes_color,
            body: self.body,
            weapon: self.weapon,
            shield: self.shield,
            head_top: self.head_top,
            head_middle: self.head_middle,
            head_bottom: self.head_bottom,
            robe: self.robe,
        }
    }
}

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone)]
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
                hair: char_model.hair,
                hair_color: char_model.hair_color,
                clothes_color: char_model.clothes_color,
                body: char_model.body,
                weapon: char_model.weapon,
                shield: char_model.shield,
                head_top: char_model.head_top,
                head_middle: char_model.head_mid,
                head_bottom: char_model.head_bottom,
                robe: char_model.robe,
            }),
            zeny: char_model.zeny,
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
            zeny: 0,
        }
    }
}