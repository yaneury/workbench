use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Duration, Instant};

pub struct TaskScheduler {
    tasks: Arc<Mutex<BinaryHeap<Reverse<Task>>>>,
    looper: JoinHandle<()>,
    next_id: AtomicU64,
}

impl TaskScheduler {
    pub fn new() -> TaskScheduler {
        let tasks = Arc::new(Mutex::new(BinaryHeap::<Reverse<Task>>::new()));
        let tasks_clone = tasks.clone();
        let (tx, rw) = mpsc::channel::<()>();

        let looper = spawn(move || {
            let mut tasks = tasks_clone;

            loop {
                let now = Instant::now();

                {
                    let tasks = tasks.lock().unwrap();

                    match tasks.peek() {
                        None => {
                            sleep(Duration::from_millis(10));
                            continue;
                        }
                        Some(Reverse(task)) if task.due_at > now => {
                            sleep(task.due_at - now);
                            continue;
                        }
                        _ => {} // Task is due, fall through
                    }
                }

                let mut tasks = tasks.lock().unwrap();

                if let Some(Reverse(mut task)) = tasks.pop() {
                    let runnable = Arc::clone(&task.runnable);
                    spawn(move || runnable());
                    task.due_at = now + task.cadence;
                    tasks.push(Reverse(task));
                }
            }
        });

        TaskScheduler {
            tasks,
            looper,
            next_id: AtomicU64::new(0),
        }
    }

    pub fn schedule(&mut self, f: impl Fn() + Send + Sync + 'static, cadence: Duration) {
        let runnable = Arc::new(f);
        assert!(!cadence.is_zero());

        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(Reverse(Task {
            runnable,
            cadence,
            due_at: Instant::now() + cadence,
        }));
    }
}

type Runnable = Arc<dyn Fn() + Send + Sync>;

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
