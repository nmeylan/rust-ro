use crate::enums::bonus::BonusType;
use crate::enums::skill_enums::SkillEnum;
use crate::enums::{EnumWithMaskValueU64, EnumWithNumberValue, EnumWithStringValue};
use accessor::GettersAll;
use enum_macro::{WithMaskValueU64, WithStringValue};
use std::fmt::{Display, Formatter};
use std::slice::{Iter, IterMut};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default, Debug, Clone)]
pub struct StatusBonuses(Vec<StatusBonus>);

impl StatusBonuses {
    pub fn new(bonuses: Vec<StatusBonus>) -> Self {
        Self(bonuses)
    }

    pub fn iter(&self) -> Iter<'_, StatusBonus> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, StatusBonus> {
        self.0.iter_mut()
    }
}

impl From<StatusBonuses> for Vec<StatusBonus> {
    fn from(val: StatusBonuses) -> Self {
        val.0
    }
}

#[derive(GettersAll, Debug, Clone, Copy, Eq, PartialEq)]
pub struct StatusBonus {
    bonus: BonusType,
}

#[derive(GettersAll, Debug, Clone, Copy)]
pub struct TemporaryStatusBonus {
    bonus: BonusType,
    flags: u64,
    expirency: BonusExpiry,
    state: StatusBonusState,
    source: Option<StatusBonusSource>,
}

impl PartialEq for TemporaryStatusBonus {
    fn eq(&self, other: &Self) -> bool {
        match (self.source, other.source) {
            (Some(source1), Some(source2)) => source1 == source2 && self.bonus == other.bonus,
            _ => false,
        }
    }
}


impl TemporaryStatusBonus {
    pub fn with_duration(bonus: BonusType, flags: u64, tick: u128, duration: u32, skill_id: u16) -> Self {
        Self {
            bonus,
            flags,
            expirency: BonusExpiry::Time(tick + duration as u128),
            state: StatusBonusState::No,
            source: Some(StatusBonusSource::Skill(skill_id)),
        }
    }
    pub fn with_duration_and_source(bonus: BonusType, flags: u64, tick: u128, duration: u32, source: Option<StatusBonusSource>) -> Self {
        Self {
            bonus,
            flags,
            expirency: BonusExpiry::Time(tick + duration as u128),
            state: StatusBonusState::No,
            source,
        }
    }

    pub fn with_passive_skill(bonus: BonusType, flags: u64, skill_id: u16) -> Self {
        Self {
            bonus,
            flags,
            expirency: BonusExpiry::Never,
            state: StatusBonusState::No,
            source: Some(StatusBonusSource::PassiveSkill(skill_id))
        }
    }

    pub fn has_icon(&self) -> bool {
        self.flags & StatusBonusFlag::Icon.as_flag() > 0
    }

    pub fn icon(&self) -> Option<u16> {
        if !self.has_icon() {
            return None;
        }
        if let Some(ref source) = self.source {
            match source {
                StatusBonusSource::Skill(skill_id) => {
                    let skill = SkillEnum::from_id(*skill_id as u32);
                    return skill.client_icon().map(|icon| icon.value() as u16).or(None)
                }
                StatusBonusSource::PassiveSkill(_) => {}
                StatusBonusSource::Item(_) => {}
            }
        }
        None
    }

    pub fn remaining_ms(&self, tick: u128) -> u32{
        match self.expirency {
            BonusExpiry::Never => u32::max_value(),
            BonusExpiry::Time(until) => if until > tick { (until - tick) as u32 } else { 0 },
            BonusExpiry::Counter(_) => u32::max_value()
        }
    }
}


impl Display for StatusBonus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.bonus)
    }
}

impl Display for TemporaryStatusBonus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.bonus)?;
        write!(f, ", Expire: ")?;
        match self.expirency {
            BonusExpiry::Never => write!(f, " Never "),
            BonusExpiry::Time(_until) => write!(f, " {}ms ", self.remaining_ms(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis())),
            BonusExpiry::Counter(count) => write!(f, " {} hit ", count)
        }?;
        if let Some(ref source) = self.source {
            match source {
                StatusBonusSource::Skill(skill) | StatusBonusSource::PassiveSkill(skill) => {
                    write!(f, ", Source: {:?}", SkillEnum::from_id(*skill as u32))?;
                }
                StatusBonusSource::Item(item) => {
                    write!(f, ", Source: item {:?}", item)?;
                }
            }
        }
        write!(f, ", flags: [")?;
        if self.flags & StatusBonusFlag::Icon.as_flag() > 0 {
            write!(f, "HasIcon, ")?;
        }
        if self.flags & StatusBonusFlag::Persist.as_flag() > 0 {
            write!(f, "ShouldPersist, ")?;
        }
        if self.flags & StatusBonusFlag::Unique.as_flag() > 0 {
            write!(f, "Unique per source, ")?;
        }
        write!(f, "]")?;
        write!(f, ".")
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TemporaryStatusBonuses(Vec<TemporaryStatusBonus>);

