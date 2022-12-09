


use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::MobModel;
use crate::server::core::configuration::GameConfig;

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
            hp: self.hp,
            sp: self.sp,
            max_hp: self.max_hp,
            max_sp: self.max_sp,
            str: self.str,
            agi: self.agi,
            vit: self.vit,
            int: self.int,
            dex: self.dex,
            luk: self.luk,
            base_atk: self.base_atk,
            matk_min: self.matk_min,
            matk_max: self.matk_max,
            speed: self.speed,
            attack_motion: self.attack_motion,
            attack_delay: self.attack_delay,
            delay_motion: self.delay_motion,
            hit: self.hit,
            flee: self.flee,
            crit: self.crit,
            def: self.def,
            mdef: self.mdef,
            aspd: self.aspd,
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
            hp: char_model.hp as u32,
            sp: char_model.sp as u32,
            max_hp: char_model.max_hp as u32,
            max_sp: char_model.max_sp as u32,
            str: char_model.str as u16,
            agi: char_model.agi as u16,
            vit: char_model.vit as u16,
            int: char_model.int as u16,
            dex: char_model.dex as u16,
            luk: char_model.luk as u16,
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
                hair: char_model.hair as u16,
                hair_color: char_model.hair_color as u32,
                clothes_color: char_model.clothes_color as u32,
                body: char_model.body as u32,
                weapon: char_model.weapon as u32,
                shield: char_model.shield as u32,
                head_top: char_model.head_top as u32,
                head_middle: char_model.head_mid as u32,
                head_bottom: char_model.head_bottom as u32,
                robe: char_model.robe as u32,
            }),
            zeny: char_model.zeny as u32,
        }
    }
    pub fn from_mob_model(mob_model: &MobModel) -> Status {
        Status {
            hp: mob_model.hp as u32,
            sp: mob_model.sp as u32,
            max_hp: mob_model.hp as u32,
            max_sp: mob_model.sp as u32,
            str: mob_model.str as u16,
            agi: mob_model.agi as u16,
            vit: mob_model.vit as u16,
            int: mob_model.int as u16,
            dex: mob_model.dex as u16,
            luk: mob_model.luk as u16,
            base_atk: mob_model.atk1 as u32,
            matk_min: mob_model.atk1 as u32,
            matk_max: mob_model.atk2 as u32,
            speed: mob_model.speed as u16,
            attack_motion: mob_model.atk_motion as u32,
            attack_delay: mob_model.atk_delay as u32,
            delay_motion: mob_model.damage_motion as u32,
            hit: 0,
            flee: 0,
            crit: 0,
            def: mob_model.def as u32,
            mdef: mob_model.mdef as u32,
            aspd: 0,
            look: None,
            zeny: 0,
        }
    }
}