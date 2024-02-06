use std::thread;
use std::sync::{Arc, Mutex};

use std::collections::VecDeque;

use std::time::Duration;
use rand::Rng;


struct Task {
    id: usize
}

impl Task {
    // Some expensive calculation
    fn execute(&self) {
        thread::sleep(Duration::from_millis((self.id * 10) as u64));
    }
}

const NUM_TASKS: usize = 200;
const NUM_THREADS: usize = 5;
const MIN_WAIT_TIME_MILLIS: u64 = 10;
const MAX_WAIT_TIME_MILLIS: u64 = 40;


fn main() {
    let task_queue = Arc::from(Mutex::new(VecDeque::<Task>::new()));

    // Spawn child thread(s)
    let handles: Vec<thread::JoinHandle<_>> = (0..NUM_THREADS).map(|id| {
        let task_queue_clone = task_queue.clone();
        std::thread::Builder::new().name(format!("{id}")).spawn(move || {
            loop {
                let mut queue = task_queue_clone.lock().expect("Couldn't lock task queue in child");
                let task = queue.pop_front();

                // Free the lock early
                drop(queue);

                if let Some(task) = task {
                    task.execute();
                    println!("Thread {} executed task {}", thread::current().name().unwrap(), task.id);
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }).unwrap()
    }).collect();

    // Randomly queue tasks
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_TASKS {
        let sleep_duration = rng.gen_range(MIN_WAIT_TIME_MILLIS..MAX_WAIT_TIME_MILLIS);
        thread::sleep(Duration::from_millis(sleep_duration));

        let mut queue = task_queue.lock().expect("Couldn't lock task queue in main");

        queue.push_back(Task { id: sleep_duration as usize });

        let new_len = queue.len();

        // Free the lock early
        drop(queue);

        // A potentially expensive calculation follows
        println!("\tQueued task {sleep_duration}, queue is now {} long", new_len);
    }

    // Collect child threads and exit
    // Actually this never returns because each child thread is in an infinite loop
    // Ah well
    for handle in handles {
        handle.join().unwrap();
    }
}
