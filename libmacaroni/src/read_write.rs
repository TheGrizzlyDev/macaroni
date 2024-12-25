use core::ffi::{c_int, c_long, c_size_t, c_void};
use libc_interposition_lib::LibcResult;
use libc_interposition_macro::interpose;

#[interpose]
pub fn read(fd: c_int, buf: *mut c_void, count: c_size_t) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn write(fd: c_int, buf: *const c_void, count: c_size_t) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn pread(fd: c_int, buf: *mut c_void, count: c_size_t, offset: i64) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn pwrite(fd: c_int, buf: *const c_void, count: c_size_t, offset: i64) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn readv(fd: c_int, iov: *const c_void, iovcnt: c_int) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn writev(fd: c_int, iov: *const c_void, iovcnt: c_int) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn preadv(fd: c_int, iov: *const c_void, iovcnt: c_int, offset: i64) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn pwritev(fd: c_int, iov: *const c_void, iovcnt: c_int, offset: i64) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn lseek(fd: c_int, offset: i64, whence: c_int) -> LibcResult<i64> {
    todo!()
}

#[interpose]
pub fn fsync(fd: c_int) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn fdatasync(fd: c_int) -> LibcResult<c_int> {
    todo!()
}
