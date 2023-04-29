mod client;
mod codeforces;
mod config;
mod platform;
mod tool;

use std::{
    env::{self, current_dir},
    process,
};

use clap::Parser;
use client::Client;
use config::Config;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // actions: (config, submit, test)
    action: String,

    #[arg(short, long)]
    directory: Option<String>,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let args = Args::parse();
    let action = args.action;
    let path = home::home_dir().unwrap();
    let mut config = Config::new(&path.as_path());
    let cur = match current_dir() {
        Ok(path) => path,
        Err(err) => {
            log::error!("Get work dir failed. {}", err);
            process::exit(1);
        }
    };
    let cur_path = cur.as_path();
    println!("{}", cur_path.to_str().unwrap());
}
