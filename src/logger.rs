use lazy_static::lazy_static;
lazy_static! {
    pub static ref LOGGER: AceLogger = AceLogger::new();
}
pub struct AceLogger {}
impl AceLogger {
    pub fn new() -> AceLogger {
        AceLogger {}
    }
    #[allow(dead_code)]
    pub fn info(&self, info: &str) {
        log::info!("{}", info);
    }
    #[allow(dead_code)]
    pub fn error(&self, info: &str) {
        log::error!("{}", info);
    }
    #[allow(dead_code)]
    pub fn warn(&self, info: &str) {
        log::warn!("{}", info);
    }
    #[allow(dead_code)]
    pub fn debug(&self, info: &str) {
        log::debug!("{}", info);
    }
    #[allow(dead_code)]
    pub fn trace(&self, info: &str) {
        log::trace!("{}", info);
    }
}
