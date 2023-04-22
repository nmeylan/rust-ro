use std::sync::{Arc, Once};
use std::sync::mpsc::SyncSender;
use enums::class::JobName;
use crate::repository::CharacterRepository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::server::state::skill::Skill;
use crate::enums::EnumWithNumberValue;
use crate::server::model::configuration::{JobSkillTree, SkillInTree};


static mut SERVICE_INSTANCE: Option<SkillTreeService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();
pub struct SkillTreeService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
}

impl SkillTreeService {
    pub fn instance() -> &'static SkillTreeService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) -> Self {
        Self { client_notification_sender, configuration_service }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(SkillTreeService { client_notification_sender, configuration_service });
        });
    }

    pub fn skill_tree(&self, character: &Character) -> Vec<Skill> {
        let skilltree = self.configuration_service.get_job_skilltree(JobName::from_value(character.status.job as usize));
        let mut skills = vec![];
        Self::available_skills_in_tree(character, &skilltree.tree(), &mut skills);
        for (_, parent_skills) in skilltree.parent_skills().iter() {
            Self::available_skills_in_tree(character, &parent_skills, &mut skills);
        }
        return skills;
    }

    fn available_skills_in_tree(character: &Character, skilltree: &Vec<SkillInTree>, skills: &mut Vec<Skill>) {
        for skill_in_tree in skilltree.iter() {
            let skill = enums::skills::Skill::from_name(skill_in_tree.name());
            let level = character.skills.iter().find(|s| s.value.id() == skill.id()).map_or(0, |s| s.level);
            if let Some(requirements) = skill_in_tree.requires() {
                let fulfill_requirements = requirements.iter().all(|requirement| {
                    let requirement_skill = enums::skills::Skill::from_name(requirement.name());
                    character.skills.iter().any(|s| { s.value.id() == requirement_skill.id() && s.level >= *requirement.level() })
                });
                if fulfill_requirements {
                    skills.push(Skill { value: skill, level })
                }
                continue;
            }
            if skill.is_platinium() || *skill_in_tree.job_level() as u32 > character.get_job_level() {
                continue;
            }
            skills.push(Skill { value: skill, level });

        }
    }
}

