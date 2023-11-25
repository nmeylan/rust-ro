#![allow(dead_code)]
use crate::*;
#[derive(WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Effect {
    #[value = 0]
    BaseLevelUp,
    JobLevelUp,
    RefineFailure,
    RefineSuccess,
    GameOver,
    PharmacySuccess,
    PharmacyFailure,
    SuperNoviceBaseLevelUp,
    SuperNoviceJobLevelUp,
    TaekwonBaseLevelUp,
}
