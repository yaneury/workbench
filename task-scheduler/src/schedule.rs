use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::atomic::AtomicU64;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{JoinHandle, spawn};
use std::time::{Duration, Instant};

pub struct TaskScheduler {
    tasks: Arc<Mutex<BinaryHeap<Reverse<Task>>>>,
    looper: JoinHandle<()>,
    next_id: AtomicU64,
    wakeup_tx: Sender<()>,
}

impl TaskScheduler {
    pub fn new() -> TaskScheduler {
        let tasks = Arc::new(Mutex::new(BinaryHeap::<Reverse<Task>>::new()));
        let tasks_clone = tasks.clone();
        let (wakeup_tx, wakeup_rx) = mpsc::channel::<()>();

        let looper = spawn(move || {
            let tasks = tasks_clone;

            let sleep = move |duration: Duration| {
                match wakeup_rx.recv_timeout(duration) {
                    Ok(()) => {}
                    _ => {} // Timedout
                };
            };

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
            wakeup_tx,
        }
    }

    pub fn schedule(&mut self, f: impl Fn() + Send + Sync + 'static, cadence: Duration) {
        let runnable = Arc::new(f);
        assert!(!cadence.is_zero());

        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let mut tasks = self.tasks.lock().unwrap();
        tasks.push(Reverse(Task {
            handle: Handle { id },
            runnable,
            cadence,
            due_at: Instant::now() + cadence,
        }));
        let _ = self.wakeup_tx.send(());
    }

    pub fn deschedule(&mut self, handle: Handle) -> bool {
        let mut tasks = self.tasks.lock().unwrap();

        let pre_filter_len = tasks.len();
        tasks.retain(|Reverse(task)| task.handle.id != handle.id);
        let post_filter_len = tasks.len();

        return pre_filter_len > post_filter_len;
    }
}

type Runnable = Arc<dyn Fn() + Send + Sync>;

pub struct Handle {
    id: u64,
}

struct Task {
    handle: Handle,
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
