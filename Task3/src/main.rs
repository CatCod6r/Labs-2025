use memoization::{EvictionPolicy, Memoizer};

mod memoization;
fn expensive_function(x: u32) -> u32 {
    println!("Computing...");
    x * x
}

fn main() {
    let memo = Memoizer::new(expensive_function, Some(3), EvictionPolicy::LRU);

    println!("{}", memo.call(4)); // Computes
    println!("{}", memo.call(4)); // Cached
    println!("{}", memo.call(5)); // Computes
}
