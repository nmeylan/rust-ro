use std::fmt::{Display, Formatter};
use std::slice::{Iter, IterMut};
use std::time::{SystemTime, UNIX_EPOCH};
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

#[derive(GettersAll, Debug, Clone, Copy, Eq, PartialEq)]
pub struct TemporaryStatusBonus {
    bonus: BonusType,
    flags: u64,
    expirency: BonusExpiry,
    state: StatusBonusState,
    source: Option<StatusBonusSource>,
}

#[derive(GettersAll, Debug, Clone, Copy, Eq, PartialEq)]
pub struct TemporaryStatusBonusPersistedModel {
    bonus: BonusType,
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

    pub fn to_persisted_model(&self) {

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
pub struct TemporaryStatusBonuses(pub Vec<TemporaryStatusBonus>);

impl TemporaryStatusBonuses {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn merge(&mut self, other: TemporaryStatusBonuses) {
        // TODO handle uniq
        // TODO do not use extend in the future
        self.0.extend(other.0);
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
    use crate::status_bonus::StatusBonus;

    #[test]
    fn size_of_status_bonus() {
        let bonus = StatusBonus::new(BonusType::Str(10));
        println!("{}", std::mem::size_of_val(&bonus));
    }
}