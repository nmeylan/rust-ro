#![allow(unused_imports)]

use crate::base::swordsman_base::*;
use crate::{GroundSkill, InteractiveSkill, OffensiveSkill, PassiveSkill, PerformanceSkill, Skill, SupportiveSkill};

impl Skill for SwordMastery {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl PassiveSkill for SwordMastery {}
impl Skill for TwoHandedSwordMastery {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl PassiveSkill for TwoHandedSwordMastery {}
impl Skill for IncreaseHpRecovery {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl PassiveSkill for IncreaseHpRecovery {}
impl Skill for Bash {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl OffensiveSkill for Bash {}
impl Skill for Provoke {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl OffensiveSkill for Provoke {}
impl Skill for MagnumBreak {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl OffensiveSkill for MagnumBreak {}
impl Skill for Endure {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 10 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl SupportiveSkill for Endure {}
impl Skill for MovingHpRecovery {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 1 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl PassiveSkill for MovingHpRecovery {}
impl Skill for FatalBlow {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 1 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl PassiveSkill for FatalBlow {}
impl Skill for AutoBerserk {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 1 {
            return None;
        }
        Some(Self {
            level,
            cast_time: 0,
            after_cast_act_delay: 0,
            after_cast_walk_delay: 0,
        })
    }
}
impl PassiveSkill for AutoBerserk {}
