use std::collections::HashMap;
use std::sync::{Once};
use models::enums::class::JobName;
use models::enums::EnumWithStringValue;
use crate::repository::model::item_model::ItemModel;
use crate::repository::model::mob_model::MobModel;
use configuration::configuration::{Config, JobConfig, JobSkillTree, SkillConfig};
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
    jobs_skills_tree: Vec<JobSkillTree>,
    skills: HashMap<u32, SkillConfig>,
    skills_name_id: HashMap<String, u32>,
    pub(crate) maps: HashMap<String, Map>,
}

impl GlobalConfigService {
    pub fn instance() -> &'static GlobalConfigService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }
    pub unsafe fn instance_mut() -> &'static mut GlobalConfigService {
        SERVICE_INSTANCE.as_mut().unwrap()
    }

    pub fn init(configuration: Config,
                items: Vec<ItemModel>,
                mobs: Vec<MobModel>,
                jobs: Vec<JobConfig>,
                jobs_skills_tree: Vec<JobSkillTree>,
                skills: HashMap<u32, SkillConfig>,
                maps: HashMap<String, Map>,
    ) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {

            let mut items_name_id: HashMap<String, u32> = Default::default();
            items.iter().for_each(|item| {
                items_name_id.insert(item.name_aegis.clone(), item.id as u32);
            });

            let mut mobs_name_id: HashMap<String, u32> = Default::default();
            mobs.iter().for_each(|mob| {
                mobs_name_id.insert(mob.name.clone(), mob.id as u32);
            });
            let mut skills_name_id: HashMap<String, u32> = Default::default();
            skills.values().for_each(|skill_config| {
                skills_name_id.insert(skill_config.name().clone(), skill_config.id());
            });
            SERVICE_INSTANCE = Some(GlobalConfigService { configuration,
                items: items.into_iter().map(|item| (item.id as u32, item)).collect(), items_name_id,
                mobs: mobs.into_iter().map(|mob| (mob.id as u32, mob)).collect(), mobs_name_id,
                jobs, jobs_skills_tree,
                skills, skills_name_id, maps });
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
        self.jobs.iter().find(|config| config.id() == id).unwrap_or_else(|| panic!("Expected to find job config for id {id} but found none"))
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
    pub fn get_mob_safe(&self, id: i32) -> Option<&MobModel> {
        self.mobs.get(&(id as u32))
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

    pub fn get_job_skilltree(&self, job: JobName) -> &JobSkillTree {
        self.jobs_skills_tree.iter().find(|tree| tree.name().eq(job.as_str())).unwrap_or_else(|| panic!("Expected to find skill tree for job {}", job.as_str()))
    }
}