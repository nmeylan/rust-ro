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
    pub aspd: u32
}

impl Status {
    pub fn default() -> Status {
        Status {
            hp: 100,
            sp: 100,
            max_hp: 100,
            max_sp: 100,
            str: 1,
            agi: 1,
            vit: 1,
            int: 1,
            dex: 1,
            luk: 1,
            base_atk: 1,
            matk_min: 1,
            matk_max: 1,
            speed: 100,
            attack_motion: 1,
            attack_delay: 1,
            delay_motion: 1,
            hit: 1,
            flee: 1,
            crit: 1,
            def: 1,
            mdef: 1,
            aspd: 1
        }
    }
    pub fn from_char_model(char_model: &CharSelectModel, configuration: &GameConfig) -> Status {
        Status {
            hp: char_model.max_hp, // in db current hp are stored in max_hp col, and max_hp in hp col
            sp: char_model.max_sp, // in db current sp are stored in max_sp col, and max_sp in sp col
            max_hp: char_model.hp,
            max_sp: char_model.sp,
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
            aspd: 0
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
            delay_motion: mob_model.defense_motion,
            hit: 0,
            flee: 0,
            crit: 0,
            def: mob_model.def,
            mdef: mob_model.mdef,
            aspd: 0
        }
    }
}