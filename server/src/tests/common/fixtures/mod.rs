use models::status::{Status, StatusSnapshot};

use crate::tests::common::fixtures::battle_fixture::BattleFixture;

pub mod battle_fixture;

#[derive(Clone)]
pub(crate) struct TestResult {
    pub(crate) id: String,
    pub(crate) job: String,
    pub(crate) job_level: usize,
    pub(crate) passed: bool,
    pub(crate) actual_status: StatusSnapshot,
    pub(crate) actual_combat_result: Option<CombatTestResult>,
    pub(crate) expected: BattleFixture,
    pub(crate) status: Status,
    pub(crate) desc: Option<String>,
}

#[derive(Clone)]
pub(crate) struct CombatTestResult {
    pub(crate) min_dmg: i32,
    pub(crate) max_dmg: i32,
}

#[macro_export]
macro_rules! format_result {
    ($passed:expr, $arg1:expr) => {
        if $passed {
            format!("**{}**", format!("{}", $arg1))
        } else {
            format!("*{}*", format!("{}", $arg1))
        }
    };
    ($passed:expr, $arg1:expr, $arg2:expr) => {
        if $passed {
            format!("**{}**", format!("{}/{}", $arg1, $arg2))
        } else {
            format!("*{}*", format!("{}/{}", $arg1, $arg2))
        }
    };
    ($passed:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
        if $passed {
            format!("**{}**", format!("{}+{}/{}+{}", $arg1, $arg2, $arg3, $arg4))
        } else {
            format!("*{}*", format!("{}+{}/{}+{}", $arg1, $arg2, $arg3, $arg4))
        }
    };
}
