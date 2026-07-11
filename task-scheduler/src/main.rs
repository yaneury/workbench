use std::time::Duration;

mod schedule;

fn main() {
    let mut scheduler = schedule::TaskScheduler::new();

    scheduler.schedule(
        || println!("Closure A, every 1 sec"),
        Duration::from_secs(1),
    );

    scheduler.schedule(
        || println!("Closure B, every 500ms"),
        Duration::from_millis(500),
    );

    scheduler.schedule(
        || println!("Closure C, every 2 secs"),
        Duration::from_secs(2),
    );

    scheduler.schedule(
        || println!("Closure D, every 100ms"),
        Duration::from_millis(100),
    );

    std::thread::sleep(Duration::from_secs(10));
}
