use std::time::{Instant};

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}


fn main() {
    let start = Instant::now();
    fibonacci(35);

    println!("Time: {:?}", start.elapsed().as_secs());
}
