use libc_interposition_macro::interpose;
use libc_interposition_lib::{LibcResult, InterposeEntry};

#[interpose]
pub fn getcwd(buf: *mut i8, size: usize) -> LibcResult<*mut i8> {
    println!("Interposed!!!");
    unsafe {
        return LibcResult::Ok(libc::getcwd(buf, size));
    }
}

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_TABLE: [InterposeEntry; 1] = [
    getcwd::INTERPOSE_ENTRY,
];