mod command;
mod constants;
mod database;
mod library;
mod misc;
mod model;
mod platform;
fn main() {
    misc::init_logger_configuration();
    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
