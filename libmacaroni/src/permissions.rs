use core::ffi::{c_int, c_char, c_uint, c_void};
use libc_interposition_lib::LibcResult;
use libc_interposition_macro::interpose;

#[interpose]
pub fn chmod(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn fchmod(fd: c_int, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn lchmod(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn fchmodat(fd: c_int, path: *const c_char, mode: c_uint, flag: c_int) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn chown(path: *const c_char, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn fchown(fd: c_int, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn lchown(path: *const c_char, owner: c_uint, group: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn fchownat(
    fd: c_int,
    path: *const c_char,
    owner: c_uint,
    group: c_uint,
    flag: c_int,
) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn umask(mask: c_int) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn utimes(path: *const c_char, times: *const c_void) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn futimes(fd: c_int, times: *const c_void) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn lutimes(path: *const c_char, times: *const c_void) -> LibcResult<c_int> {
    todo!()
}
