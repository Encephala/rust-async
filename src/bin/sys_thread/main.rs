use std::thread;
use std::sync::{Arc, Mutex};

use std::collections::VecDeque;

use std::time::Duration;
use rand::Rng;


struct Task {
    parameter: usize
}

impl Task {
    // Some expensive calculation
    fn execute(&self) {
        thread::sleep(Duration::from_millis(self.parameter as u64));
    }
}

const NUM_TASKS: usize = 200;
const NUM_THREADS: usize = 5;
const MIN_PARAM: usize = 100;
const MAX_PARAM: usize = 400;

fn task_handler(task_queue: Arc<Mutex<VecDeque<Task>>>) {
    loop {
        let mut queue = task_queue.lock().expect("Couldn't lock task queue in child");
        let task = queue.pop_front();

        // Free the lock early while executing
        drop(queue);

        if let Some(task) = task {
            task.execute();
            println!("Thread {} executed task {}", thread::current().name().unwrap(), task.parameter);
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
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_TASKS {
        let parameter = rng.gen_range(MIN_PARAM..MAX_PARAM);

        let mut queue = task_queue.lock().expect("Couldn't lock task queue in main");

        queue.push_back(Task { parameter: parameter as usize });

        // A potentially expensive calculation follows
        println!("\tQueued task {parameter}");
    }

    // Collect child threads and exit
    // Actually this never returns because each child thread is in an infinite loop
    // Ah well
    for handle in handles {
        handle.join().unwrap();
    }
}
