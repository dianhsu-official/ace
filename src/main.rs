use log::LevelFilter;

mod command;
mod constants;
mod context;
mod database;
mod model;
mod platform;
mod snippet;
mod traits;
mod utility;
use std::io::Write;
fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}:{}] [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .filter(Some("ace"), LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Auto)
        .init();
    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
