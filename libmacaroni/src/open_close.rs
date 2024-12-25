use libc_interposition_macro::interpose;
use core::ffi::{c_char, c_int};
use libc_interposition_lib::LibcResult;

#[interpose]
pub fn open(path: *const c_char, oflag: c_int, mode: c_int) -> LibcResult<c_int> {
    // Interpose logic here...
    // E.g. call the original libc open, log, modify arguments, etc.
    // unsafe { real::open(path, oflag, mode) }
    todo!();
}

#[interpose]
pub fn openat(fd: c_int, path: *const c_char, oflag: c_int, mode: c_int) -> LibcResult<c_int> {
    todo!();
}

#[interpose]
pub fn creat(path: *const c_char, mode: c_int) -> LibcResult<c_int> {
    todo!();
}

#[interpose]
pub fn close(fd: c_int) -> LibcResult<c_int> {
    todo!();
}

#[interpose]
pub fn dup(oldfd: c_int) -> LibcResult<c_int> {
    todo!();
}

#[interpose]
pub fn dup2(oldfd: c_int, newfd: c_int) -> LibcResult<c_int> {
    todo!();
}

#[interpose]
pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> LibcResult<c_int> {
    todo!();
}
