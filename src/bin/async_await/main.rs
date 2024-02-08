// https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
use async_std::task::sleep;
use futures::{executor::block_on, future::join_all};

use std::time::Duration;
use rand::Rng;

struct Task {
    id: usize
}

impl Task {
    // Some expensive calculation
    async fn execute(&self) -> usize {
        sleep(Duration::from_millis(self.id as u64)).await;
        println!("Finished task {}", self.id);
        return self.id;
    }
}

const NUM_TASKS: usize = 200;
const MIN_WAIT_TIME_MILLIS: usize = 1;
const MAX_WAIT_TIME_MILLIS: usize = 5000;

async fn process_task(task: Task) -> usize {
    return task.execute().await;
}

async fn spawn_tasks() {
    let mut rng = rand::thread_rng();

    // Spawn tasks, collect future
    let task_futures: Vec<_> = (0..NUM_TASKS).map(|_| {
        let duration = rng.gen_range(MIN_WAIT_TIME_MILLIS..MAX_WAIT_TIME_MILLIS);

        println!("\tQueued task {duration}");
        process_task(Task { id: duration })
    }).collect();

    // Execute all tasks
    println!("Joined everything: {:?}", join_all(task_futures).await);
}

fn main() {
    block_on(spawn_tasks());
}
