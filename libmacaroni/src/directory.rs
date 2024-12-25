use core::ffi::{c_int, c_char, c_uint, c_void, c_long};
use libc_interposition_lib::LibcResult;
use libc_interposition_macro::interpose;

#[interpose]
pub fn mkdir(path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn mkdirat(fd: c_int, path: *const c_char, mode: c_uint) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn rmdir(path: *const c_char) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn opendir(name: *const c_char) -> LibcResult<*mut c_void> {
    todo!()
}

#[interpose]
pub fn readdir(dirp: *mut c_void) -> LibcResult<*mut c_void> {
    todo!()
}

// readdir_r is deprecated on most platforms; included just for completeness
#[interpose]
pub fn readdir_r(
    dirp: *mut c_void,
    entry: *mut c_void,
    result: *mut *mut c_void,
) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn closedir(dirp: *mut c_void) -> LibcResult<c_int> {
    todo!()
}

#[interpose]
pub fn rewinddir(dirp: *mut c_void) -> LibcResult<*mut c_void> {
    todo!()
}

#[interpose]
pub fn telldir(dirp: *mut c_void) -> LibcResult<c_long> {
    todo!()
}

#[interpose]
pub fn seekdir(dirp: *mut c_void, loc: c_long) -> LibcResult<*mut c_void> {
    todo!()
}

#[interpose]
pub fn dirfd(dirp: *mut c_void) -> LibcResult<c_int> {
    todo!()
}
