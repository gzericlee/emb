use std::io;
use env_logger::{Env};
use io::Write;

pub fn init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("error"))
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] [{}:{}]: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap(),
                record.line().unwrap(),
                record.args()
            )
        }).init();
}