mod command;
mod constants;
mod context;
mod database;
mod model;
mod platform;
mod snippet;
mod traits;
mod utility;
fn main() {
    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
