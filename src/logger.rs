use lazy_static::lazy_static;
use std::io::Write;
lazy_static! {
    pub static ref LOGGER: AceLogger = AceLogger::new();
}
#[derive(Clone, Copy)]
pub enum LogLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}
pub struct AceLogger {
    log_level: LogLevel,
}
impl AceLogger {
    pub fn new() -> AceLogger {
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
            .init();
        AceLogger {
            log_level: LogLevel::Info,
        }
    }
    #[allow(dead_code)]
    pub fn info(&self, info: &str) {
        if self.log_level as i32 >= LogLevel::Info as i32 {
            log::info!("{}", info);
        }
    }
    #[allow(dead_code)]
    pub fn error(&self, info: &str) {
        if self.log_level as i32 >= LogLevel::Error as i32 {
            log::error!("{}", info);
        }
    }
    #[allow(dead_code)]
    pub fn warn(&self, info: &str) {
        if self.log_level as i32 >= LogLevel::Warn as i32 {
            log::warn!("{}", info);
        }
    }
    #[allow(dead_code)]
    pub fn debug(&self, info: &str) {
        if self.log_level as i32 >= LogLevel::Debug as i32 {
            log::debug!("{}", info);
        }
    }
    #[allow(dead_code)]
    pub fn trace(&self, info: &str) {
        if self.log_level as i32 >= LogLevel::Trace as i32 {
            log::trace!("{}", info);
        }
    }
}
