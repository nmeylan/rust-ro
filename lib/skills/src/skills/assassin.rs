#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::assassin_base::{*};

impl Skill for RighthandMastery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for RighthandMastery {
}
impl Skill for LefthandMastery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for LefthandMastery {
}
impl Skill for KatarMastery {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for KatarMastery {
}
impl Skill for Cloaking {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Cloaking {
}
impl Skill for SonicBlow {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for SonicBlow {
}
impl Skill for Grimtooth {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=5).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for Grimtooth {
}
impl Skill for EnchantPoison {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for EnchantPoison {
}
impl Skill for PoisonReact {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for PoisonReact {
}
impl SelfSkill for PoisonReact {
}
impl Skill for VenomDust {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl GroundSkill for VenomDust {
}
impl Skill for VenomSplasher {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if !(1..=10).contains(&level) { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for VenomSplasher {
}
impl Skill for SonicAcceleration {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for SonicAcceleration {
}
impl Skill for ThrowVenomKnife {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level != 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ThrowVenomKnife {
}
