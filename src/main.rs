mod command;
mod config;
mod misc;
mod platform;
fn main() {
    misc::init_logger_configuration();
    command::Cli::run();
}
