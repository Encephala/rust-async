use async_std::task::block_on;
// https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
use futures::future::join_all;

const NUM_TASKS: usize = 200;
const NUM_THREADS: usize = 6;
const MAX_PARAM: usize = 10_000_000;

fn expensive_calculation() {
    let _: Vec<_> = (0..MAX_PARAM).map(|n| f64::sqrt(n as f64)).collect();
}

struct Task {
    id: usize
}

impl Task {
    // Some expensive calculation
    async fn execute(&self) -> usize {
        expensive_calculation();
        println!("Executed task {}", self.id);
        return self.id;
    }
}

async fn process_task(task: Task) -> usize {
    return task.execute().await;
}

async fn spawn_tasks(runtime: tokio::runtime::Runtime) {
    // Spawn tasks, collect future
    let task_futures: Vec<_> = (0..NUM_TASKS).map(|i| {
        println!("\tQueued task {i}");

        runtime.spawn(process_task(Task { id: i }))
    }).collect();

    // Execute all tasks
    println!("Joined everything: {:?}", join_all(task_futures).await);
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(NUM_THREADS)
        .enable_all()
        .build()
        .unwrap();

    block_on(spawn_tasks(runtime));
}
