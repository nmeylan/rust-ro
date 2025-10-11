use std::sync::mpsc::SyncSender;

use configuration::configuration::SkillInTree;
use models::enums::EnumWithNumberValue;
use models::enums::class::JobName;
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;
use packets::packets::{Packet, PacketZcSkillinfoList, SKILLINFO};

use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::util::string::StringUtil;

pub struct SkillTreeService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
}

impl SkillTreeService {
    pub fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService) -> Self {
        Self {
            client_notification_sender,
            configuration_service,
        }
    }

    pub fn skill_tree(&self, character: &Character) -> Vec<KnownSkill> {
        let skilltree = self
            .configuration_service
            .get_job_skilltree(JobName::from_value(character.status.job as usize));
        let mut skills = vec![];

        let maybe_novice_basic = character.status.known_skills.iter().find(|s| s.value == SkillEnum::NvBasic);
        let mut platinium_novice_skills = character
            .status
            .known_skills
            .iter()
            .filter(|s| s.value.to_name().starts_with("NV") && s.value.is_platinium())
            .cloned()
            .collect::<Vec<KnownSkill>>();
        if maybe_novice_basic.is_none() {
            platinium_novice_skills.extend(vec![KnownSkill {
                value: SkillEnum::NvBasic,
                level: 0,
            }]);
            return platinium_novice_skills;
        } else if maybe_novice_basic.unwrap().level < 9 {
            platinium_novice_skills.extend(vec![KnownSkill {
                value: SkillEnum::NvBasic,
                level: maybe_novice_basic.unwrap().level,
            }]);
            return platinium_novice_skills;
        }
        Self::available_skills_in_tree(character, skilltree.tree(), &mut skills);
        for (_, parent_skills) in skilltree.parent_skills().iter() {
            Self::available_skills_in_tree(character, parent_skills, &mut skills);
        }
        skills
    }

    pub fn send_skill_tree(&self, character: &Character) {
        let skills = self.skill_tree(character);
        let skills_info: Vec<SKILLINFO> = skills
            .iter()
            .filter_map(|skill| {
                let skill_enum = skill.value;
                let know_level = skill.level;
                if let Some(skill) = skills::skill_enums::to_object(skill_enum, know_level) {
                    let mut skill_info = SKILLINFO::new(self.configuration_service.packetver());
                    skill_info.set_skid(skill.id() as i16);
                    skill_info.set_atype(skill.client_type() as i32);
                    skill_info.set_level(skill.level() as i16);

                    skill_info.set_spcost(skill.sp_cost() as i16);
                    skill_info.set_attack_range(skill.range().abs() as i16);
                    let mut skill_name: [char; 24] = [0 as char; 24];
                    skill_enum.to_name().fill_char_array(&mut skill_name);
                    skill_info.set_skill_name(skill_name);
                    let mut is_upgradable = 0_i8;
                    if !skill_enum.is_platinium() {
                        is_upgradable = if skill.level() < skill.max_level() { 1 } else { 0 };
                    }
                    skill_info.set_upgradable(is_upgradable);
                    return Some(skill_info);
                } else {
                    error!("No skill defined for enum {}", skill_enum.to_name());
                }
                None
            })
            .collect::<Vec<SKILLINFO>>();
        let mut packet_zc_skillinfo_list = PacketZcSkillinfoList::new(self.configuration_service.packetver());
        packet_zc_skillinfo_list.set_packet_length(
            (PacketZcSkillinfoList::base_len(self.configuration_service.packetver())
                + (skills_info.len() * SKILLINFO::base_len(self.configuration_service.packetver()))) as i16,
        );
        packet_zc_skillinfo_list.set_skill_list(skills_info);
        packet_zc_skillinfo_list.fill_raw();
        self.client_notification_sender
            .send(Notification::Char(CharNotification::new(
                character.char_id,
                packet_zc_skillinfo_list.raw,
            )))
            .unwrap_or_else(|_| error!("Failed to send notification packet_zc_skillinfo_list to client"));
    }

    fn available_skills_in_tree(character: &Character, skilltree: &Vec<SkillInTree>, skills: &mut Vec<KnownSkill>) {
        for skill_in_tree in skilltree.iter() {
            let skill = SkillEnum::from_name(skill_in_tree.name());
            let level = character
                .status
                .known_skills
                .iter()
                .find(|s| s.value.id() == skill.id())
                .map_or(0, |s| s.level);
            if let Some(requirements) = skill_in_tree.requires() {
                let fulfill_requirements = requirements.iter().all(|requirement| {
                    let requirement_skill = SkillEnum::from_name(requirement.name());
                    character
                        .status
                        .known_skills
                        .iter()
                        .any(|s| s.value.id() == requirement_skill.id() && s.level >= requirement.level())
                });
                if fulfill_requirements {
                    skills.push(KnownSkill { value: skill, level })
                }
                continue;
            }
            if skill.is_platinium() || skill_in_tree.job_level() as u32 > character.get_job_level() {
                continue;
            }
            skills.push(KnownSkill { value: skill, level });
        }
    }
}
