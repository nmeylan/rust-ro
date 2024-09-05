use accessor::GettersAll;
use enum_macro::{WithMaskValueU64, WithStringValue};
use crate::enums::bonus::BonusType;
use crate::enums::{EnumWithMaskValueU64, EnumWithStringValue};


#[derive(Default, Debug, Clone)]
pub struct StatusBonuses(Vec<StatusBonus>);

#[derive(GettersAll, Debug, Clone, Copy)]
pub struct StatusBonus {
    bonus: BonusType,
    flags: u64,
}

#[derive(GettersAll, Debug, Clone, Copy)]
pub struct TemporaryStatusBonus {
    bonus: BonusType,
    flags: u64,
    expirency: BonusExpiry,
    state: StatusBonusState,
    source: Option<u16>,
}

impl TemporaryStatusBonus {
    pub fn with_duration(bonus: BonusType, flags: u64, tick: u128, duration: u32, skill_id: u16) -> Self {
        Self {
            bonus,
            flags,
            expirency: BonusExpiry::Time(tick + duration as u128),
            state: StatusBonusState::No,
            source: Some(skill_id),
        }
    }

    pub fn with_passive_skill(bonus: BonusType, flags: u64, skill_id: u16) -> Self {
        Self {
            bonus,
            flags,
            expirency: BonusExpiry::Never,
            state: StatusBonusState::No,
            source: Some(skill_id)
        }
    }

    pub fn has_icon(&self) -> bool {
        self.flags & StatusBonusFlag::Icon.as_flag() > 0
    }

    pub fn icon(&self) -> Option<u16> {
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

#[derive(Debug, Clone, Copy)]
pub enum StatusBonusSource {
    Equipment,
    PassiveSkill,
    TargetSkill,
    SelfSkill,
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
            bonus,
            flags: StatusBonusFlag::Default.as_flag(),
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub enum BonusExpiry {
    Never,
    Time(u128),
    Counter(u64), // counter can be a number of hit (eg: Safety wall) or an amount of damage received (eg: kyrie eleison)
}

/// Hold the state of status bonus
#[derive(Debug, Clone, Copy)]
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