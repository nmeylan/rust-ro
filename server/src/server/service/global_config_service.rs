use std::collections::HashMap;
use std::sync::Once;
use crate::repository::model::item_model::ItemModel;
use crate::server::core::configuration::{Config, JobConfig, SkillConfig};

static mut SERVICE_INSTANCE: Option<GlobalConfigService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct GlobalConfigService {
    configuration: &'static Config,
    items: HashMap<u32, ItemModel>,
    jobs: Vec<JobConfig>,
    skills: HashMap<String, SkillConfig>,
    skills_id_name: HashMap<u32, String>,
}

impl GlobalConfigService {
    pub fn instance() -> &'static GlobalConfigService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(configuration: &'static Config,
                items: HashMap<u32, ItemModel>,
                jobs: Vec<JobConfig>,
                skills: HashMap<String, SkillConfig>,
                skills_id_name: HashMap<u32, String>,
    ) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(GlobalConfigService { configuration, items, jobs, skills, skills_id_name });
        });
    }

    pub fn packetver(&self) -> u32 {
        self.configuration.packetver()
    }
    pub fn config(&self) -> &Config {
        self.configuration
    }

    pub fn get_job_config(&self, id: u32) -> &JobConfig {
        self.jobs.iter().find(|config| *config.id() == id).unwrap_or_else(|| panic!("Expected to find job config for id {} but found none", id))
    }

    pub fn get_skill_config(&self, name: &str) -> &SkillConfig {
        self.skills.get(name).unwrap_or_else(|| panic!("Expected to find skill config for name {} but found none", name))
    }

    pub fn get_skill_config_by_id(&self, id: u32) -> &SkillConfig {
        let name = self.skills_id_name.get(&id).unwrap_or_else(|| panic!("Expected to find skill config for id {} but found none", id));
        self.skills.get(name).unwrap_or_else(|| panic!("Expected to find skill config for name {} but found none", name))
    }

    pub fn get_item(&self, id: i32) -> &ItemModel {
        self.items.get(&(id as u32)).unwrap_or_else(|| panic!("Expected to find item for id {} but found none", id))
    }
}