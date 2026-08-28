#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, InteractiveSkill};


use crate::base::taekwon_base::{*};

impl Skill for Running {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl InteractiveSkill for Running {
}
impl Skill for TornadoStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for TornadoStance {
}
impl Skill for TornadoKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for TornadoKick {
}
impl Skill for HeelDropStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for HeelDropStance {
}
impl Skill for HeelDrop {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for HeelDrop {
}
impl Skill for RoundhouseStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for RoundhouseStance {
}
impl Skill for RoundhouseKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for RoundhouseKick {
}
impl Skill for CounterKickStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for CounterKickStance {
}
impl Skill for CounterKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for CounterKick {
}
impl Skill for Tumbling {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for Tumbling {
}
impl Skill for FlyingKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for FlyingKick {
}
impl Skill for PeacefulBreak {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl InteractiveSkill for PeacefulBreak {
}
impl Skill for HappyBreak {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl InteractiveSkill for HappyBreak {
}
impl Skill for Kihop {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for Kihop {
}
impl Skill for MildWind {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for MildWind {
}
impl Skill for TaekwonJump {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for TaekwonJump {
}
impl Skill for TaekwonMission {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl InteractiveSkill for TaekwonMission {
}
