#![allow(dead_code)]

pub enum ActionType {
    ATTACK,
    ITEMPICKUP,
    SIT,
    STAND,
    ATTACK_NOMOTION,
    SPLASH,
    SKILL,
    ATTACK_REPEAT,
    ATTACK_MULTIPLE,
    ATTACK_MULTIPLE_NOMOTION,
    ATTACK_CRITICAL,
    ATTACK_LUCKY,
    TOUCHSKILL
}

impl ActionType {
    pub fn value(&self) -> u32 {
        match *self {
            ActionType::ATTACK => 0,
            ActionType::ITEMPICKUP => 1,
            ActionType::SIT => 2,
            ActionType::STAND => 3,
            ActionType::ATTACK_NOMOTION => 4,
            ActionType::SPLASH => 5,
            ActionType::SKILL => 6,
            ActionType::ATTACK_REPEAT => 7,
            ActionType::ATTACK_MULTIPLE => 8,
            ActionType::ATTACK_MULTIPLE_NOMOTION => 9,
            ActionType::ATTACK_CRITICAL => 10,
            ActionType::ATTACK_LUCKY => 11,
            ActionType::TOUCHSKILL => 12
        }
    }
}