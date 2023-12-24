mod command;
mod constants;
mod context;
mod database;
mod model;
mod platform;
mod snippet;
mod utility;
#[tokio::main]
async fn main() {
    match command::Cli::run().await {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
