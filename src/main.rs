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
    misc::init_logger_configuration();
    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
