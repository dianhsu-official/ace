use std::io::Write;

pub mod http_client;
pub mod utility;
pub fn init_logger_configuration() {
    #[cfg(debug_assertions)]
    env_logger::builder()
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_bold(true);
            writeln!(
                buf,
                "[{}:{}] [{}] [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                style.value(record.args())
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();
    #[cfg(not(debug_assertions))]
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}:{}] [{}] [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();
}
