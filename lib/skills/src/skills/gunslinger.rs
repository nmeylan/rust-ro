#![allow(unused_imports)]

use crate::base::gunslinger_base::*;
use crate::{GroundSkill, InteractiveSkill, OffensiveSkill, PassiveSkill, PerformanceSkill, Skill, SupportiveSkill};

impl Skill for FliptheCoin {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 5 {
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
impl InteractiveSkill for FliptheCoin {}
impl Skill for Fling {
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
impl OffensiveSkill for Fling {}
impl Skill for TripleAction {
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
impl OffensiveSkill for TripleAction {}
impl Skill for BullsEye {
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
impl OffensiveSkill for BullsEye {}
impl Skill for MadnessCanceller {
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
impl OffensiveSkill for MadnessCanceller {}
impl Skill for AdJustment {
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
impl OffensiveSkill for AdJustment {}
impl Skill for IncreasingAccuracy {
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
impl SupportiveSkill for IncreasingAccuracy {}
impl Skill for MagicalBullet {
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
impl OffensiveSkill for MagicalBullet {}
impl Skill for Cracker {
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
impl OffensiveSkill for Cracker {}
impl Skill for SingleAction {
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
impl InteractiveSkill for SingleAction {}
impl Skill for SnakeEye {
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
impl InteractiveSkill for SnakeEye {}
impl Skill for ChainAction {
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
impl OffensiveSkill for ChainAction {}
impl Skill for Tracking {
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
impl OffensiveSkill for Tracking {}
impl Skill for Disarm {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 5 {
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
impl OffensiveSkill for Disarm {}
impl Skill for PiercingShot {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 5 {
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
impl OffensiveSkill for PiercingShot {}
impl Skill for RapidShower {
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
impl OffensiveSkill for RapidShower {}
impl Skill for Desperado {
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
impl OffensiveSkill for Desperado {}
impl Skill for GatlingFever {
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
impl OffensiveSkill for GatlingFever {}
impl Skill for Dust {
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
impl OffensiveSkill for Dust {}
impl Skill for FullBuster {
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
impl OffensiveSkill for FullBuster {}
impl Skill for SpreadAttack {
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
impl OffensiveSkill for SpreadAttack {}
impl Skill for GroundDrift {
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
impl OffensiveSkill for GroundDrift {}
impl GroundSkill for GroundDrift {}
