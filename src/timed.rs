use std::time::Instant;

pub fn timed<T, F: FnOnce() -> T>(description: &str, f: F) -> T {
    let start = Instant::now();
    let out = f();
    let elapsed = start.elapsed();
    println!("{} took {:?}", description, elapsed);
    out
}
