#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::acolyte_base::{*};

impl Skill for DivineProtection {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for DivineProtection {
}
impl Skill for DemonBane {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for DemonBane {
}
impl Skill for Ruwach {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Ruwach {
}
impl Skill for Pneuma {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl GroundSkill for Pneuma {
}
impl Skill for Teleport {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 2 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Teleport {
}
impl Skill for WarpPortal {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 4 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl GroundSkill for WarpPortal {
}
impl Skill for Heal {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for Heal {
}
impl Skill for IncreaseAgi {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for IncreaseAgi {
}
impl Skill for DecreaseAgi {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for DecreaseAgi {
}
impl Skill for AquaBenedicta {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for AquaBenedicta {
}
impl Skill for SignumCrucis {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for SignumCrucis {
}
impl Skill for Angelus {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Angelus {
}
impl Skill for Blessing {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for Blessing {
}
impl Skill for Cure {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for Cure {
}
impl Skill for HolyLight {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for HolyLight {
}
