#![feature(c_size_t)]

use ctor::ctor;
use libc_interposition_lib::InterposeEntry;
mod config;
mod path_remapper;
mod filesystem;
mod exec;

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_TABLE: [InterposeEntry; 56] = [
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
    filesystem::getxattr::INTERPOSE_ENTRY,
    filesystem::setxattr::INTERPOSE_ENTRY,
    filesystem::removexattr::INTERPOSE_ENTRY,
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
    exec::execlp::INTERPOSE_ENTRY,
    exec::execv::INTERPOSE_ENTRY,
    exec::execve::INTERPOSE_ENTRY,
    exec::execvp::INTERPOSE_ENTRY,
    exec::popen::INTERPOSE_ENTRY,
    exec::posix_spawn::INTERPOSE_ENTRY,
    exec::posix_spawnp::INTERPOSE_ENTRY,
    exec::system::INTERPOSE_ENTRY,
];

#[ctor]
fn init() {
    println!("libmacaroni loaded");
}