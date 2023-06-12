use once_cell::sync::Lazy;
use sqlite::ConnectionWithFullMutex;
mod command;
mod config;
mod misc;
mod platform;
pub static CONN: Lazy<ConnectionWithFullMutex> = Lazy::new(|| misc::init_database_configuration());
fn main() {
    misc::init_logger_configuration();
    command::Cli::run();
}