impl TemporaryStatusBonuses {
    pub fn new(bonuses: Vec<TemporaryStatusBonus>) -> Self {
        Self(bonuses)
    }
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn merge(&mut self, other: TemporaryStatusBonuses) {
        for new_bonus in other.0 {
            if let Some(existing_index) = self.0.iter().position(|existing| existing == &new_bonus) {
                info!("Replace old bonus with {} new bonus: {}", self.0[existing_index], new_bonus);
                self.0[existing_index] = new_bonus;
            } else {
                info!("Add new bonus: {}", new_bonus);
                self.0.push(new_bonus);
            }
        }
    }

    pub fn to_vec(&self) ->  Vec<StatusBonus> {
            self.0.iter().map(|temporary_status_bonus: &TemporaryStatusBonus| StatusBonus { bonus: temporary_status_bonus.bonus }).collect()
    }

    pub fn iter(&self) -> Iter<'_, TemporaryStatusBonus> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, TemporaryStatusBonus> {
        self.0.iter_mut()
    }
    pub fn retain(&mut self, f: impl Fn(&TemporaryStatusBonus) -> bool) {
        self.0.retain(f)
    }

    /// This should be used only internally or when loading bonuses from database, otherwise we can stack bonuses which are not supposed to be stacked
    pub fn add(&mut self, temporary_bonus: TemporaryStatusBonus) {
        self.0.push(temporary_bonus)
    }
}

impl From<TemporaryStatusBonuses> for Vec<TemporaryStatusBonus> {
    fn from(val: TemporaryStatusBonuses) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatusBonusSource {
    Skill(u16),
    PassiveSkill(u16),
    Item(u32)
}

impl StatusBonusSource {
    pub fn serialize_to_sc_data(&self) -> (&'static str, i32) {
        match self {
            StatusBonusSource::Skill(v) => ("Skill", *v as i32),
            StatusBonusSource::PassiveSkill(v) => ("PassiveSkill", *v as i32),
            StatusBonusSource::Item(v) => ("Item", *v as i32),
        }
    }

    pub fn deserialize_sc_data(source: &str, value: i32) -> Option<Self> {
        match source {
            "Skill" => Some(StatusBonusSource::Skill(value as u16)),
            "PassiveSkill" => Some(StatusBonusSource::PassiveSkill(value as u16)),
            "Item" => Some(StatusBonusSource::Item(value as u32)),
            _ =>  None
        }
    }
}


#[derive(WithMaskValueU64, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum StatusBonusFlag {
    #[mask_value = 0]
    Default,
    Unique, // Meaning that the same BonusType with the same source, cannot be present more than once in StatusBonuses, when adding an already present BonusType, the new one will replace the old one
    Persist,
    Icon, // If there is an icon to display on client side
}

impl StatusBonus {
    pub fn new(bonus: BonusType) -> StatusBonus {
        Self {
            bonus
        }
    }

}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BonusExpiry {
    Never,
    Time(u128),
    Counter(u64), // counter can be a number of hit (eg: Safety wall) or an amount of damage received (eg: kyrie eleison)
}

/// Hold the state of status bonus
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatusBonusState {
    No,
    Counter(u64) // counter can be a number of hit (eg: Safety wall) or an amount of damage received (eg: kyrie eleison)
}

#[cfg(test)]
mod tests {
    use crate::enums::bonus::BonusType;
    use crate::enums::EnumWithMaskValueU64;
    use crate::status_bonus::{StatusBonus, StatusBonusFlag, StatusBonusSource, TemporaryStatusBonus, TemporaryStatusBonuses};

    #[test]
    fn size_of_status_bonus() {
        let bonus = StatusBonus::new(BonusType::Str(10));
        println!("{}", std::mem::size_of_val(&bonus));
    }

    #[test]
    fn test_merge_empty_collections() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let bonuses2 = TemporaryStatusBonuses::empty();
        
        bonuses1.merge(bonuses2);
        
