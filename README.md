# rust-async

Project to play with multiple ways to manage (dummy) asynchronous workloads.
First using system threads, then async/await, then Tokio?

## Performance

Sys threads naturally is parallel, async/await isn't, but Tokio can allow it to be.

Performance:
Workload is to calculate the `f64` square root of the floats `0..10_000_000`. This is done by `N` tasks.
Measurements weren't repeated because. Some light processes running in background but twelve virtual cores available,
so those shouldn't interfere with performance too much, even when using ten threads.

(i7 8700K CPU; DDR4 3000 MHz RAM)

(Using `cargo build; cargo run` and `coreutils`' `time`, surely `cargo run` has negligible overhead)

### `N = 200` tasks

| Method              | Number of threads | real time (s) | user time (s) |
| ------------------- | ----------------- | ------------- | ------------- |
| `sys_thread`        | 1                 | 33.4          | 32.6          |
| `sys_thread`        | 4                 | 10.2 (0.83)   | 39.5          |
| `sys_thread`        | 10                | 6.8 (0.49)    | 63.0          |
| `async_await`       | -                 | 33.7          | 32.8          |
| `tokio multithread` | 1                 | 33.1          | 32.4          |
| `tokio multithread` | 4                 | 10.2 (0.80)   | 39.3          |
| `tokio multithread` | 10                | 6.6 (0.50)    | 61.6          |

Multithreaded solutions suffer pretty badly from overhead judging by user time.

### `N = 1000` tasks

| Method              | Number of threads | real time (s) | user time (s) |
| ------------------- | ----------------- | ------------- | ------------- |
| `sys_thread`        | 1                 | 165.3         | 161.0         |
| `sys_thread`        | 4                 | 51.4 (0.80)   | 200.1         |
| `sys_thread`        | 10                | 31.9 (0.52)   | 308.9         |
| `async_await`       | -                 | 170.1         | 166.0         |
| `tokio multithread` | 1                 | 168.4         | 164.1         |
| `tokio multithread` | 4                 | 51.1 (0.83)   | 209.4         |
| `tokio multithread` | 10                | 31.5 (0.53)   | 303.3         |

Similar relative performance numbers.
Seems overhead is mostly constant, not just `O(1)` overhead from spawning threads.
Then again efficiency decreases when going from `4` to `10` threads, so idk.
