use std::collections::HashMap;
use std::sync::{Once};
use crate::repository::model::item_model::ItemModel;
use crate::repository::model::mob_model::MobModel;
use crate::server::model::configuration::{Config, JobConfig, SkillConfig};
use crate::server::model::map::Map;

static mut SERVICE_INSTANCE: Option<GlobalConfigService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct GlobalConfigService {
    configuration: Config,
    items: HashMap<u32, ItemModel>,
    items_name_id: HashMap<String, u32>,
    mobs: HashMap<u32, MobModel>,
    mobs_name_id: HashMap<String, u32>,
    jobs: Vec<JobConfig>,
    skills: HashMap<u32, SkillConfig>,
    skills_name_id: HashMap<String, u32>,
    maps: HashMap<String, Map>,
}

impl GlobalConfigService {
    pub fn instance() -> &'static GlobalConfigService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }
    pub unsafe fn instance_mut() -> &'static mut GlobalConfigService {
        SERVICE_INSTANCE.as_mut().unwrap()
    }

    pub fn init(configuration: Config,
                items: HashMap<u32, ItemModel>,
                items_name_id: HashMap<String, u32>,
                mobs: HashMap<u32, MobModel>,
                mobs_name_id: HashMap<String, u32>,
                jobs: Vec<JobConfig>,
                skills: HashMap<u32, SkillConfig>,
                skills_name_id: HashMap<String, u32>,
                maps: HashMap<String, Map>,
    ) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(GlobalConfigService { configuration, items, items_name_id, mobs, mobs_name_id, jobs, skills, skills_name_id, maps });
        });
    }

    pub fn packetver(&self) -> u32 {
        self.configuration.packetver()
    }
    pub fn config(&self) -> &Config {
        &self.configuration
    }
    pub fn set_config(&mut self, config: Config) {
        self.configuration = config;
    }

    pub fn get_job_config(&self, id: u32) -> &JobConfig {
        self.jobs.iter().find(|config| *config.id() == id).unwrap_or_else(|| panic!("Expected to find job config for id {id} but found none"))
    }

    pub fn get_skill_config_by_name(&self, name: &str) -> &SkillConfig {
        let id = self.skills_name_id.get(name).unwrap_or_else(|| panic!("Expected to find skill config for name {name} but found none"));
        self.skills.get(id).unwrap_or_else(|| panic!("Expected to find skill config for name {name} but found none"))
    }

    pub fn get_skill_config(&self, id: u32) -> &SkillConfig {
        self.skills.get(&id).unwrap_or_else(|| panic!("Expected to find skill config for id {id} but found none"))
    }

    pub fn get_item(&self, id: i32) -> &ItemModel {
        self.items.get(&(id as u32)).unwrap_or_else(|| panic!("Expected to find item for id {id} but found none"))
    }

    pub fn get_item_by_name(&self, name: &str) -> &ItemModel {
        let id = &self.get_item_id_from_name(name);
        self.items.get(id).unwrap_or_else(|| panic!("Expected to find item for id {id} but found none"))
    }

    pub fn get_item_id_from_name(&self, name: &str) -> u32 {
        *self.items_name_id.get(name).unwrap_or_else(|| panic!("Expected to find item for name {name} but found none"))
    }

    pub fn get_mob(&self, id: i32) -> &MobModel {
        self.mobs.get(&(id as u32)).unwrap_or_else(|| panic!("Expected to find mob for id {id} but found none"))
    }

    pub fn get_mob_by_name(&self, name: &str) -> &MobModel {
        let id = &self.get_mob_id_from_name(name);
        self.mobs.get(id).unwrap_or_else(|| panic!("Expected to find mob for id {id} but found none"))
    }

    pub fn get_mob_id_from_name(&self, name: &str) -> u32 {
        *self.mobs_name_id.get(name).unwrap_or_else(|| panic!("Expected to find mob for name {name} but found none"))
    }

    pub fn maps(&self) -> &HashMap<String, Map> {
        &self.maps
    }

    pub fn get_map(&self, name: &str) -> &Map {
        self.maps.get(name).unwrap_or_else(|| panic!("Can't find map with name {name}"))
    }
}