use std::collections::VecDeque;
use std::sync::Mutex;

pub struct TasksQueue<T> {
    tasks: Mutex<VecDeque<Vec<T>>>,
}


impl <T> TasksQueue<T> {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Default::default())
        }
    }
    pub fn add_to_first_index(&self, task: T) {
        let mut tasks_guard = self.tasks.lock().expect("Fail to acquire lock on tasks");
        if tasks_guard.len() == 0 {
            tasks_guard.push_back(vec![])
        }
        tasks_guard[0].push(task);
    }
    pub fn add_to_index(&self, task: T, index: usize) {
        let mut tasks_guard = self.tasks.lock().expect("Fail to acquire lock on tasks");
        while tasks_guard.len() <= index {
            tasks_guard.push_back(vec![])
        }
        tasks_guard[index].push(task);
    }

    pub fn pop(&self) -> Option<Vec<T>> {
        let mut tasks_guard = self.tasks.lock().expect("Fail to acquire lock on tasks");
        tasks_guard.pop_front()
    }
}