#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::novice_base::{*};

impl Skill for BasicSkill {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=9).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for BasicSkill {
}
impl Skill for FirstAid {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for FirstAid {
}
impl Skill for PlayDead {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for PlayDead {
}
