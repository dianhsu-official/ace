mod command;
mod constants;
mod context;
mod database;
mod misc;
mod model;
mod platform;
mod snippet;
mod traits;
fn main() {
    simple_logger::init().unwrap();
    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
