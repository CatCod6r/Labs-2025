mod log_wrap;
mod logger;

use log::Level;
use log_wrap::{log_wrap, log_wrap_async};
use logger::init_logger;

#[tokio::main]
async fn main() {
    //Use: RUST_LOG=info cargo run
    init_logger();

    let add = log_wrap("add", Level::Info, |(a, b): (i32, i32)| a + b);
    println!("add result: {}", add((2, 3)));

    let delayed_add = log_wrap_async(
        "delayed_add",
        Level::Info,
        |(a, b): (i32, i32)| async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            a + b
        },
    );

    println!("delayed_add result: {}", delayed_add((5, 7)).await);
}
