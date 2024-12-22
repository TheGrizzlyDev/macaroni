use libc_interposition_macro::interpose;
use libc_interposition_lib::LibcResult;

#[interpose]
pub fn getcwd(buf: *mut i8, size: usize) -> LibcResult<*mut i8> {
    // TODO handle errors
    unsafe {
        return LibcResult::Ok(libc::getcwd(buf, size));
    }
}