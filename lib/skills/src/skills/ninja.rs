#![allow(unused_imports)]


use crate::{Skill, PassiveSkill, SupportiveSkill, PerformanceSkill, OffensiveSkill, GroundSkill, InteractiveSkill};


use crate::base::ninja_base::{*};

impl Skill for ShurikenTraining {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl PassiveSkill for ShurikenTraining {
}
impl Skill for ThrowShuriken {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ThrowShuriken {
}
impl Skill for ThrowKunai {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ThrowKunai {
}
impl Skill for ThrowHuumaShuriken {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ThrowHuumaShuriken {
}
impl Skill for ThrowZeny {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ThrowZeny {
}
impl Skill for ImprovisedDefense {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for ImprovisedDefense {
}
impl Skill for VanishingSlash {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for VanishingSlash {
}
impl Skill for ShadowLeap {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl InteractiveSkill for ShadowLeap {
}
impl GroundSkill for ShadowLeap {
}
impl Skill for ShadowSlash {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for ShadowSlash {
}
impl Skill for CicadaSkinSheeding {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for CicadaSkinSheeding {
}
impl Skill for MirrorImage {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for MirrorImage {
}
impl Skill for SpiritoftheBlade {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for SpiritoftheBlade {
}
impl Skill for CrimsonFirePetal {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for CrimsonFirePetal {
}
impl Skill for CrimsonFireFormation {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for CrimsonFireFormation {
}
impl Skill for RagingFireDragon {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for RagingFireDragon {
}
impl Skill for SpearofIce {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for SpearofIce {
}
impl Skill for HiddenWater {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for HiddenWater {
}
impl GroundSkill for HiddenWater {
}
impl Skill for IceMeteor {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for IceMeteor {
}
impl Skill for WindBlade {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for WindBlade {
}
impl Skill for LightningStrikeofDestruction {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for LightningStrikeofDestruction {
}
impl Skill for Kamaitachi {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for Kamaitachi {
}
impl Skill for Soul {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 5 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl SupportiveSkill for Soul {
}
impl Skill for FinalStrike {
    fn new(level: u8) -> Option<Self> where Self : Sized {
        if level > 10 { return None }
        Some(Self { level, cast_time: 0, after_cast_act_delay: 0, after_cast_walk_delay: 0 })
    }
}
impl OffensiveSkill for FinalStrike {
}
