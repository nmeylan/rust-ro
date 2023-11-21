// Generated by tools/skills/main.rs
// Auto generated file do not edit manually

#![allow(dead_code, unused_must_use, unused_imports, unused_variables)]

use enums::{EnumWithMaskValueU64, EnumWithNumberValue};
use enums::skill::*;
use enums::weapon::AmmoType;

use models::item::WearWeapon;

use models::status::Status;
use models::item::NormalInventoryItem;

use crate::{*};

use crate::base::*;
use std::any::Any;
// HW_SOULDRAIN
pub struct SoulDrain {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for SoulDrain {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        364
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn is_passive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_passive_skill(&self) -> Option<&dyn PassiveSkill> {
        Some(self)
    }
}
impl PassiveSkillBase for SoulDrain {
}
// HW_MAGICCRASHER
pub struct StaveCrasher {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for StaveCrasher {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        365
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 8 { Ok(8) } else {Err(())}
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       300
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       300
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for StaveCrasher {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
       1
    }
}
// HW_MAGICPOWER
pub struct MysticalAmplification {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for MysticalAmplification {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        366
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp >= 14 { return Ok(14) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 18 { return Ok(18) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 22 { return Ok(22) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 26 { return Ok(26) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 30 { return Ok(30) } else {return Err(())}
        }
        if self.level == 6 {
            if status.sp >= 34 { return Ok(34) } else {return Err(())}
        }
        if self.level == 7 {
            if status.sp >= 38 { return Ok(38) } else {return Err(())}
        }
        if self.level == 8 {
            if status.sp >= 42 { return Ok(42) } else {return Err(())}
        }
        if self.level == 9 {
            if status.sp >= 46 { return Ok(46) } else {return Err(())}
        }
        if self.level == 10 {
            if status.sp >= 50 { return Ok(50) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       700
    }
    #[inline(always)]
    fn is_self_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_self_skill(&self) -> Option<&dyn SelfSkill> {
        Some(self)
    }
}
impl SelfSkillBase for MysticalAmplification {
}
// HW_NAPALMVULCAN
pub struct NapalmVulcan {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for NapalmVulcan {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        400
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp >= 10 { return Ok(10) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 25 { return Ok(25) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 55 { return Ok(55) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 70 { return Ok(70) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       1000
    }
    #[inline(always)]
    fn is_offensive_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_offensive_skill(&self) -> Option<&dyn OffensiveSkill> {
        Some(self)
    }
}
impl OffensiveSkillBase for NapalmVulcan {
    #[inline(always)]
    fn _hit_count(&self) -> i8 {
        if self.level == 1 {
            return 1
        }
        if self.level == 2 {
            return 2
        }
        if self.level == 3 {
            return 3
        }
        if self.level == 4 {
            return 4
        }
        if self.level == 5 {
            return 5
        }
        0
    }
}
// HW_GANBANTEIN
pub struct Ganbantein {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for Ganbantein {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        483
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if status.sp > 40 { Ok(40) } else {Err(())}
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 715, name_english: "Yellow_Gemstone".to_string(), amount: 1}),(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 715 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::NeedItem);
        }
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       3000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for Ganbantein {
}
// HW_GRAVITATION
pub struct GravitationField {
    pub(crate) level: u8,
    pub(crate) cast_time: u32,
    pub(crate) after_cast_act_delay: u32,
    pub(crate) after_cast_walk_delay: u32,
}
impl SkillBase for GravitationField {
    #[inline(always)]
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn _id(&self) -> u32 {
        484
    }
    fn _level(&self) -> u8 {
        self.level
    }
    #[inline(always)]
    fn _cast_time(&self) -> u32 {
        self.cast_time
    }
    #[inline(always)]
    fn _after_cast_act_delay(&self) -> u32 {
        self.after_cast_act_delay
    }
    #[inline(always)]
    fn _after_cast_walk_delay(&self) -> u32 {
        self.after_cast_walk_delay
    }
    #[inline(always)]
    fn _update_cast_time(&mut self, new_value: u32) {
        self.cast_time = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_act_delay(&mut self, new_value: u32) {
        self.after_cast_act_delay = new_value;
    }
    #[inline(always)]
    fn _update_after_cast_walk_delay(&mut self, new_value: u32) {
        self.after_cast_walk_delay = new_value;
    }
    #[inline(always)]
    fn _validate_sp(&self, status: &Status) -> SkillRequirementResult<u32> {
        if self.level == 1 {
            if status.sp >= 20 { return Ok(20) } else {return Err(())}
        }
        if self.level == 2 {
            if status.sp >= 40 { return Ok(40) } else {return Err(())}
        }
        if self.level == 3 {
            if status.sp >= 60 { return Ok(60) } else {return Err(())}
        }
        if self.level == 4 {
            if status.sp >= 80 { return Ok(80) } else {return Err(())}
        }
        if self.level == 5 {
            if status.sp >= 100 { return Ok(100) } else {return Err(())}
        }
        Err(())
    }
    #[inline(always)]
    fn _validate_item(&self, inventory: &Vec<NormalInventoryItem>) -> Result<Option<Vec<NormalInventoryItem>>, UseSkillFailure> {
        let required_items = vec![(NormalInventoryItem {item_id: 717, name_english: "Blue_Gemstone".to_string(), amount: 1})]; 
        if inventory.iter().find(|item| item.item_id == 717 && item.amount >= 1).is_none() {
            return Err(UseSkillFailure::BlueGemstone);
        }
        Ok(Some(required_items))
    }
    #[inline(always)]
    fn _base_cast_time(&self) -> u32 {
       5000
    }
    #[inline(always)]
    fn _base_after_cast_act_delay(&self) -> u32 {
       2000
    }
    #[inline(always)]
    fn is_ground_skill(&self) -> bool {
        true
    }
    #[inline(always)]
    fn as_ground_skill(&self) -> Option<&dyn GroundSkill> {
        Some(self)
    }
}
impl GroundSkillBase for GravitationField {
}