        assert!(bonuses1.is_empty());
    }

    #[test]
    fn test_merge_into_empty() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let mut bonuses2 = TemporaryStatusBonuses::empty();
        
        let bonus = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(10),
            StatusBonusFlag::Persist.as_flag(),
            1000,
            5000,
            Some(StatusBonusSource::Skill(123))
        );
        bonuses2.add(bonus);
        
        bonuses1.merge(bonuses2);
        
        assert_eq!(bonuses1.0.len(), 1);
        assert_eq!(*bonuses1.0[0].source(), Some(StatusBonusSource::Skill(123)));
    }

    #[test]
    fn test_merge_from_empty() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let bonuses2 = TemporaryStatusBonuses::empty();
        
        let bonus = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Agi(15),
            StatusBonusFlag::Persist.as_flag(),
            1000,
            3000,
            Some(StatusBonusSource::Item(456))
        );
        bonuses1.add(bonus);
        
        bonuses1.merge(bonuses2);
        
        assert_eq!(bonuses1.0.len(), 1);
        assert_eq!(*bonuses1.0[0].source(), Some(StatusBonusSource::Item(456)));
    }

    #[test]
    fn test_merge_no_duplicates() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let mut bonuses2 = TemporaryStatusBonuses::empty();
        
        // Add different bonuses to each collection
        let bonus1 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(10),
            StatusBonusFlag::Persist.as_flag(),
            1000,
            5000,
            Some(StatusBonusSource::Skill(123))
        );
        let bonus2 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Agi(15),
            StatusBonusFlag::Persist.as_flag(),
            1000,
            3000,
            Some(StatusBonusSource::Item(456))
        );
        
        bonuses1.add(bonus1);
        bonuses2.add(bonus2);
        
        bonuses1.merge(bonuses2);
        
        assert_eq!(bonuses1.0.len(), 2);
        assert_eq!(*bonuses1.0[0].source(), Some(StatusBonusSource::Skill(123)));
        assert_eq!(*bonuses1.0[1].source(), Some(StatusBonusSource::Item(456)));
    }

    #[test]
    fn test_merge_with_same_source_and_bonus_replacement() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let mut bonuses2 = TemporaryStatusBonuses::empty();
        
        // Create bonuses with same source and same bonus type but different values
        let bonus1 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(10),
            StatusBonusFlag::Persist.as_flag(),
            1000,
            5000,
            Some(StatusBonusSource::Skill(123))
        );
        let bonus2 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(20), // Same bonus type, different value
            StatusBonusFlag::Icon.as_flag(), // Different flags
            2000,
            8000,
            Some(StatusBonusSource::Skill(123)) // Same source
        );
        
        bonuses1.add(bonus1);
        bonuses2.add(bonus2);
        
        bonuses1.merge(bonuses2);
        
        // Should have only 1 bonus (replacement occurred)
        assert_eq!(bonuses1.0.len(), 1);
        assert_eq!(*bonuses1.0[0].source(), Some(StatusBonusSource::Skill(123)));
        // Verify the new values replaced the old ones
        if let BonusType::Str(val) = bonuses1.0[0].bonus() {
            assert_eq!(*val, 20); // Should be the new value
        } else {
            panic!("Expected Str bonus type");
        }
        assert_eq!(bonuses1.0[0].flags(), StatusBonusFlag::Icon.as_flag());
    }

    #[test]
    fn test_merge_with_same_source_different_bonus_no_replacement() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let mut bonuses2 = TemporaryStatusBonuses::empty();
        
        // Create bonuses with same source but different bonus types
        let bonus1 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(10),
            StatusBonusFlag::Persist.as_flag(),
            1000,
            5000,
            Some(StatusBonusSource::Skill(123))
        );
        let bonus2 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Agi(15), // Different bonus type
            StatusBonusFlag::Icon.as_flag(),
            2000,
            8000,
            Some(StatusBonusSource::Skill(123)) // Same source
        );
        
        bonuses1.add(bonus1);
        bonuses2.add(bonus2);
        
        bonuses1.merge(bonuses2);
        
        // Should have 2 bonuses (no replacement since bonus types are different)
        assert_eq!(bonuses1.0.len(), 2);
        assert_eq!(*bonuses1.0[0].source(), Some(StatusBonusSource::Skill(123)));
        assert_eq!(*bonuses1.0[1].source(), Some(StatusBonusSource::Skill(123)));
        // Verify both bonus types are present
        if let BonusType::Str(val) = bonuses1.0[0].bonus() {
            assert_eq!(*val, 10); // Original value
        } else {
            panic!("Expected Str bonus type");
        }
        if let BonusType::Agi(val) = bonuses1.0[1].bonus() {
            assert_eq!(*val, 15); // New value
        } else {
            panic!("Expected Agi bonus type");
        }
    }

    #[test]
    fn test_merge_mixed_scenarios() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let mut bonuses2 = TemporaryStatusBonuses::empty();
        
        // bonuses1: [Skill(1)+Str, Item(2)+Agi, Skill(3)+Vit]
        bonuses1.add(TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(10), StatusBonusFlag::Persist.as_flag(), 1000, 5000,
            Some(StatusBonusSource::Skill(1))
        ));
        bonuses1.add(TemporaryStatusBonus::with_duration_and_source(
            BonusType::Agi(15), StatusBonusFlag::Persist.as_flag(), 1000, 3000,
            Some(StatusBonusSource::Item(2))
        ));
        bonuses1.add(TemporaryStatusBonus::with_duration_and_source(
            BonusType::Vit(20), StatusBonusFlag::Persist.as_flag(), 1000, 7000,
            Some(StatusBonusSource::Skill(3))
        ));
        
        // bonuses2: [Skill(1)+Str, Item(4)+Dex] - one replacement (same source+bonus), one new
        bonuses2.add(TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(25), StatusBonusFlag::Icon.as_flag(), 2000, 6000,
            Some(StatusBonusSource::Skill(1)) // Should replace first bonus (same source and bonus type)
        ));
        bonuses2.add(TemporaryStatusBonus::with_duration_and_source(
            BonusType::Dex(30), StatusBonusFlag::Persist.as_flag(), 1000, 4000,
            Some(StatusBonusSource::Item(4)) // Should be added as new
        ));
        
        bonuses1.merge(bonuses2);
        
        // Should have 4 bonuses: [Skill(1)+Str-replaced, Item(2)+Agi, Skill(3)+Vit, Item(4)+Dex-new]
        assert_eq!(bonuses1.0.len(), 4);
        
        // Check that Skill(1)+Str was replaced at position 0
        assert_eq!(*bonuses1.0[0].source(), Some(StatusBonusSource::Skill(1)));
        if let BonusType::Str(val) = bonuses1.0[0].bonus() {
            assert_eq!(*val, 25); // Should be the new value
        } else {
            panic!("Expected Str bonus type");
        }
        assert_eq!(bonuses1.0[0].flags(), StatusBonusFlag::Icon.as_flag());
        
        // Check that Item(2)+Agi is still at position 1
        assert_eq!(*bonuses1.0[1].source(), Some(StatusBonusSource::Item(2)));
        if let BonusType::Agi(val) = bonuses1.0[1].bonus() {
            assert_eq!(*val, 15);
        } else {
            panic!("Expected Agi bonus type");
        }
        
        // Check that Skill(3)+Vit is still at position 2
        assert_eq!(*bonuses1.0[2].source(), Some(StatusBonusSource::Skill(3)));
        if let BonusType::Vit(val) = bonuses1.0[2].bonus() {
            assert_eq!(*val, 20);
        } else {
            panic!("Expected Vit bonus type");
        }
        
        // Check that Item(4)+Dex was added at position 3
        assert_eq!(*bonuses1.0[3].source(), Some(StatusBonusSource::Item(4)));
        if let BonusType::Dex(val) = bonuses1.0[3].bonus() {
            assert_eq!(*val, 30);
        } else {
            panic!("Expected Dex bonus type");
        }
    }

    #[test]
    fn test_merge_with_none_sources() {
        let mut bonuses1 = TemporaryStatusBonuses::empty();
        let mut bonuses2 = TemporaryStatusBonuses::empty();
        
        // Create bonuses with no source
        let bonus1 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Str(10), StatusBonusFlag::Persist.as_flag(), 1000, 5000, None
        );
        let bonus2 = TemporaryStatusBonus::with_duration_and_source(
            BonusType::Agi(15), StatusBonusFlag::Persist.as_flag(), 1000, 3000, None
        );
        
        bonuses1.add(bonus1);
        bonuses2.add(bonus2);
        
        bonuses1.merge(bonuses2);

        assert_eq!(bonuses1.0.len(), 2);
        assert_eq!(*bonuses1.0[0].source(), None);
        assert_eq!(*bonuses1.0[1].source(), None);
    }
}
