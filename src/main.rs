mod command;
mod config;
mod library;
mod misc;
mod model;
mod platform;
fn main() {
    misc::init_logger_configuration();
    command::Cli::run();
}
