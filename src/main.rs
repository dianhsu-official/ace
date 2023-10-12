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
    #[cfg(not(debug_assertions))]
    let _guard = sentry::init(
        (
            "https://a7c3387690d736d2a3a8cbba70a20d7a@o4505770420862976.ingest.sentry.io/4506038010904576", 
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            }
        )
    );

    match command::Cli::run() {
        Ok(_) => {}
        Err(info) => {
            log::error!("{}", info)
        }
    }
}
