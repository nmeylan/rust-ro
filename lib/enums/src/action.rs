#![allow(dead_code)]
use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, WithNumberValue)]
pub enum ActionType {
    #[value = 0]
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
    Touchskill,
    AttackMultipleCritical,
}