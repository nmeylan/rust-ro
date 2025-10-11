#![allow(unused_imports)]

use crate::base::sniper_base::*;
use crate::{GroundSkill, InteractiveSkill, OffensiveSkill, PassiveSkill, PerformanceSkill, Skill, SupportiveSkill};

impl Skill for FalconEyes {
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
impl SupportiveSkill for FalconEyes {}
impl Skill for FalconAssault {
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
impl OffensiveSkill for FalconAssault {}
impl Skill for FocusedArrowStrike {
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
impl OffensiveSkill for FocusedArrowStrike {}
impl Skill for WindWalker {
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
impl SupportiveSkill for WindWalker {}
