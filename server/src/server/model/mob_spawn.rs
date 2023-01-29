use crate::repository::model::mob_model::MobModel;
use crate::server::model::Npc;

#[derive(SettersAll, Clone, Debug)]
pub struct MobSpawn {
    pub map_name: String,
    pub name: String,
    pub mob_id: i16,
    pub to_spawn_amount: i16,
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub x_size: u16,
    pub y_size: u16,
    pub mob_type: MobType,
    pub fixed_delay_in_ms: u32,
    pub random_variance_delay_in_ms: u32,
    pub level: u16,
    pub info: MobModel,
}

#[derive(Clone, Debug)]
pub enum MobType {
    Monster,
    Miniboss,
    Mvp
}

impl MobType {
    pub fn from(string: &str) -> MobType {
        if string == "boss_monster" {
            MobType::Mvp
        } else if string == "miniboss_monster" {
            MobType::Miniboss
        } else {
            MobType::Monster
        }
    }
}

impl MobSpawn {
    pub fn default() -> MobSpawn {
        MobSpawn {
            map_name: "".to_string(),
            name: "".to_string(),
            mob_id: 0,
            to_spawn_amount: 0,
            id: 0,
            x: 0,
            y: 0,
            level: 0,
            x_size: 0,
            y_size: 0,
            mob_type: MobType::Monster,
            fixed_delay_in_ms: 0,
            random_variance_delay_in_ms: 0,
            info: Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn is_fixed_position(&self) -> bool {
        self.x != 0 && self.y != 0
    }
    #[allow(dead_code)]
    pub fn is_zone_constraint(&self) -> bool {
        self.x_size != 0 || self.y_size != 0
    }
    #[allow(dead_code)]
    pub fn has_delay(&self) -> bool {
        self.fixed_delay_in_ms != 0
    }
    #[allow(dead_code)]
    pub fn has_delay_variance(&self) -> bool {
        self.random_variance_delay_in_ms != 0
    }
}

impl Npc for MobSpawn {
    fn get_map_name(&self) -> String {
        self.map_name.clone()
    }
}