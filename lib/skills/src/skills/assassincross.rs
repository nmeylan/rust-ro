#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::assassincross_base::{*};

impl Skill for AdvancedKatarMastery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for AdvancedKatarMastery {
}
impl Skill for EnchantDeadlyPoison {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for EnchantDeadlyPoison {
}
impl Skill for SoulDestroyer {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for SoulDestroyer {
}
impl Skill for MeteorAssault {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for MeteorAssault {
}
impl SelfSkill for MeteorAssault {
}
impl Skill for CreateDeadlyPoison {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for CreateDeadlyPoison {
}
