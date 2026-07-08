use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};

type Runnable = Box<dyn Fn()>;

struct Task {
    runnable: Runnable,
    cadence: Duration,
    due_at: Instant,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.due_at.cmp(&other.due_at)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.due_at == other.due_at
    }
}

pub struct TaskScheduler {
    tasks: Arc<Mutex<BinaryHeap<Reverse<Task>>>>,
    looper: JoinHandle<()>,
}

impl TaskScheduler {
    pub fn new() -> TaskScheduler {
        let tasks = Arc::new(Mutex::new(BinaryHeap::new()));

        let tasks_clone = tasks.clone();
        let looper = spawn(|| {
            let mut tasks = tasks_clone;

            loop {
                let now = Instant::now();
                let mut tasks = tasks.lock().unwrap();

                if let Some(task) = tasks.peek() {
                    let Reverse(t) = task;
                    if t.due_at > now {
                        sleep(t.due_at - now);
                    } else {
                    }
                }
            }
        });

        TaskScheduler { tasks, looper }
    }

    pub fn schedule(&mut self, f: Box<dyn Fn()>, cadence: Duration) {
        assert!(!cadence.is_zero());

        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(Reverse(Task {
            runnable: f,
            cadence,
            due_at: Instant::now() + cadence,
        }));
    }
}

/*
* If I schedule, every 500ms. I need to fire at least
*
*
*/
