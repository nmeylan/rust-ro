use std::{fs};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::slice::Iter;
use regex_lite::Regex;
use serde::{Deserialize, Serialize};
use configuration::configuration::{BonusPerLevel, JobSkillTree, SkillConfig, SkillsConfig};
use models::enums::{EnumWithMaskValueU16, EnumWithMaskValueU64, EnumWithNumberValue, EnumWithStringValue};
use models::enums::bonus::BonusType;
use models::enums::element::Element;
use models::enums::skill::{SkillTargetType, SkillFlags, SkillType, SkillDamageFlags};
use models::enums::weapon::WeaponType;
use models::status::KnownSkill;
use models::status_bonus::StatusBonusFlag;

lazy_static! {
    pub static ref SHORT_CLASS_NAME: BTreeMap<&'static str, &'static str> = BTreeMap::from([
        ("novice", "nv"),
        ("swordsman", "sm"),
        ("acolyte", "al"),
        ("magician", "mg"),
        ("mage", "mg"),
        ("merchant", "mc"),
        ("archer", "ac"),
        ("thief", "tf"),
        ("knight", "kn"),
        ("priest", "pr"),
        ("wizard", "wz"),
        ("blacksmith", "bs"),
        ("hunter", "ht"),
        ("assassin", "as"),
        ("rogue", "rg"),
        ("crusader", "cr"),
        ("monk", "mo"),
        ("sage", "sa"),
        ("alchemist", "am"),
        ("bard", "ba"),
        ("dancer", "dc"),
        ("lordknight", "lk"),
        ("highpriest", "hp"),
        ("champion", "ch"),
        ("paladin", "pa"),
        ("assassincross", "asc"),
        ("sniper", "sn"),
        ("stalker", "st"),
        ("professor", "pf"),
        ("whitesmith", "ws"),
        ("clown", "cg"),
        ("creator", "cr"),
        ("highwizard", "hw"),
        ("taekwon", "tk"),
        ("stargladiator", "sg"),
        ("soullinker", "sl"),
        ("gunslinger", "gs"),
        ("ninja", "nj"),
    ]);
     static ref NON_ALPHA_REGEX: Regex = Regex::new(r"[^A-Za-z0-9]*").unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemModels {
    items: Vec<ItemModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ItemModel {
    pub id: i32,
    pub name_aegis: String,
}

impl From<Vec<ItemModel>> for ItemModels {
    fn from(items: Vec<ItemModel>) -> Self {
        ItemModels {
            items
        }
    }
}

impl From<ItemModels> for Vec<ItemModel> {
    fn from(item_models: ItemModels) -> Self {
        item_models.items
    }
}

pub fn main() {
    let path = Path::new("./config/skill.json");
    let skill_tree_path = Path::new("./config/skill_tree.json");
    let output_path = Path::new("lib/skills/src");
    let output_enum_path = Path::new("lib/models/src/enums");
    if !path.exists() {
        panic!("config/skill.json file does not exists at {}", path.to_str().unwrap());
    }
    if !skill_tree_path.exists() {
        panic!("config/skill_tree.json file does not exists at {}", skill_tree_path.to_str().unwrap());
    }

    let json = fs::read_to_string(path).unwrap();
    let mut config_deserializer = serde_json::Deserializer::from_str(&json);
    let result: Result<SkillsConfig, _> = serde_path_to_error::deserialize(&mut config_deserializer);
    match result {
        Err(err) => {
            let path = err.path().to_string();
            println!("Path in error {}", path);
            panic!("{}", err);
        }
        _ => {}
    }
    let skills_config: SkillsConfig = result.unwrap();
    let skill_tree: Vec<JobSkillTree> = configuration::configuration::Config::load_jobs_skill_tree(".").unwrap();
    let mut vec: Vec<(u32, SkillConfig)> = skills_config.skills.into_iter().collect::<Vec<(u32, SkillConfig)>>();
    vec.sort_by_key(|&(k, _)| k);
    let skills: Vec<SkillConfig> = vec.into_iter().map(|(_, v)| v).collect();

    let item_models = serde_json::from_str::<ItemModels>(&fs::read_to_string("./config/items.json").unwrap());
    let items: Vec<ItemModel> = item_models.unwrap().into();
    let mut items_name_id: BTreeMap<String, u32> = Default::default();
    items.iter().for_each(|item| {
        items_name_id.insert(item.name_aegis.clone(), item.id as u32);
    });

    let mut skills_already_generated: BTreeSet<String> = BTreeSet::new();
    let mut jobs_with_skills: BTreeSet<String> = BTreeSet::new();

    generate_skills_impl(output_path, &skills, &skill_tree, &mut skills_already_generated, &mut jobs_with_skills, &items_name_id);
    generate_skills_enum(output_enum_path, &skills, &skill_tree, &skills_already_generated, &jobs_with_skills);
    generate_skills_enum_to_object(output_path, &skills, &skill_tree, &skills_already_generated, &jobs_with_skills);
}

fn generate_skills_impl(output_path: &Path, skills: &Vec<SkillConfig>, skill_tree: &Vec<JobSkillTree>, skills_already_generated: &mut BTreeSet<String>, jobs_with_skills: &mut BTreeSet<String>, item_name_ids: &BTreeMap<String, u32>) {
    let file_path_base = output_path.join("base").join("mod.rs");
    if !output_path.join("base").exists() {
        std::fs::create_dir(output_path.join("base")).unwrap();
    }
    let mut mod_file_base = File::create(file_path_base.clone()).unwrap();

    mod_file_base.write_all("// Generated by tools/skills/main.rs\n".to_string().as_bytes()).unwrap();
    mod_file_base.write_all("// Auto generated file do not edit manually\n\n".to_string().as_bytes()).unwrap();
    mod_file_base.write_all("#![allow(dead_code)]\n\n".to_string().as_bytes()).unwrap();

    #[cfg(feature = "generate_override_stub")]
    let mut mod_file = {
        let file_path = output_path.join("skills").join("mod.rs");
        if !output_path.join("skills").exists() {
            std::fs::create_dir(output_path.join("skills")).unwrap();
        }
        let mut mod_file = File::create(file_path.clone()).unwrap();
        mod_file.write_all("// Generated by tools/skills/main.rs\n".to_string().as_bytes()).unwrap();
        mod_file.write_all("// Auto generated file do not edit manually\n\n".to_string().as_bytes()).unwrap();
        mod_file.write_all("#![allow(dead_code)]\n\n".to_string().as_bytes()).unwrap();
        mod_file
    };

    for skill in skills.iter() {
        mod_file_base.write_all(format!("pub struct {};\n", to_enum_name(skill)).as_bytes()).unwrap();
    }

    for job_tree in skill_tree.iter() {
        if job_tree.tree().is_empty() {
            continue;
        }
        // Base
        let file_base_name = format!("{}_base", job_tree.name().to_lowercase().replace(' ', ""));
        mod_file_base.write_all(format!("pub mod {};\n", file_base_name).as_bytes()).unwrap();
        let mut job_skills_file_base = File::create(output_path.join("base").join(format!("{}.rs", file_base_name))).unwrap();

        let file_name = job_tree.name().to_lowercase().replace(' ', "").to_string();
        #[cfg(feature = "generate_override_stub")]
        let mut job_skills_file = {
            // // Override stub
            mod_file.write_all(format!("pub mod {};\n", file_name).as_bytes()).unwrap();
            let mut job_skills_file = File::create(output_path.join("skills").join(format!("{}.rs", file_name))).unwrap();
            job_skills_file.write_all("#![allow(unused_imports)]\n\n".to_string().as_bytes()).unwrap();
            job_skills_file.write_all(b"\nuse crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, InteractiveSkill};\n\n").unwrap();
            job_skills_file.write_all(format!("\nuse crate::base::{}::{{*}};\n\n", file_base_name).as_bytes()).unwrap();
            job_skills_file
        };

        write_file_header(&mut job_skills_file_base);
        for skill in job_tree.tree().iter() {
            if skills_already_generated.contains(skill.name()) {
                continue;
            }

            if let Some(job) = SHORT_CLASS_NAME.get(job_tree.name().to_lowercase().as_str()) {
                if (!skill.name().to_lowercase().starts_with(job) && !job_tree.name().to_lowercase().eq("bard")) || (job_tree.name().to_lowercase().eq("bard") && !(skill.name().to_lowercase().starts_with("bd") || skill.name().to_lowercase().starts_with("ba"))) {
                    println!("Skipping skill {} for job {}", skill.name(), job_tree.name());
                    continue;
                }
                let skill_config = get_skill_config(skill.name(), skills).expect(format!("Expected to find a skills with name [{}]", skill.name()).as_str());
                write_skills(&mut job_skills_file_base, skill_config, item_name_ids);
                #[cfg(feature = "generate_override_stub")] {
                    job_skills_file.write_all(format!("impl Skill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                    generate_new(&mut job_skills_file, skill_config);
                    job_skills_file.write_all(b"}\n").unwrap();
                    if is_offensive(skill_config) {
                        job_skills_file.write_all(format!("impl OffensiveSkill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                        job_skills_file.write_all(b"}\n").unwrap();
                    }
                    if is_support(skill_config) {
                        job_skills_file.write_all(format!("impl SupportiveSkill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                        job_skills_file.write_all(b"}\n").unwrap();
                    }
                    if is_interactive(skill_config) {
                        job_skills_file.write_all(format!("impl InteractiveSkill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                        job_skills_file.write_all(b"}\n").unwrap();
                    }
                    if is_ground(skill_config) {
                        job_skills_file.write_all(format!("impl GroundSkill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                        job_skills_file.write_all(b"}\n").unwrap();
                    }
                    if is_performance(skill_config) {
                        job_skills_file.write_all(format!("impl PerformanceSkill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                        job_skills_file.write_all(b"}\n").unwrap();
                    }
                    if is_passive(skill_config) {
                        job_skills_file.write_all(format!("impl PassiveSkill for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
                        job_skills_file.write_all(b"}\n").unwrap();
                    }
                }

                skills_already_generated.insert(skill.name().clone());
                jobs_with_skills.insert(file_name.clone());
            }
        }
    }

    // NPC and third classes
    // file.write_all(b"pub mod any;").unwrap();
    // let mut file = File::create(output_path.join("skills").join("any.rs")).unwrap();
    // write_file_header(&mut file);
    // for skill_config in skills.iter() {
    //     if skills_already_generated.contains(skill_config.name()) {
    //         continue;
    //     }
    //     write_skills(&mut file, skill_config);
    // }
}

fn write_file_header(file: &mut File) {
    write_file_header_comments(file);
    file.write_all(b"use models::enums::{*};\n").unwrap();
    file.write_all(b"use models::enums::skill::*;\n").unwrap();
    file.write_all(b"use models::enums::weapon::AmmoType;\n").unwrap();
    file.write_all(b"use models::enums::element::Element::{*};\n").unwrap();
    file.write_all(b"\nuse models::item::WearWeapon;\n").unwrap();
    file.write_all(b"\nuse models::status::StatusSnapshot;\n").unwrap();
    file.write_all(b"use models::item::NormalInventoryItem;\n").unwrap();
    file.write_all(b"use models::enums::weapon::WeaponType::{*};\n").unwrap();
    file.write_all(b"use models::enums::bonus::{BonusType};\n").unwrap();
    file.write_all(b"use models::enums::status::StatusEffect::{*};\n").unwrap();
    file.write_all(b"use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonus};\n").unwrap();
    file.write_all(b"use models::enums::mob::MobRace::{*};\n").unwrap();
    file.write_all(b"\nuse crate::{*};\n\n").unwrap();
    file.write_all(b"use crate::base::*;\n").unwrap();
    file.write_all(b"use std::any::Any;\n").unwrap();
}

fn write_file_header_comments(file: &mut File) {
    file.write_all("// Generated by tools/skills/main.rs\n".to_string().as_bytes()).unwrap();
    file.write_all("// Auto generated file do not edit manually\n\n".to_string().as_bytes()).unwrap();
    file.write_all("#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]\n\n".to_string().as_bytes()).unwrap();
}

fn write_skills(job_skills_file: &mut File, skill_config: &SkillConfig, item_name_ids: &BTreeMap<String, u32>) {
    job_skills_file.write_all(format!("// {} - {}\n", skill_config.name, skill_config.description).as_bytes()).unwrap();

    generate_struct(job_skills_file, skill_config);

    job_skills_file.write_all(format!("impl SkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
    generate_as_any(job_skills_file);
    generate_getters(job_skills_file, skill_config);

    // Validation methods
    generate_validate_sp(job_skills_file, skill_config);
    generate_validate_hp(job_skills_file, skill_config);
    generate_validate_ammo(job_skills_file, skill_config);
    generate_validate_state(job_skills_file, skill_config);
    generate_validate_zeny(job_skills_file, skill_config);
    generate_validate_item(job_skills_file, skill_config, item_name_ids);
    generate_validate_weapon(job_skills_file, skill_config);
    generate_skip_validation_item(job_skills_file, skill_config);

    // Delays methods
    generate_base_cast_time(job_skills_file, skill_config);
    generate_base_after_cast_act_delay(job_skills_file, skill_config);
    generate_base_after_cast_walk_delay(job_skills_file, skill_config);
    generate_bonuses(job_skills_file, skill_config);

    if is_offensive(skill_config) {
        generate_is_offensive_skill(job_skills_file);
        generate_as_offensive_skill(job_skills_file);
    }
    if is_support(skill_config) {
        generate_is_supportive_skill(job_skills_file);
        generate_as_supportive_skill(job_skills_file);
        generate_client_type(job_skills_file, skill_config);
    }
    if is_interactive(skill_config) {
        generate_is_interactive_skill(job_skills_file);
        generate_as_interactive_skill(job_skills_file);
    }
    if is_ground(skill_config) {
        generate_is_ground_skill(job_skills_file);
        generate_as_ground_skill(job_skills_file);
    }
    if is_performance(skill_config) {
        generate_is_performance_skill(job_skills_file);
        generate_as_performance_skill(job_skills_file);
    }
    if is_passive(skill_config) {
        generate_is_passive_skill(job_skills_file);
        generate_as_passive_skill(job_skills_file);
    }
    job_skills_file.write_all(b"}\n").unwrap();


    // Offensive skills
    if is_offensive(skill_config) {
        job_skills_file.write_all(format!("impl OffensiveSkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
        generate_hit_count(job_skills_file, skill_config);
        generate_dmg_atk(job_skills_file, skill_config);
        generate_dmg_matk(job_skills_file, skill_config);
        generate_element(job_skills_file, skill_config);
        generate_inflict_status(job_skills_file, skill_config);
        job_skills_file.write_all(b"}\n").unwrap();
    }
    if is_support(skill_config) {
        job_skills_file.write_all(format!("impl SupportiveSkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
        job_skills_file.write_all(b"}\n").unwrap();
    }
    if is_interactive(skill_config) {
        job_skills_file.write_all(format!("impl InteractiveSkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
        job_skills_file.write_all(b"}\n").unwrap();
    }
    if is_ground(skill_config) {
        job_skills_file.write_all(format!("impl GroundSkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
        job_skills_file.write_all(b"}\n").unwrap();
    }
    if is_performance(skill_config) {
        job_skills_file.write_all(format!("impl PerformanceSkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
        job_skills_file.write_all(b"}\n").unwrap();
    }
    if is_passive(skill_config) {
        job_skills_file.write_all(format!("impl PassiveSkillBase for {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
        job_skills_file.write_all(b"}\n").unwrap();
    }
}
/*
*   Skills structure
*
*/
fn generate_struct(job_skills_file: &mut File, skill_config: &SkillConfig) {
    job_skills_file.write_all(format!("pub struct {} {{\n", to_struct_name(skill_config)).as_bytes()).unwrap();
    job_skills_file.write_all(b"    pub(crate) level: u8,\n").unwrap();
    job_skills_file.write_all(b"    pub(crate) cast_time: u32,\n").unwrap();
    job_skills_file.write_all(b"    pub(crate) after_cast_act_delay: u32,\n").unwrap();
    job_skills_file.write_all(b"    pub(crate) after_cast_walk_delay: u32,\n").unwrap();
    job_skills_file.write_all(b"}\n").unwrap();
}

fn generate_new(job_skills_file: &mut File, skill_config: &SkillConfig) {
    job_skills_file.write_all(b"    fn new(level: u8) -> Option<Self> where Self : Sized {\n").unwrap();
    job_skills_file.write_all(format!("        if level > {} {{ return None }}\n", skill_config.max_level()).as_bytes()).unwrap();
    job_skills_file.write_all(b"        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_getters(job_skills_file: &mut File, skill_config: &SkillConfig) {
    job_skills_file.write_all(b"    fn _id(&self) -> u32 {\n").unwrap();
    job_skills_file.write_all(format!("        {}\n", skill_config.id).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    fn skill_type(&self) -> SkillType {\n").unwrap();
    job_skills_file.write_all(format!("        SkillType::{:?}\n", skill_config.skill_type().expect(format!("Expected a skill type for skill {}", skill_config.name()).as_str())).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    fn _level(&self) -> u8 {\n").unwrap();
    job_skills_file.write_all(b"        self.level\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _cast_time(&self) -> u32 {\n").unwrap();
    job_skills_file.write_all(b"        self.cast_time\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _after_cast_act_delay(&self) -> u32 {\n").unwrap();
    job_skills_file.write_all(b"        self.after_cast_act_delay\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _after_cast_walk_delay(&self) -> u32 {\n").unwrap();
    job_skills_file.write_all(b"        self.after_cast_walk_delay\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();


    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _update_cast_time(&mut self, new_value: u32) {\n").unwrap();
    job_skills_file.write_all(b"        self.cast_time = new_value;\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _update_after_cast_act_delay(&mut self, new_value: u32) {\n").unwrap();
    job_skills_file.write_all(b"        self.after_cast_act_delay = new_value;\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {\n").unwrap();
    job_skills_file.write_all(b"        self.after_cast_walk_delay = new_value;\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();

    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _range(&self) -> i8 {\n").unwrap();
    if skill_config.range().is_none() {
        job_skills_file.write_all(b"        0\n").unwrap();
    } else {
        generate_return_per_level_i32(job_skills_file, skill_config.range(), skill_config.range_per_level());
    }
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    fn _is_ranged(&self) -> bool {\n").unwrap();
    if skill_config.range().is_none() {
        job_skills_file.write_all(b"        false\n").unwrap();
    } else if let Some(range) = skill_config.range() {
        if *range < -1 || *range > 1 {
            job_skills_file.write_all(b"        true\n").unwrap();
        } else {
            job_skills_file.write_all(b"        false\n").unwrap();
        }
    } else if skill_config.range_per_level().is_some() {
        job_skills_file.write_all(b"        true\n").unwrap();
    } else {
        job_skills_file.write_all(b"        false\n").unwrap();
    }
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _max_level(&self) -> u8 {\n").unwrap();
    job_skills_file.write_all(format!("        {}\n", skill_config.max_level()).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _sp_cost(&self) -> u16 {\n").unwrap();
    let requirements = skill_config.requires().as_ref();
    if requirements.is_none() || (requirements.unwrap().sp_cost().is_none() && requirements.unwrap().sp_cost_per_level().is_none()) {
        job_skills_file.write_all(b"        0\n").unwrap();
    } else {
        generate_return_per_level_u32(job_skills_file, requirements.map(|c| c.sp_cost()).unwrap_or(&None), requirements.map(|c| c.sp_cost_per_level()).unwrap_or(&None));
    }

    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    fn _target_type(&self) -> SkillTargetType {\n").unwrap();
    job_skills_file.write_all(format!("        SkillTargetType::{:?}\n", skill_config.target_type()).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    fn _is_magic(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(format!("        {}\n", skill_config.damage_flags().unwrap_or(0) & SkillDamageFlags::IsMagical.as_flag() > 0).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
    job_skills_file.write_all(b"    fn _is_physical(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(format!("        {}\n", skill_config.damage_flags().unwrap_or(0) & SkillDamageFlags::IsPhysical.as_flag() > 0).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

/*
*   Skills validation methods
*
*/
fn generate_validate_sp(job_skills_file: &mut File, skill_config: &SkillConfig) {
    let requirements = skill_config.requires().as_ref();
    if requirements.is_none() || (requirements.unwrap().sp_cost().is_none() && requirements.unwrap().sp_cost_per_level().is_none()) {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _validate_sp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {\n").unwrap();
    let requirements = skill_config.requires().as_ref();
    generate_validate_per_level(job_skills_file, "status.sp()", requirements.map(|c| c.sp_cost()).unwrap_or(&None), requirements.map(|c| c.sp_cost_per_level()).unwrap_or(&None));
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_validate_hp(job_skills_file: &mut File, skill_config: &SkillConfig) {
    let requirements = skill_config.requires().as_ref();
    if requirements.is_none() || (requirements.unwrap().hp_cost().is_none() && requirements.unwrap().hp_cost_per_level().is_none()) {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _validate_hp(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {\n").unwrap();
    let requirements = skill_config.requires().as_ref();
    generate_validate_per_level(job_skills_file, "status.hp()", requirements.map(|c| c.hp_cost()).unwrap_or(&None), requirements.map(|c| c.hp_cost_per_level()).unwrap_or(&None));
    job_skills_file.write_all(b"    }\n").unwrap();
}


fn generate_validate_ammo(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if let Some(requirements) = skill_config.requires() {
        if let Some(ammo_amount) = requirements.ammo_amount() {
            if let Some(ammo_flags) = requirements.ammo_flags() {
                job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
                job_skills_file.write_all(b"    fn _validate_ammo(&self, character_ammo: Option<(AmmoType, u32)>) -> SkillRequirementResult<u32> {\n").unwrap();
                job_skills_file.write_all(b"        if let Some(ammo_and_amount) = character_ammo {\n").unwrap();
                job_skills_file.write_all(format!("            if ammo_and_amount.1 >= {} && ({} & ammo_and_amount.0.as_flag()) > 0 {{ Ok({}) }} else {{ Err(()) }}\n", ammo_amount, ammo_flags, ammo_amount).as_bytes()).unwrap();
                job_skills_file.write_all(b"        } else {\n").unwrap();
                job_skills_file.write_all(b"            Err(())\n").unwrap();
                job_skills_file.write_all(b"        }\n").unwrap();
                job_skills_file.write_all(b"    }\n").unwrap();
            } else {
                panic!("Skill ({}) configuration has ammo amount requirement but no ammo flags", skill_config.name);
            }
        }
    }
}

fn generate_validate_state(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if let Some(requirements) = skill_config.requires() {
        if let Some(state) = requirements.state() {
            job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
            job_skills_file.write_all(b"    fn _validate_state(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {\n").unwrap();
            job_skills_file.write_all(b"        if status.state() > 0 {\n").unwrap();
            job_skills_file.write_all(format!("            // {}\n", state.as_str()).as_bytes()).unwrap();
            job_skills_file.write_all(format!("            if status.state() & {} > 0 {{ Ok(()) }} else {{ Err(()) }}\n", state.as_flag()).as_bytes()).unwrap();
            job_skills_file.write_all(b"        } else {\n").unwrap();
            job_skills_file.write_all(b"            Err(())\n").unwrap();
            job_skills_file.write_all(b"        }\n").unwrap();
            job_skills_file.write_all(b"    }\n").unwrap();
        }
    }
}

fn generate_validate_zeny(job_skills_file: &mut File, skill_config: &SkillConfig) {
    let requirements = skill_config.requires().as_ref();
    if requirements.is_none() || (requirements.unwrap().zeny_cost().is_none() && requirements.unwrap().zeny_cost_per_level().is_none()) {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _validate_zeny(&self, status: &StatusSnapshot) -> SkillRequirementResult<u32> {\n").unwrap();
    generate_validate_per_level(job_skills_file, "status.zeny()", requirements.map(|c| c.zeny_cost()).unwrap_or(&None), requirements.map(|c| c.zeny_cost_per_level()).unwrap_or(&None));
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_validate_item(job_skills_file: &mut File, skill_config: &SkillConfig, item_name_ids: &BTreeMap<String, u32>) {
    if let Some(requirements) = skill_config.requires() {
        if !requirements.item_cost().is_empty() {
            job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
            job_skills_file.write_all(b"    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {\n").unwrap();
            job_skills_file.write_all(format!("        let required_items = vec![{}]; \n", requirements.item_cost().iter()
                .map(|item| format!("(NormalInventoryItem {{item_id: {}, name_english: \"{}\".to_string(), amount: {}}})", item_name_ids.get(item.item()).unwrap_or_else(|| panic!("Item {} not found", item.item())), item.item(), item.amount())).collect::<Vec<String>>().join(",")).as_bytes()).unwrap();
            for item in requirements.item_cost().iter() {
                job_skills_file.write_all(format!("        if !inventory.iter().any(|item| item.item_id == {} && item.amount >= {}) {{\n", item_name_ids.get(item.item()).unwrap(), item.amount()).as_bytes()).unwrap();
                if item.item().eq("Red_Gemstone") {
                    job_skills_file.write_all(b"            return Err(UseSkillFailure::RedGemstone);\n").unwrap();
                } else if item.item().eq("Blue_Gemstone") {
                    job_skills_file.write_all(b"            return Err(UseSkillFailure::BlueGemstone);\n").unwrap();
                } else if item.item().eq("Holy_Water") {
                    job_skills_file.write_all(b"            return Err(UseSkillFailure::Holywater);\n").unwrap();
                } else {
                    job_skills_file.write_all(b"            return Err(UseSkillFailure::NeedItem);\n").unwrap();
                }
                job_skills_file.write_all(b"        }\n").unwrap();
            }
            job_skills_file.write_all(b"        Ok(Some(required_items))\n").unwrap();
            job_skills_file.write_all(b"    }\n").unwrap();
        }
    }
}

fn generate_validate_weapon(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if let Some(requirements) = skill_config.requires() {
        if let Some(weapon) = requirements.weapon_flags() {
            job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
            job_skills_file.write_all(b"    fn _validate_weapon(&self, status: &StatusSnapshot) -> SkillRequirementResult<()> {\n").unwrap();
            job_skills_file.write_all(b"        if let Some(character_weapon) = status.right_hand_weapon() {\n").unwrap();
            job_skills_file.write_all(format!("            if {} & character_weapon.weapon_type().as_flag() > 0 {{ Ok(()) }} else {{ Err(()) }}\n", weapon).as_bytes()).unwrap();
            job_skills_file.write_all(b"        } else {\n").unwrap();
            if weapon & WeaponType::Fist.as_flag() > 0 {
                job_skills_file.write_all(b"            // Allow to use Fist\n").unwrap();
                job_skills_file.write_all(b"            Ok(())\n").unwrap();
            } else {
                job_skills_file.write_all(b"            Err(())\n").unwrap();
            }
            job_skills_file.write_all(b"        }\n").unwrap();
            job_skills_file.write_all(b"    }\n").unwrap();
        }
    }
}

fn generate_skip_validation_item(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if let Some(skip_requirement) = skill_config.skip_requires() {
        if let Some(item_cost) = skip_requirement.item_cost() {
            job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
            job_skills_file.write_all(b"    fn _skip_item_validation(&self, state: Option<u64>) -> bool {\n").unwrap();
            if let Some(state) = item_cost.state() {
                job_skills_file.write_all(format!("        // {}\n", state.as_str()).as_bytes()).unwrap();
                job_skills_file.write_all(format!("        if state.unwrap_or(0) & {} > 0 {{ return true; }}\n", state.as_flag()).as_bytes()).unwrap();
            }
            job_skills_file.write_all(b"        false\n").unwrap();
            job_skills_file.write_all(b"    }\n").unwrap();
        }
    }
}
/*
* Other common methods
*
 */
fn generate_client_type(job_skills_file: &mut File, skill_config: &SkillConfig) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _client_type(&self) -> usize {\n").unwrap();
    if matches!(skill_config.target_type(), SkillTargetType::Target) {
        job_skills_file.write_all(format!("        {}\n", SkillTargetType::Friend.value()).as_bytes()).unwrap();
    } else {
        job_skills_file.write_all(format!("        {}\n", skill_config.target_type().value()).as_bytes()).unwrap();
    }
    job_skills_file.write_all(b"    }\n").unwrap();
}

/*
*   Skills offensive methods
*
*/
fn generate_hit_count(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.hit_count().is_none() && skill_config.hit_count_per_level().is_none() {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _hit_count(&self) -> i8 {\n").unwrap();
    generate_return_per_level_i32(job_skills_file, skill_config.hit_count(), skill_config.hit_count_per_level());
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_dmg_atk(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.dmg_atk().is_none() && skill_config.dmg_atk_per_level().is_none() {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _dmg_atk(&self) -> Option<f32> {\n").unwrap();
    generate_return_per_level_option_f32(job_skills_file, skill_config.dmg_atk(), skill_config.dmg_atk_per_level());
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_dmg_matk(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.dmg_matk().is_none() && skill_config.dmg_matk_per_level().is_none() {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _dmg_matk(&self) -> Option<f32> {\n").unwrap();
    generate_return_per_level_option_f32(job_skills_file, skill_config.dmg_matk(), skill_config.dmg_matk_per_level());
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_element(job_skills_file: &mut File, skill_config: &SkillConfig) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _element(&self) -> Element {\n").unwrap();
    job_skills_file.write_all(format!("        Element::{:?}\n", skill_config.element().unwrap_or(Element::Neutral)).as_bytes()).unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_inflict_status(job_skills_file: &mut File, skill_config: &SkillConfig) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _inflict_status_effect_to_target(&self, _status: &StatusSnapshot, _target_status: &StatusSnapshot, mut _rng: fastrand::Rng) -> Vec<StatusEffect> {\n").unwrap();
    if skill_config.bonus_to_target_before_hit().is_empty() {
        job_skills_file.write_all(b"        vec![]\n").unwrap();
    } else {
        let change_to_inflict_status_effects = skill_config.bonus_to_target_before_hit().iter()
            .filter(|b| matches!(b.value.0, BonusType::ChanceToInflictStatusOnAttackPercentage(_, _)));
        if change_to_inflict_status_effects.clone().count() == 0 {
            job_skills_file.write_all(b"        vec![]\n").unwrap();
        } else {
            // We need to group per effect, because skills like meteor assault can cause multiple effect
            let mut map: HashMap<String, Vec<BonusPerLevel>> = HashMap::new();
            for bonus in change_to_inflict_status_effects {
                match bonus.value.0 {
                    BonusType::ChanceToInflictStatusOnAttackPercentage(effect, _) => {
                        map.entry(format!("{:?}", effect))
                            .or_insert_with(Vec::new)
                            .push(bonus.clone());
                    }
                    _ => {}
                }
            }
            let change_to_inflict_status_effects_grouped: Vec<Vec<BonusPerLevel>> = map.into_values().collect::<Vec<Vec<BonusPerLevel>>>();
            job_skills_file.write_all(format!("        let mut effects = Vec::with_capacity({});\n", change_to_inflict_status_effects_grouped.len()).as_bytes()).unwrap();
            for j in 0..change_to_inflict_status_effects_grouped.len() {
                job_skills_file.write_all(b"        let chance = _rng.u8(1..=100);\n").unwrap();
                for i in 1..=skill_config.max_level() {
                    job_skills_file.write_all(format!("        if self.level == {} {{\n", i).as_bytes()).unwrap();
                    for bonus in change_to_inflict_status_effects_grouped[j].iter() {
                        let mut write_inflict_effect = || match bonus.value.0 {
                            BonusType::ChanceToInflictStatusOnAttackPercentage(effect, chance) => {
                                job_skills_file.write_all(format!("            if chance <= {} {{\n", chance).as_bytes()).unwrap();
                                job_skills_file.write_all(format!("                effects.push(StatusEffect::{:?});\n", effect).as_bytes()).unwrap();
                                job_skills_file.write_all(b"            }\n").unwrap();
                            }
                            _ => panic!("Can't go there")
                        };
                        if let Some(level) = bonus.level {
                            if level == i as u8 {
                                write_inflict_effect();
                            }
                        } else {
                            write_inflict_effect();
                        }
                    }
                    job_skills_file.write_all(b"        }\n").unwrap();
                }
            }

            job_skills_file.write_all(b"        effects\n").unwrap();
        }
    }
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_bonuses(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.bonus_to_self().len() > 0 {
        job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
        job_skills_file.write_all(b"    fn _has_bonuses_to_self(&self) -> bool {\n").unwrap();
        job_skills_file.write_all(b"        true\n").unwrap();
        job_skills_file.write_all(b"    }\n").unwrap();

        job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
        job_skills_file.write_all(b"    fn _bonuses_to_self(&self, tick: u128) -> TemporaryStatusBonuses {\n").unwrap();
        generate_for_each_bonus_level(job_skills_file, skill_config, &skill_config.bonus_to_self());
        job_skills_file.write_all(b"        TemporaryStatusBonuses::default()\n").unwrap();
        job_skills_file.write_all(b"    }\n").unwrap();
    }
    if skill_config.bonus_to_target().len() > 0 {
        job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
        job_skills_file.write_all(b"    fn _has_bonuses_to_target(&self) -> bool {\n").unwrap();
        job_skills_file.write_all(b"        true\n").unwrap();
        job_skills_file.write_all(b"    }\n").unwrap();

        job_skills_file.write_all(b"    fn _bonuses_to_target(&self, tick: u128) -> TemporaryStatusBonuses {\n").unwrap();
        generate_for_each_bonus_level(job_skills_file, skill_config, &skill_config.bonus_to_target());
        job_skills_file.write_all(b"        TemporaryStatusBonuses::default()\n").unwrap();
        job_skills_file.write_all(b"    }\n").unwrap();
    }

    if skill_config.bonus_to_party().len() > 0 {
        job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
        job_skills_file.write_all(b"    fn _has_bonuses_to_party(&self) -> bool {\n").unwrap();
        job_skills_file.write_all(b"        true\n").unwrap();
        job_skills_file.write_all(b"    }\n").unwrap();

        job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
        job_skills_file.write_all(b"    fn _bonuses_to_party(&self, tick: u128) -> TemporaryStatusBonuses {\n").unwrap();
        generate_for_each_bonus_level(job_skills_file, skill_config, &skill_config.bonus_to_party());
        job_skills_file.write_all(b"        TemporaryStatusBonuses::default()\n").unwrap();
        job_skills_file.write_all(b"    }\n").unwrap();
    }
}

fn generate_for_each_bonus_level(job_skills_file: &mut File, skill_config: &SkillConfig, bonuses: &&Vec<BonusPerLevel>) {
    if bonuses.len() > 0 {
        for i in 1..=skill_config.max_level() {
            job_skills_file.write_all(format!("        if self.level == {} {{\n", i).as_bytes()).unwrap();
            job_skills_file.write_all(b"            return TemporaryStatusBonuses(vec![").unwrap();
            for bonus in bonuses.iter() {
                if let Some(level) = bonus.level {
                    if level == i as u8 {
                        write_bonus(job_skills_file, bonus, skill_config, Some(i));
                    }
                } else {
                    write_bonus(job_skills_file, bonus, skill_config, None);
                }
            }
            job_skills_file.write_all(b"]);\n").unwrap();
            job_skills_file.write_all(b"        }\n").unwrap();
        }
    }
}

fn write_bonus(job_skills_file: &mut File, bonus: &BonusPerLevel, skill_config: &SkillConfig, level: Option<u32>) {
    if skill_config.skill_type().is_some() && matches!(skill_config.skill_type().unwrap(), SkillType::Passive) {
        job_skills_file.write_all(format!("\n                TemporaryStatusBonus::with_passive_skill({:?}, StatusBonusFlag::Default.as_flag(), {}),",
                                          bonus.value(), skill_config.id)
            .as_bytes()).unwrap();
        return;
    }
    let duration = if let Some(duration) = skill_config.duration1() {
        *duration
    } else if let Some(duration) = skill_config.duration1_per_level() {
        duration[(level.unwrap()) as usize]
    } else {
        println!("No duration found for bonus for skill {}, will not generate bonus", skill_config.name);
        return;
    };
    job_skills_file.write_all(format!("\n                TemporaryStatusBonus::with_duration({:?}, {}, tick, {}, {}),",
                                      bonus.value(), skill_config.bonus_flags().unwrap_or(StatusBonusFlag::Default.as_flag()), duration, skill_config.id)
        .as_bytes()).unwrap();
}

/*
*   Skills delays methods
*
*/
fn generate_base_cast_time(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.cast_time().is_none() && skill_config.cast_time_per_level().is_none() {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _base_cast_time(&self) -> u32 {\n").unwrap();
    generate_return_per_level_u32(job_skills_file, skill_config.cast_time(), skill_config.cast_time_per_level());
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_base_after_cast_walk_delay(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.after_cast_walk_delay().is_none() && skill_config.after_cast_act_delay_per_level().is_none() {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _base_after_cast_walk_delay(&self) -> u32 {\n").unwrap();
    generate_return_per_level_u32(job_skills_file, skill_config.after_cast_walk_delay(), skill_config.after_cast_act_delay_per_level());
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_base_after_cast_act_delay(job_skills_file: &mut File, skill_config: &SkillConfig) {
    if skill_config.after_cast_act_delay().is_none() && skill_config.after_cast_act_delay_per_level().is_none() {
        return;
    }
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn _base_after_cast_act_delay(&self) -> u32 {\n").unwrap();
    generate_return_per_level_u32(job_skills_file, skill_config.after_cast_act_delay(), skill_config.after_cast_act_delay_per_level());
    job_skills_file.write_all(b"    }\n").unwrap();
}

/*
*
*   Generate helper methods
*
 */
fn generate_as_any(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_any(&self) -> &dyn Any {\n").unwrap();
    job_skills_file.write_all(b"        self\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_is_offensive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn is_offensive_skill(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(b"        true\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_as_offensive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {\n").unwrap();
    job_skills_file.write_all(b"        Some(self)\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_is_supportive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn is_supportive_skill(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(b"        true\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_as_supportive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_supportive_skill(&self) -> Option<&dyn SupportiveSkill> {\n").unwrap();
    job_skills_file.write_all(b"        Some(self)\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_is_interactive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn is_interactive_skill(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(b"        true\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_as_interactive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_interactive_skill(&self) -> Option<&dyn InteractiveSkill> {\n").unwrap();
    job_skills_file.write_all(b"        Some(self)\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_is_ground_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn is_ground_skill(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(b"        true\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_as_ground_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {\n").unwrap();
    job_skills_file.write_all(b"        Some(self)\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_is_performance_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn is_performance_skill(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(b"        true\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_as_performance_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_performance_skill(&self) -> Option<&dyn PerformanceSkill> {\n").unwrap();
    job_skills_file.write_all(b"        Some(self)\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_is_passive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn is_passive_skill(&self) -> bool {\n").unwrap();
    job_skills_file.write_all(b"        true\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

fn generate_as_passive_skill(job_skills_file: &mut File) {
    job_skills_file.write_all(b"    #[inline(always)]\n").unwrap();
    job_skills_file.write_all(b"    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {\n").unwrap();
    job_skills_file.write_all(b"        Some(self)\n").unwrap();
    job_skills_file.write_all(b"    }\n").unwrap();
}

/*
*
*   Skill Enum
*
 */
fn generate_skills_enum(output_path: &Path, skills: &Vec<SkillConfig>, skill_tree: &Vec<JobSkillTree>, _skills_already_generated: &BTreeSet<String>, _jobs_with_skills: &BTreeSet<String>) {
    let file_path = output_path.join("skill_enums.rs");
    let mut file = File::create(file_path.clone()).unwrap();
    write_file_header_comments(&mut file);
    file.write_all(b"use crate::enums::client_effect_icon::ClientEffectIcon;\n\n").unwrap();
    file.write_all("#[derive(Clone, Copy, PartialEq, Debug)]\n".to_string().as_bytes()).unwrap();
    file.write_all("pub enum SkillEnum {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        let class_name = class_name(skill, skill_tree);
        if let Some(class_name) = class_name {
            file.write_all(format!("    // {}: {}\n", class_name, skill.description).as_bytes()).unwrap();
        } else {
            file.write_all(format!("    // {}\n", skill.description).as_bytes()).unwrap();
        }
        file.write_all(format!("    {enum_name},\n").as_bytes()).unwrap();
    }
    file.write_all("}\n".to_string().as_bytes()).unwrap();
    file.write_all("impl SkillEnum {\n".to_string().as_bytes()).unwrap();
    // Start id
    file.write_all("    pub fn id(&self) -> u32{\n".to_string().as_bytes()).unwrap();
    file.write_all("        match self {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        file.write_all(format!("            Self::{} => {},\n", enum_name, skill.id).as_bytes()).unwrap();
    }
    file.write_all("        }\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    // End id

    // Start from_id
    file.write_all("    pub fn from_id(id: u32) -> Self {\n".to_string().as_bytes()).unwrap();
    file.write_all("        match id {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        file.write_all(format!("            {} => Self::{},\n", skill.id, enum_name).as_bytes()).unwrap();
    }
    file.write_all("            _ => panic!(\"unknown skill with id {}\", id)\n".to_string().as_bytes()).unwrap();
    file.write_all("        }\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    // End from_id

    // Start from_name
    file.write_all("    pub fn from_name(name: &str) -> Self {\n".to_string().as_bytes()).unwrap();
    file.write_all("        match name {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        file.write_all(format!("            \"{}\" => Self::{},\n", skill.name, enum_name).as_bytes()).unwrap();
    }
    file.write_all("            _ => panic!(\"unknown skill with name {}\", name)\n".to_string().as_bytes()).unwrap();
    file.write_all("        }\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    // end to_name

    // Start to_name
    file.write_all("    pub fn to_name(&self) -> &str {\n".to_string().as_bytes()).unwrap();
    file.write_all("        match self {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        file.write_all(format!("            Self::{} => \"{}\",\n", enum_name, skill.name).as_bytes()).unwrap();
    }
    file.write_all("        }\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    // End to_name

    // Start is_platinium
    file.write_all("    pub fn is_platinium(&self) -> bool {\n".to_string().as_bytes()).unwrap();
    file.write_all("        match self {\n".to_string().as_bytes()).unwrap();
    let default_flags = 0;
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        file.write_all(format!("            Self::{} => {},\n", enum_name,
                               SkillFlags::Iswedding.as_flag() & skill.flags.as_ref().unwrap_or(&default_flags) != 0 ||
                                   SkillFlags::Isquest.as_flag() & skill.flags.as_ref().unwrap_or(&default_flags) != 0
        ).as_bytes()).unwrap();
    }
    file.write_all("        }\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    // End is_platinium

    // Start client_icon
    file.write_all("    pub fn client_icon(&self) -> Option<ClientEffectIcon> {\n".to_string().as_bytes()).unwrap();
    file.write_all("        match self {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        let enum_name = to_enum_name(skill);
        if let Some(effect_icon) = skill.effect_client_icon() {
            file.write_all(format!("            Self::{} => Some(ClientEffectIcon::{:?}),\n", enum_name, effect_icon).as_bytes()).unwrap();
        } else {
            file.write_all(format!("            Self::{} => None,\n", enum_name).as_bytes()).unwrap();
        }
    }
    file.write_all("        }\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    // End client_icon

    file.write_all("}\n".to_string().as_bytes()).unwrap();
    println!("Skills enum generated at {}", file_path.to_str().unwrap());
}

fn generate_skills_enum_to_object(output_path: &Path, skills: &Vec<SkillConfig>, _skill_tree: &Vec<JobSkillTree>, skills_already_generated: &BTreeSet<String>, jobs_with_skills: &BTreeSet<String>) {
    let file_path = output_path.join("skill_enums.rs");
    let mut file = File::create(file_path.clone()).unwrap();
    write_file_header_comments(&mut file);
    for job in jobs_with_skills {
        file.write_all(format!("use crate::skills::{}::{{*}};\n", job).as_bytes()).unwrap();
        file.write_all(format!("use crate::base::{}_base::{{*}};\n", job).as_bytes()).unwrap();
    }
    file.write_all("use models::enums::skill_enums::SkillEnum;\n\n".to_string().as_bytes()).unwrap();
    file.write_all("use crate::{Skill, OffensiveSkill};\n\n".to_string().as_bytes()).unwrap();

    file.write_all("impl Into<Box<dyn Skill>> for models::status::KnownSkill {\n".to_string().as_bytes()).unwrap();
    file.write_all("    fn into(self) -> Box<dyn Skill> {\n".to_string().as_bytes()).unwrap();
    file.write_all("        self::to_object(self.value, self.level).unwrap()\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    file.write_all("}\n\n".to_string().as_bytes()).unwrap();

    file.write_all("pub fn to_object(skill_enum: SkillEnum, level: u8) -> Option<Box<dyn Skill>> {\n".to_string().as_bytes()).unwrap();
    file.write_all("    match skill_enum {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter() {
        if !skills_already_generated.contains(skill.name()) {
            continue;
        }
        file.write_all(format!("        SkillEnum::{} => {}::new(level).map(|s| Box::new(s) as Box<dyn Skill>),\n", to_enum_name(skill), to_struct_name(skill)).as_bytes()).unwrap();
    }
    file.write_all("    _ => None\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    file.write_all("}\n".to_string().as_bytes()).unwrap();

    file.write_all("pub fn to_offensive_skill(skill_enum: SkillEnum, level: u8) -> Option<Box<dyn OffensiveSkill>> {\n".to_string().as_bytes()).unwrap();
    file.write_all("    match skill_enum {\n".to_string().as_bytes()).unwrap();
    for skill in skills.iter().filter(|skill_config| is_offensive(&skill_config)) {
        if !skills_already_generated.contains(skill.name()) {
            continue;
        }
        file.write_all(format!("        SkillEnum::{} => {}::new(level).map(|s| Box::new(s) as Box<dyn OffensiveSkill>),\n", to_enum_name(skill), to_struct_name(skill)).as_bytes()).unwrap();
    }
    file.write_all("    _ => None\n".to_string().as_bytes()).unwrap();
    file.write_all("    }\n".to_string().as_bytes()).unwrap();
    file.write_all("}\n".to_string().as_bytes()).unwrap();

    println!("Skills to object generated at {}", file_path.to_str().unwrap());
}

/*
*
* Macro
*
 */

macro_rules! generate_return_per_level {
    ($function:ident, $type:ty, $is_option:expr) => {
        fn $function(job_skills_file: &mut File, value: &Option<$type>, value_per_level: &Option<Vec<$type>>) {

            if let Some(value) = value {
                if $is_option {
                    job_skills_file.write_all(format!("       Some({:.3})\n", value).as_bytes()).unwrap();
                } else {
                    job_skills_file.write_all(format!("       {:.3}\n", value).as_bytes()).unwrap();
                }

            } else if let Some(value_per_level) = value_per_level {
                for (level, value_per_level) in value_per_level.iter().enumerate() {
                    if level == 0 { continue; }
                    job_skills_file.write_all(format!("        if self.level == {} {{\n", level).as_bytes()).unwrap();
                    if $is_option {
                        job_skills_file.write_all(format!("            return Some({:.3})\n", value_per_level).as_bytes()).unwrap();
                    } else {
                        job_skills_file.write_all(format!("            return {:.3}\n", value_per_level).as_bytes()).unwrap();

                    }
                    job_skills_file.write_all(b"        }\n").unwrap();
                }
                if $is_option {
                    job_skills_file.write_all(b"        None\n").unwrap();
                } else {
                    job_skills_file.write_all(b"        0\n").unwrap();
                }
            }
        }
    }
}
generate_return_per_level!(generate_return_per_level_f32, f32, false);
generate_return_per_level!(generate_return_per_level_u32, u32, false);
generate_return_per_level!(generate_return_per_level_i32, i32, false);
generate_return_per_level!(generate_return_per_level_option_f32, f32, true);
generate_return_per_level!(generate_return_per_level_option_u32, u32, true);
generate_return_per_level!(generate_return_per_level_option_i32, i32, true);


fn generate_validate_per_level(job_skills_file: &mut File, field_name: &str, value: &Option<u32>, value_per_level: &Option<Vec<u32>>) {
    if let Some(value) = value {
        job_skills_file.write_all(format!("        if {} > {} {{ Ok({}) }} else {{Err(())}}\n", field_name, value, value).as_bytes()).unwrap();
    } else if let Some(value_per_level) = value_per_level {
        for (level, value_per_level) in value_per_level.iter().enumerate() {
            if level == 0 { continue; }
            job_skills_file.write_all(format!("        if self.level == {} {{\n", level).as_bytes()).unwrap();
            job_skills_file.write_all(format!("            if {} >= {} {{ return Ok({}) }} else {{return Err(())}}\n", field_name, value_per_level, value_per_level).as_bytes()).unwrap();
            job_skills_file.write_all(b"        }\n").unwrap();
        }
        job_skills_file.write_all(b"        Err(())\n").unwrap();
    }
}
/*
*
*   Helpers
*
 */
fn to_enum_name(skill: &SkillConfig) -> String {
    skill.name.to_case(Case::Title).replace(' ', "")
}

fn to_struct_name(skill: &SkillConfig) -> String {
    NON_ALPHA_REGEX.replace_all(&skill.description, "").to_case(Case::UpperCamel)
}

fn get_skill_config<'a>(skill_name: &String, skills: &'a Vec<SkillConfig>) -> Option<&'a SkillConfig> {
    skills.iter().find(|&skill_config| skill_name.eq(skill_config.name()))
}

fn class_name(skill_config: &SkillConfig, skill_tree: &Vec<JobSkillTree>) -> Option<String> {
    for job_tree in skill_tree.iter() {
        if job_tree.name().eq("Super Novice") || job_tree.name().ends_with("High") || job_tree.name().eq("Super_Baby") {
            continue;
        }
        for skill in job_tree.tree().iter() {
            if skill_config.name().to_lowercase().starts_with("bd") {
                return Some("Bard".to_string());
            }
            if skill_config.name().to_lowercase().starts_with("cg") {
                return Some("Clown".to_string());
            }
            if skill.name().eq(skill_config.name()) && job_tree.name().chars().nth(0).unwrap().to_lowercase().eq(skill.name().chars().nth(0).unwrap().to_lowercase()) {
                return Some(job_tree.name().clone());
            }
        }
    }
    None
}

fn is_offensive(skill_config: &SkillConfig) -> bool {
    skill_config.skill_type().map_or(false, |skill_type| matches!(skill_type, SkillType::Offensive))
}

fn is_passive(skill_config: &SkillConfig) -> bool {
    skill_config.skill_type().map_or(false, |skill_type| matches!(skill_type, SkillType::Passive))
}

fn is_ground(skill_config: &SkillConfig) -> bool {
    matches!(skill_config.target_type(), SkillTargetType::Ground)
}

fn is_support(skill_config: &SkillConfig) -> bool {
    skill_config.skill_type().map_or(false, |skill_type| matches!(skill_type, SkillType::Support))
}

fn is_performance(skill_config: &SkillConfig) -> bool {
    skill_config.skill_type().map_or(false, |skill_type| matches!(skill_type, SkillType::Performance))
}
fn is_interactive(skill_config: &SkillConfig) -> bool {
    skill_config.skill_type().map_or(false, |skill_type| matches!(skill_type, SkillType::Interactive))
}