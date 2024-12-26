#![feature(c_size_t)]

use std::{env, fs, path::PathBuf};

use ctor::ctor;
use libc_interposition_lib::InterposeEntry;

mod config;
mod filesystem;

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_TABLE: [InterposeEntry; 52] = [
    filesystem::open::INTERPOSE_ENTRY,
    filesystem::openat::INTERPOSE_ENTRY,
    filesystem::creat::INTERPOSE_ENTRY,
    filesystem::stat::INTERPOSE_ENTRY,
    filesystem::lstat::INTERPOSE_ENTRY,
    filesystem::fstatat::INTERPOSE_ENTRY,
    filesystem::chmod::INTERPOSE_ENTRY,
    filesystem::lchmod::INTERPOSE_ENTRY,
    filesystem::fchmodat::INTERPOSE_ENTRY,
    filesystem::chown::INTERPOSE_ENTRY,
    filesystem::lchown::INTERPOSE_ENTRY,
    filesystem::fchownat::INTERPOSE_ENTRY,
    filesystem::utimes::INTERPOSE_ENTRY,
    filesystem::lutimes::INTERPOSE_ENTRY,
    filesystem::mkdir::INTERPOSE_ENTRY,
    filesystem::mkdirat::INTERPOSE_ENTRY,
    filesystem::rmdir::INTERPOSE_ENTRY,
    filesystem::opendir::INTERPOSE_ENTRY,
    filesystem::link::INTERPOSE_ENTRY,
    filesystem::linkat::INTERPOSE_ENTRY,
    filesystem::unlink::INTERPOSE_ENTRY,
    filesystem::unlinkat::INTERPOSE_ENTRY,
    filesystem::symlink::INTERPOSE_ENTRY,
    filesystem::symlinkat::INTERPOSE_ENTRY,
    filesystem::readlink::INTERPOSE_ENTRY,
    filesystem::readlinkat::INTERPOSE_ENTRY,
    filesystem::rename::INTERPOSE_ENTRY,
    filesystem::renameat::INTERPOSE_ENTRY,
    filesystem::remove::INTERPOSE_ENTRY,
    filesystem::listxattr::INTERPOSE_ENTRY,
    filesystem::llistxattr::INTERPOSE_ENTRY,
    filesystem::getxattr::INTERPOSE_ENTRY,
    filesystem::lgetxattr::INTERPOSE_ENTRY,
    filesystem::setxattr::INTERPOSE_ENTRY,
    filesystem::lsetxattr::INTERPOSE_ENTRY,
    filesystem::removexattr::INTERPOSE_ENTRY,
    filesystem::lremovexattr::INTERPOSE_ENTRY,
    filesystem::access::INTERPOSE_ENTRY,
    filesystem::faccessat::INTERPOSE_ENTRY,
    filesystem::chdir::INTERPOSE_ENTRY,
    filesystem::realpath::INTERPOSE_ENTRY,
    filesystem::statfs::INTERPOSE_ENTRY,
    filesystem::truncate::INTERPOSE_ENTRY,
    filesystem::mknod::INTERPOSE_ENTRY,
    filesystem::unmount::INTERPOSE_ENTRY,
    filesystem::mkfifo::INTERPOSE_ENTRY,
    filesystem::mkfifoat::INTERPOSE_ENTRY,
    filesystem::utimensat::INTERPOSE_ENTRY,
    filesystem::getfh::INTERPOSE_ENTRY,
    filesystem::open_dprotected_np::INTERPOSE_ENTRY,
    filesystem::searchfs::INTERPOSE_ENTRY,
    filesystem::fsctl::INTERPOSE_ENTRY,
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