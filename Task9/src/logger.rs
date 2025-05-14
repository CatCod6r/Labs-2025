use chrono::Local;
use log::Record;
use std::io::Write;

pub fn init_logger() {
    env_logger::builder()
        .format(|buf, record: &Record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}

