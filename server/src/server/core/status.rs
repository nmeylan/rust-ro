use crate::repository::model::char_model::CharSelectModel;
use crate::server::configuration::GameConfig;

#[derive(Setters, Debug)]
pub struct Status {
    #[set]
    pub hp: u32,
    #[set]
    pub sp: u32,
    #[set]
    pub max_hp: u32,
    #[set]
    pub max_sp: u32,
    #[set]
    pub str: u16,
    #[set]
    pub agi: u16,
    #[set]
    pub vit: u16,
    #[set]
    pub int: u16,
    #[set]
    pub dex: u16,
    #[set]
    pub luk: u16,
    #[set]
    pub base_atk: u32,
    #[set]
    pub matk_min: u32,
    #[set]
    pub matk_max: u32,
    #[set]
    pub speed: u16,
    #[set]
    pub attack_motion: u32,
    #[set]
    pub attack_delay: u32,
    #[set]
    pub delay_motion: u32,
    #[set]
    pub hit: u32,
    #[set]
    pub flee: u32,
    #[set]
    pub crit: u32,
    #[set]
    pub def: u32,
    #[set]
    pub mdef: u32,
    #[set]
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
}