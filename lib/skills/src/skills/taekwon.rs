#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, SelfSkill};


use crate::base::taekwon_base::{*};

impl Skill for Running {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Running {
}
impl Skill for TornadoStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for TornadoStance {
}
impl Skill for TornadoKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for TornadoKick {
}
impl SelfSkill for TornadoKick {
}
impl Skill for HeelDropStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for HeelDropStance {
}
impl Skill for HeelDrop {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for HeelDrop {
}
impl SelfSkill for HeelDrop {
}
impl Skill for RoundhouseStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for RoundhouseStance {
}
impl Skill for RoundhouseKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for RoundhouseKick {
}
impl SelfSkill for RoundhouseKick {
}
impl Skill for CounterKickStance {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for CounterKickStance {
}
impl Skill for CounterKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for CounterKick {
}
impl SelfSkill for CounterKick {
}
impl Skill for Tumbling {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for Tumbling {
}
impl Skill for FlyingKick {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for FlyingKick {
}
impl SupportiveSkill for FlyingKick {
}
impl Skill for PeacefulBreak {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for PeacefulBreak {
}
impl Skill for HappyBreak {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for HappyBreak {
}
impl Skill for Kihop {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for Kihop {
}
impl Skill for MildWind {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 7 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for MildWind {
}
impl Skill for TaekwonJump {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for TaekwonJump {
}
impl Skill for TaekwonMission {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level < 1 || level > 1 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SelfSkill for TaekwonMission {
}
