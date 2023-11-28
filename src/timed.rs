use std::time::{Duration, Instant};

pub fn timed<T, F: FnOnce() -> T>(f: F) -> (Duration, T) {
    let start = Instant::now();
    let out = f();
    let elapsed = start.elapsed();
    (elapsed, out)
}
