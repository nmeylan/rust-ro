use std::hash::{Hash, Hasher};
use accessor::GettersAll;
use enum_macro::{WithMaskValueU64, WithStringValue};
use crate::enums::bonus::BonusType;
use crate::enums::{EnumWithMaskValueU64, EnumWithNumberValue, EnumWithStringValue};
use crate::enums::skill_enums::SkillEnum;

#[derive(Default, Debug, Clone)]
pub struct StatusBonuses(Vec<StatusBonus>);

impl StatusBonuses {
    pub fn new(bonuses: Vec<StatusBonus>) -> Self {
        Self(bonuses)
    }
}

impl Into<Vec<StatusBonus>> for StatusBonuses {
    fn into(self) -> Vec<StatusBonus> {
        self.0
    }
}

#[derive(GettersAll, Debug, Clone, Copy, Eq, PartialEq)]
pub struct StatusBonus {
    bonus: BonusType,
}

#[derive(GettersAll, Debug, Clone, Copy, Eq, PartialEq)]
pub struct TemporaryStatusBonus {
    bonus: BonusType,
    flags: u64,
    expirency: BonusExpiry,
    state: StatusBonusState,
    source: Option<StatusBonusSource>,
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
}

#[derive(Default, Debug, Clone)]
pub struct TemporaryStatusBonuses(pub Vec<TemporaryStatusBonus>);

impl TemporaryStatusBonuses {
    pub fn empty() -> Self {
        Self { 0: vec![] }
    }
}

impl Into<Vec<TemporaryStatusBonus>> for TemporaryStatusBonuses {
    fn into(self) -> Vec<TemporaryStatusBonus> {
        self.0
    }
}
impl Into<Vec<StatusBonus>> for TemporaryStatusBonuses {
    fn into(self) -> Vec<StatusBonus> {
        self.0.iter().map(|temporary_status_bonus: &TemporaryStatusBonus| StatusBonus { bonus: temporary_status_bonus.bonus }).collect()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StatusBonusSource {
    Skill(u16),
    PassiveSkill(u16),
    Item(u32)
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
    use crate::status_bonus::StatusBonus;

    #[test]
    fn size_of_status_bonus() {
        let bonus = StatusBonus::new(BonusType::Str(10));
        println!("{}", std::mem::size_of_val(&bonus));
    }
}