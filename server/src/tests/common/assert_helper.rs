use std::fmt::Debug;
use std::sync::Arc;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::tasks_queue::TasksQueue;

pub fn task_queue_contains_event_at_tick<T: PartialEq + Debug>(task_queue: Arc<TasksQueue<T>>, expected_event: T, tick: usize) {
    let mut events = vec![];
    for i in 0..=tick {
        events = task_queue.pop().unwrap_or_else(|| panic!("Expected task queue to contains events at tick {}", tick));
    }
    for event in events {
        if matches!(&event, expected_event) {
            assert!(event == expected_event,"Expected {:?} == {:?}", event, expected_event);
            return;
        }
    }
    assert!(false, "Task queue does not contains event {:?}", expected_event);
}

pub fn variance(expectation: u32, variance: usize) -> u32 {
   (variance as f32 / 100 as f32).round() as u32 * expectation
}

#[macro_export]
macro_rules! assert_eq_with_variance {
    ($variance:expr, $actual:expr, $expected:expr $(,)?) => {
        let _variance = ($variance as f32 / 100 as f32) * $expected as f32;
        assert!($actual as f32 - _variance <= $expected as f32 && $expected as f32 <= $actual as f32 + _variance);
    };
    ($variance:expr,$actual:expr, $expected:expr, $($arg:tt)+) => {
        let _variance = ($variance as f32 / 100 as f32) * $expected as f32;
        assert!($actual as f32 - _variance <= $expected as f32 && $expected as f32 <= $actual as f32 + _variance, $($arg)+);
    }
}

#[macro_export]
macro_rules! assert_task_queue_contains_event_at_tick {
    ($task_queue:expr, $expected_event:expr, $tick:expr $(,)?) => {
        task_queue_contains_event_at_tick($task_queue.clone(), $expected_event, $tick)
    }
}