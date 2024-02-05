use std::{env, fs};
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use lazy_static::lazy_static;
use configuration::configuration::InternalJobsConfig;

lazy_static! {
    static ref HP_JOB_MODIFIER: HashMap<&'static str, f32> = HashMap::from([
        ("novice", 0.0),
        ("superNovice", 0.0),
        ("swordsman", 0.7),
        ("acolyte", 0.4),
        ("magician", 0.3),
        ("merchant", 0.4),
        ("archer", 0.5),
        ("thief", 0.5),
        ("knight", 1.5),
        ("priest", 0.75),
        ("wizard", 0.55),
        ("blacksmith", 0.9),
        ("hunter", 0.85),
        ("assassin", 1.1),
        ("rogue", 0.85),
        ("crusader", 1.1),
        ("monk", 0.9),
        ("sage", 0.75),
        ("alchemist", 0.9),
        ("bard", 0.75),
        ("dancer", 0.75),
        ("taekwon", 0.7),
        ("stargladiator", 0.9),
        ("soul_linker", 0.75),
        ("gunslinger", 0.89),
        ("ninja", 0.8),
    ]);
    static ref HP_JOB_FACTOR: HashMap<&'static str, f32> = HashMap::from([
        ("crusader", 7.0),
        ("bard", 3.0),
        ("dancer", 3.0),
        ("monk", 6.5),
        ("ninja", 0.0),
        ("gunslinger", 0.0),
        ("stargladiator", 6.5),
        ]);
    static ref SP_JOB_MODIFIER: HashMap<&'static str, f32> = HashMap::from([
        ("novice", 1.0),
        ("superNovice", 1.0),
        ("swordsman", 2.0),
        ("acolyte", 5.0),
        ("magician", 6.0),
        ("merchant", 3.0),
        ("archer", 2.0),
        ("thief", 2.0),
        ("knight", 3.0),
        ("priest", 8.0),
        ("wizard", 9.0),
        ("blacksmith", 4.0),
        ("hunter", 4.0),
        ("assassin", 4.0),
        ("rogue", 5.0),
        ("crusader", 4.7),
        ("monk", 4.7),
        ("sage", 7.0),
        ("alchemist", 4.0),
        ("bard", 6.0),
        ("dancer", 6.0),
        ("taekwon", 2.0),
        ("stargladiator", 4.0),
        ("soul_linker", 9.0),
        ("gunslinger", 4.0),
        ("ninja", 5.4),
    ]);
}
fn main() {
    let max_base_level = 99;
    let path = Path::new("./").join("config/job.json");
    if !path.exists() {
        return panic!(
            "config/job.json file does not exists at {}",
            env::current_dir().unwrap().join(path).to_str().unwrap()
        );
    }
    let mut internal_configs: InternalJobsConfig = serde_json::from_str(&fs::read_to_string(path.clone()).unwrap()).unwrap();
    generate_base_hp(&mut internal_configs, max_base_level);
    generate_base_sp(&mut internal_configs, max_base_level);

    let json = serde_json::to_string_pretty(&internal_configs).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn generate_base_hp(internal_configs: &mut InternalJobsConfig, max_base_level: u32) {
    for (job, conf) in internal_configs.jobs.iter_mut() {
        if HP_JOB_MODIFIER.get(job.to_lowercase().as_str()).is_none() { continue; }
        let mut results: Vec<u32> = Vec::with_capacity(max_base_level as usize);
        for i in 1..=max_base_level {
            let mut base_hp: f32 = (35.0 + (i as f32 * (*HP_JOB_FACTOR.get(job.to_lowercase().as_str()).unwrap_or(&5_f32))));

            if (i >= 10 && (job == "ninja" || job == "gunslinger")) {
                base_hp += 90.0;
            }
            for j in 2..=i {
                base_hp = base_hp + (HP_JOB_MODIFIER.get(job.to_lowercase().as_str()).expect(format!("Can't find job modifier for {}", job.to_lowercase()).as_str()) * j as f32).round();
            }
            if (job.to_lowercase().as_str() == "soul_linker") {
                if (i >= 70) {
                    if (i <= 79) {
                        base_hp -= (i as f32 - 70.0) * 40.0;
                    } else if (i <= 84) {
                        base_hp -= (i as f32 - 80.0) * 50.0;
                    } else if (i <= 89) {
                        base_hp -= (i as f32 - 80.0) * 50.0 - 10.0;
                    } else if (i <= 92) {
                        base_hp -= (i as f32 - 90.0) * 50.0;
                    } else if (i <= 97) {
                        base_hp -= (i as f32 - 90.0) * 50.0 - 10.0;
                    } else if (i == 98) {
                        base_hp -= 375.0;
                    } else {
                        base_hp -= 4.0;
                    }
                }
            }
            if (job.to_lowercase().as_str() == "taekwon") {
                if (i <= 79) {
                    base_hp = 2127.0 + 10.0 * (i as f32 - 70.0)
                } else if (i <= 89) {
                    base_hp = 2200.0 + 50.0 * (i as f32 - 80.0)
                } else if (i <= max_base_level) {
                    base_hp = 2700.0 + 50.0 * (i as f32 - 90.0)
                }
            }
            results.push(base_hp as u32);
        }
        conf.set_base_hp(Some(results));
    }
}

fn generate_base_sp(internal_configs: &mut InternalJobsConfig, max_base_level: u32) {
    for (job, conf) in internal_configs.jobs.iter_mut() {
        let mut results: Vec<u32> = Vec::with_capacity(max_base_level as usize);
        if SP_JOB_MODIFIER.get(job.to_lowercase().as_str()).is_none() { continue; }
        for i in 1..=max_base_level {
            let mut base_sp: f32 = 10.0 + (i as f32 * SP_JOB_MODIFIER.get(job.to_lowercase().as_str()).expect(format!("Can't find job modifier for {}", job.to_lowercase()).as_str())).floor();
            if job == "ninja" {
                if i <= 20 { base_sp = 11.0 + i as f32 * 3.0 } else if i <= 40 { base_sp = 71.0 + (i as f32 - 20.0) * 4.0 } else if i <= 60 { base_sp = 151.0 + (i as f32 - 40.0) * 5.0 } else if i <= 80 { base_sp = 251.0 + (i as f32 - 60.0) * 6.0 } else { base_sp = 370.0 + (i as f32 - 80.0) * 8.0 }
            } else if job == "gunslinger" {
                if i <= 25 { base_sp = 10.0 + i as f32 * 3.0; } else if i <= 35 { base_sp = 85.0 + (i as f32 - 25.0) * 4.0; } else if i <= 40 { base_sp = 126.0 + (i as f32 - 35.0) * 3.0; } else if i <= 50 { base_sp = 141.0 + (i as f32 - 40.0) * 4.0; } else if i <= 75 { base_sp = 181.0 + (i as f32 - 50.0) * 5.0; } else if i <= 78 { base_sp = 306.0 + (i as f32 - 75.0) * 6.0; } else { base_sp = 330.0 + (i as f32 - 78.0) * 6.0; }
            } else if (job.to_lowercase().as_str() == "taekwon" || job.to_lowercase().as_str() == "soul_linker") {
                if (i >= 70) {
                    if (i < 80) {
                        base_sp -= (i as f32 - 70.0) * 4.0 + 5.0;
                    } else if (i < 90) {
                        base_sp -= (i as f32 - 80.0) * 4.0;
                    } else if (i < 93) {
                        base_sp -= (i as f32 - 90.0) * 4.0;
                    } else if (i < max_base_level) {
                        base_sp -= (i as f32 - 90.0) * 4.0 - 10.0;
                    } else { base_sp -= 1.0; }
                }
            }
            results.push(base_sp as u32);
        }
        conf.set_base_sp(Some(results));
    }
}