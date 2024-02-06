use std::thread;
use std::sync::{Arc, Mutex};

use std::collections::VecDeque;

use std::time::Duration;
use rand::Rng;


struct Task {
    value: usize
}

impl Task {
    // Some expensive calculation
    fn execute(&self) {
        thread::sleep(Duration::from_millis(25));
    }
}

const NUM_ITER: usize = 1_000_000;
const MIN_WAIT_TIME_MILLIS: u64 = 10;
const MAX_WAIT_TIME_MILLIS: u64 = 40;


fn main() {
    let task_queue = Arc::from(Mutex::new(VecDeque::<Task>::new()));

    // Spawn child thread(s)
    let task_queue_clone = task_queue.clone();
    let thread = thread::spawn(move ||
        loop {
            let mut queue = task_queue_clone.lock().expect("Couldn't lock task queue in child");

            if let Some(task) = queue.pop_front() {
                println!("Executed for {}", task.value);
            }
        }
    );

    // Randomly queue tasks
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_ITER {
        let sleep_duration = rng.gen_range(MIN_WAIT_TIME_MILLIS..MAX_WAIT_TIME_MILLIS);
        thread::sleep(Duration::from_millis(sleep_duration));

        let mut queue = task_queue.lock().expect("Couldn't lock task queue in main");

        queue.push_back(Task { value: sleep_duration as usize });
    }

    // Collect child threads and exit
    thread.join().expect("Couldn't join thread");
}
