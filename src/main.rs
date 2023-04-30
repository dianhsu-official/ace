mod codeforces;
mod command;
mod config;
mod tool;
use std::fs::create_dir_all;
use std::io::Write;

use command::Cli;
use config::Config;

fn main() {
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
        .filter_level(log::LevelFilter::Debug)
        .init();

    let pathbuf = home::home_dir().unwrap();
    let config_dir = pathbuf.join(".ace");
    create_dir_all(&config_dir).unwrap();

    let binding = config_dir.join("config.yaml");
    let config_path = binding.as_path();
    let mut config = Config::new(&config_path);

    Cli::run(&mut config, &config_path);
    config.save(&config_dir.join("config.yaml")).unwrap();
}
