#![allow(dead_code)]

pub enum ActionType {
    Attack,
    Itempickup,
    Sit,
    Stand,
    AttackNomotion,
    Splash,
    Skill,
    AttackRepeat,
    AttackMultiple,
    AttackMultipleNomotion,
    AttackCritical,
    AttackLucky,
    Touchskill
}

impl ActionType {
    pub fn value(&self) -> u8 {
        match *self {
            ActionType::Attack => 0,
            ActionType::Itempickup => 1,
            ActionType::Sit => 2,
            ActionType::Stand => 3,
            ActionType::AttackNomotion => 4,
            ActionType::Splash => 5,
            ActionType::Skill => 6,
            ActionType::AttackRepeat => 7,
            ActionType::AttackMultiple => 8,
            ActionType::AttackMultipleNomotion => 9,
            ActionType::AttackCritical => 10,
            ActionType::AttackLucky => 11,
            ActionType::Touchskill => 12
        }
    }
}