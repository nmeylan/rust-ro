#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::clown_base::{*};

impl Skill for VulcanArrow {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for VulcanArrow {
}
impl Skill for ShelteringBliss {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for ShelteringBliss {
}
impl Skill for MarionetteControl {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for MarionetteControl {
}
impl Skill for LongingforFreedom {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for LongingforFreedom {
}
impl Skill for WandofHermode {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for WandofHermode {
}
impl PerformanceSkill for WandofHermode {
}
impl Skill for TarotCardofFate {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for TarotCardofFate {
}
