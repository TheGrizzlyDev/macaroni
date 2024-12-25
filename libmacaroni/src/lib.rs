#![feature(c_size_t)]

use std::{env, fs, path::PathBuf};

use ctor::ctor;
use libc_interposition_lib::InterposeEntry;

mod cwd;
mod config;
mod open_close;
mod read_write;
mod permissions;
mod directory;

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_TABLE: [InterposeEntry; 1] = [
    cwd::getcwd::INTERPOSE_ENTRY,
];

#[ctor]
fn init() {
    let sandbox_path = match env::var("MACARONI_SANDBOX_PATH") {
        Ok(path_str) => {
            PathBuf::from(path_str)
        }
        Err(e) => {
            panic!("MACARONI_SANDBOX_PATH not set or invalid: {}", e)
        }
    };

    println!("Sandbox path is: {:?}", sandbox_path);

    let mut config_path = sandbox_path.clone();
    config_path.push("config.json");

    let config_raw = fs::read_to_string(config_path).unwrap();
    let config: config::Config = serde_json::from_str(&config_raw).unwrap();

    println!("Config is: {:?}", config);

}