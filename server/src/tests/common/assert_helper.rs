use std::sync::Arc;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::tasks_queue::TasksQueue;

pub fn task_queue_contains_event_at_tick(task_queue: Arc<TasksQueue<GameEvent>>, expected_event: GameEvent, tick: usize) {
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