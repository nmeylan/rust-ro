use std::collections::HashMap;

static PARALLEL_EXECUTIONS: usize = 100; // TODO add a conf for this
static MOB_CONF_PATH: &str = "./npc/scripts_monsters.conf";

#[derive(Setters, Clone, Debug)]
pub struct MobSpawn {
    #[set]
    pub map_name: String,
    #[set]
    pub name: String,
    #[set]
    pub mob_id: u16,
    #[set]
    pub to_spawn: i16,
    #[set]
    pub spawned: i16,
    #[set]
    pub id: u32,
    #[set]
    pub x: u16,
    #[set]
    pub y: u16,
    #[set]
    pub x_size: u16,
    #[set]
    pub y_size: u16,
    #[set]
    pub mob_type: MobType,
    #[set]
    pub fixed_delay_in_ms: u32,
    #[set]
    pub random_variance_delay_in_ms: u32,
}

#[derive(Clone, Debug)]
pub enum MobType {
    Monster,
    Miniboss,
    MVP
}

impl MobSpawn {
    pub fn new() -> MobSpawn {
        MobSpawn {
            map_name: "".to_string(),
            name: "".to_string(),
            mob_id: 0,
            to_spawn: 0,
            spawned: 0,
            id: 0,
            x: 0,
            y: 0,
            x_size: 0,
            y_size: 0,
            mob_type: MobType::Monster,
            fixed_delay_in_ms: 0,
            random_variance_delay_in_ms: 0
        }
    }

}