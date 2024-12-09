use ctor::ctor;

#[repr(C)]
struct InterposeEntry {
    replacement: *const (),
    original: *const (),
}

unsafe impl Sync for InterposeEntry {}

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_GETCWD: InterposeEntry = InterposeEntry {
    replacement: get_cwd as *const (),
    original: libc::getcwd as *const (),
};

extern "C" fn get_cwd(buf: *mut u8, _size: usize) -> *mut u8 {
    println!("getcwd");
    buf
}

#[ctor]
fn init() {
    println!("init");
}