#![allow(unused_imports)]

use crate::base::blacksmith_base::*;
use crate::{GroundSkill, InteractiveSkill, OffensiveSkill, PassiveSkill, PerformanceSkill, Skill, SupportiveSkill};

impl Skill for IronTempering {
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
impl PassiveSkill for IronTempering {}
impl Skill for SteelTempering {
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
impl PassiveSkill for SteelTempering {}
impl Skill for EnchantedStoneCraft {
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
impl PassiveSkill for EnchantedStoneCraft {}
impl Skill for OrideconResearch {
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
impl PassiveSkill for OrideconResearch {}
impl Skill for SmithDagger {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithDagger {}
impl Skill for SmithSword {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithSword {}
impl Skill for SmithTwohandedSword {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithTwohandedSword {}
impl Skill for SmithAxe {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithAxe {}
impl Skill for SmithMace {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithMace {}
impl Skill for SmithKnucklebrace {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithKnucklebrace {}
impl Skill for SmithSpear {
    fn new(level: u8) -> Option<Self>
    where
        Self: Sized,
    {
        if level > 3 {
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
impl PassiveSkill for SmithSpear {}
impl Skill for HiltBinding {
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
impl PassiveSkill for HiltBinding {}
impl Skill for OreDiscovery {
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
impl PassiveSkill for OreDiscovery {}
impl Skill for WeaponryResearch {
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
impl PassiveSkill for WeaponryResearch {}
impl Skill for WeaponRepair {
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
impl InteractiveSkill for WeaponRepair {}
impl Skill for SkinTempering {
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
impl PassiveSkill for SkinTempering {}
impl Skill for HammerFall {
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
impl OffensiveSkill for HammerFall {}
impl GroundSkill for HammerFall {}
impl Skill for AdrenalineRush {
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
impl SupportiveSkill for AdrenalineRush {}
impl Skill for WeaponPerfection {
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
impl SupportiveSkill for WeaponPerfection {}
impl Skill for PowerThrust {
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
impl SupportiveSkill for PowerThrust {}
impl Skill for MaximizePower {
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
impl SupportiveSkill for MaximizePower {}
impl Skill for UnfairTrick {
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
impl InteractiveSkill for UnfairTrick {}
impl Skill for Greed {
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
impl InteractiveSkill for Greed {}
impl Skill for AdvancedAdrenalineRush {
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
impl SupportiveSkill for AdvancedAdrenalineRush {}
