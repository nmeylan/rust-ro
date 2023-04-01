use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Write;
use std::sync::Mutex;

pub struct TasksQueue<T> {
    tasks: Mutex<VecDeque<Vec<T>>>,
}


impl <T> TasksQueue<T> where T: Debug + Clone {
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
    pub fn is_empty(&self) -> bool {
        let tasks_guard = self.tasks.lock().expect("Fail to acquire lock on tasks");
        tasks_guard.is_empty()
    }
    /// Use for test only
    pub fn content_as_str(&self) -> String {
        let tasks_guard = self.tasks.lock().expect("Fail to acquire lock on tasks");
        let mut s = String::new();
        for (index, content) in tasks_guard.iter().enumerate() {
            writeln!(s).unwrap();
            writeln!(s, "tick - {index}").unwrap();
            content.iter().for_each(|event| writeln!(s, "\t{event:?}").unwrap());
        }
        s
    }

    pub fn duplicate(&self) -> TasksQueue<T> {
        let tasks_guard = self.tasks.lock().expect("Fail to acquire lock on tasks");
        let mut cloned_tasks = VecDeque::new();
        tasks_guard.iter().for_each(|queue| cloned_tasks.push_back(queue.clone().to_vec()));
        TasksQueue{ tasks: Mutex::new(cloned_tasks) }
    }
}