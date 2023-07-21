use sqlite;
use std::fs::create_dir_all;
use std::io::Write;
use std::process::exit;

pub mod http;
pub mod tool;
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
pub fn init_database_configuration() -> sqlite::ConnectionWithFullMutex {
    let pathbuf = home::home_dir().unwrap();
    let config_dir = pathbuf.join(".ace");
    create_dir_all(&config_dir).unwrap();

    let binding = config_dir.join("config.db");
    let config_path = binding.as_path();
    let connection: sqlite::ConnectionWithFullMutex = match sqlite::Connection::open_with_full_mutex(config_path) {
        Ok(conn) => conn,
        Err(info) => {
            log::error!("{}", info);
            exit(1);
        }
    };
    let query = "
    CREATE TABLE IF NOT EXISTS global (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, value TEXT);
    CREATE TABLE IF NOT EXISTS account (id INTEGER PRIMARY KEY AUTOINCREMENT, platform TEXT, username TEXT, password TEXT, cookies TEXT, current INTEGER DEFAULT 0);
";
    connection.execute(query).unwrap();
    connection
}
