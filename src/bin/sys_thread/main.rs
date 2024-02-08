use std::thread;
use std::sync::{Arc, Mutex};

use std::collections::VecDeque;

use std::time::Duration;

const NUM_TASKS: usize = 200;
const NUM_THREADS: usize = 6;
const MAX_PARAM: usize = 10_000_000;

fn expensive_calculation() {
    let _: Vec<_> = (0..MAX_PARAM).map(|n| f64::sqrt(n as f64)).collect();
}

struct Task {
    parameter: usize
}

impl Task {
    // Some expensive calculation
    fn execute(&self) {
        expensive_calculation();
        println!("Thread {} executed task {}", thread::current().name().unwrap(), self.parameter);
    }
}

fn task_handler(task_queue: Arc<Mutex<VecDeque<Task>>>) {
    loop {
        // RAII drops the mutex
        let task = task_queue.lock().expect("Couldn't lock task queue in child").pop_front();

        if let Some(task) = task {
            task.execute();
        } else {
            thread::sleep(Duration::from_millis(1));
        }
    }
}

fn main() {
    let task_queue = Arc::from(Mutex::new(VecDeque::<Task>::new()));

    // Spawn child thread(s)
    let handles: Vec<thread::JoinHandle<_>> = (0..NUM_THREADS).map(|id| {
        let task_queue_clone = task_queue.clone();

        std::thread::Builder::new()
            .name(format!("{id}"))
            .spawn(|| task_handler(task_queue_clone))
            .unwrap()
    }).collect();

    // Randomly queue tasks
    for i in 0..NUM_TASKS {
        let mut queue = task_queue.lock().expect("Couldn't lock task queue in main");

        queue.push_back(Task { parameter: i });

        // A potentially expensive calculation follows
        println!("\tQueued task {i}");
    }

    // Collect child threads and exit
    // Actually this never returns because each child thread is in an infinite loop
    // Ah well
    for handle in handles {
        handle.join().unwrap();
    }
}
