#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::paladin_base::{*};

impl Skill for GloriaDomini {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for GloriaDomini {
}
impl Skill for MartyrsReckoning {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for MartyrsReckoning {
}
impl Skill for BattleChant {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for BattleChant {
}
impl Skill for ShieldChain {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ShieldChain {
}
