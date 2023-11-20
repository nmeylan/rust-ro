use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;

use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct SkillTreeServiceTestContext {
    test_context: TestContext,
    skill_tree_service: SkillTreeService,
}

fn before_each() -> SkillTreeServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> SkillTreeServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    SkillTreeServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        skill_tree_service: SkillTreeService::new(client_notification_sender, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    use enums::class::JobName;
    use crate::enums::EnumWithNumberValue;
    use crate::enums::EnumWithStringValue;
    use crate::server::state::skill::KnownSkill;
    use crate::tests::common::character_helper::create_character;
    use crate::tests::skill_tree_service_test::before_each;

    #[test]
    fn test_skilllist_should_return_list_of_skill_for_character_job() {
        // Given
        let context = before_each();
        struct Scenarii {
            job: usize,
            job_level: u32,
            skills: Vec<KnownSkill>,
            expected_skills: Vec<KnownSkill>,
        }
        // var a = ""; temp0.skillList.forEach((s) => a += "Skill {value: skills::skill_enums::Skill::from_name(\""+s.skillName+"\"), level:"+ s.level+"},\n"); console.log(a)
        let scenario = vec![
            Scenarii { job: JobName::Novice.value(), job_level: 1, skills: vec![], expected_skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 0 }] },
            Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 8 }],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 8 }, ],
            },
            Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 8 }, KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_TRICKDEAD"), level: 1 }],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 8 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_TRICKDEAD"), level: 1 }, ],
            },
            Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 }],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 0 }, ],
            }, Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 6 }, ],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 6 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 0 }, ],
            }, Scenarii {
                job: JobName::SwordsmanHigh.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 6 }, ],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 6 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 0 }, ],
            }, Scenarii {
                job: JobName::Knight.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_ENDURE"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_PIERCE"), level: 3 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_RIDING"), level: 1 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 }, ],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 7 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_ENDURE"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_PIERCE"), level: 3 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARBOOMERANG"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_RIDING"), level: 1 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 }, ],
            }, Scenarii {
                job: JobName::LordKnight.value(),
                job_level: 1,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 6 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_ENDURE"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_PIERCE"), level: 3 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_RIDING"), level: 1 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 }, ],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 6 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 7 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_ENDURE"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_PIERCE"), level: 3 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARBOOMERANG"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_RIDING"), level: 1 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("LK_AURABLADE"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("LK_PARRYING"), level: 0 }, ],
            },
            Scenarii {
                job: JobName::LordKnight.value(),
                job_level: 50,
                skills: vec![KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 6 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_ENDURE"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_PIERCE"), level: 3 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_RIDING"), level: 1 },
                             KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 }, ],
                expected_skills: vec![
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("NV_BASIC"), level: 9 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_SWORD"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_RECOVERY"), level: 6 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_BASH"), level: 7 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("SM_ENDURE"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_PIERCE"), level: 3 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_SPEARBOOMERANG"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("LK_BERSERK"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_RIDING"), level: 1 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("LK_AURABLADE"), level: 0 },
                    KnownSkill { value: enums::skill_enums::SkillEnum::from_name("LK_PARRYING"), level: 0 }, ],
            },
        ];
        let mut i = 0;
        for scenarii in scenario {
            // Given
            let mut character = create_character();
            character.status.job_level = scenarii.job_level;
            character.status.job = scenarii.job as u32;
            character.known_skills = scenarii.skills;
            // When
            let skills = context.skill_tree_service.skill_tree(&character);
            // Then
            let skills_list_str = skills.iter().fold(String::new(), |memo, skill| format!("{}\n Skill {} - lvl {}", memo, skill.value.to_name(), skill.level));
            assert_eq!(skills.len(), scenarii.expected_skills.len(), "Scenarii {} - Expected job {} to have {} skills but got {}. Received list was \n{}", i, JobName::from_value(character.status.job as usize).as_str(), scenarii.expected_skills.len(), skills.len(), skills_list_str);
            for skill in scenarii.expected_skills {
                assert!(skills.contains(&skill), "Scenarii {} - Expected {} to be included among following list but was not \n{}", i, skill.value.to_name(), skills_list_str)
            }
            i += 1;
        }
    }
}