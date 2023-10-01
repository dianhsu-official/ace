mod command;
mod constants;
mod context;
mod database;
mod logger;
mod misc;
mod model;
mod platform;
mod snippet;
mod traits;
fn main() {
    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
