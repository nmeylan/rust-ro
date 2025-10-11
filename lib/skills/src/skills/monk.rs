#![allow(unused_imports)]

use crate::base::monk_base::*;
use crate::{GroundSkill, InteractiveSkill, OffensiveSkill, PassiveSkill, PerformanceSkill, Skill, SupportiveSkill};

impl Skill for IronFists {
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
impl PassiveSkill for IronFists {}
impl Skill for SpiritualCadence {
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
impl PassiveSkill for SpiritualCadence {}
impl Skill for SummonSpiritSphere {
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
impl InteractiveSkill for SummonSpiritSphere {}
impl Skill for AbsorbSpiritSphere {
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
impl InteractiveSkill for AbsorbSpiritSphere {}
impl Skill for RagingTrifectaBlow {
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
impl PassiveSkill for RagingTrifectaBlow {}
impl Skill for Snap {
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
impl InteractiveSkill for Snap {}
impl GroundSkill for Snap {}
impl Skill for Dodge {
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
impl PassiveSkill for Dodge {}
impl Skill for OccultImpaction {
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
impl OffensiveSkill for OccultImpaction {}
impl Skill for ThrowSpiritSphere {
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
impl OffensiveSkill for ThrowSpiritSphere {}
impl Skill for MentalStrength {
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
impl SupportiveSkill for MentalStrength {}
impl Skill for Root {
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
impl InteractiveSkill for Root {}
impl Skill for Fury {
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
impl InteractiveSkill for Fury {}
impl Skill for AsuraStrike {
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
impl OffensiveSkill for AsuraStrike {}
impl Skill for RagingQuadrupleBlow {
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
impl OffensiveSkill for RagingQuadrupleBlow {}
impl Skill for RagingThrust {
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
impl OffensiveSkill for RagingThrust {}
impl Skill for KiTranslation {
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
impl SupportiveSkill for KiTranslation {}
impl Skill for KiExplosion {
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
impl SupportiveSkill for KiExplosion {}
