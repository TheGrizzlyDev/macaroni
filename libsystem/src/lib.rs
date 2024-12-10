use std::{ffi::{c_char, c_void, CStr}, sync::LazyLock};

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

#[no_mangle]
#[link_section = "__DATA,macaroni"]
pub extern "C" fn get_cwd(buf: *mut u8, _size: usize) -> *mut u8 {
    println!("getcwd");
    buf
}

extern "C" {
    static _mh_execute_header: c_void;

    fn getsegmentdata(mh: *const c_void, segname: *const c_char, size: *mut u64) -> *const c_void;
    fn _dyld_get_image_header(image_index: u32) -> *const c_void;
    fn _dyld_image_count() -> u32;
    fn _dyld_get_image_name(image_index: u32) -> *const c_char;
}

struct SegmentInfo {
    start_address: usize,
    end_address: usize,
}

static LIBMACARONI_SYSTEM_INFO: LazyLock<SegmentInfo> = LazyLock::new(|| {
    let image_count = unsafe { _dyld_image_count() };
    for i in 0..image_count {
        let header = unsafe { _dyld_get_image_header(i) };
        if header.is_null() {
            continue;
        }

        let image_name = unsafe { _dyld_get_image_name(i) };
        if !image_name.is_null() {
            let image_name = match unsafe { CStr::from_ptr(image_name) }.to_str() {
                Ok(name) => name,
                Err(_) => continue,
            };

            if !image_name.ends_with("libmacaroni_system.dylib") {
                continue;
            }
        }

        let segment_name = CStr::from_bytes_with_nul(b"__DATA\0").unwrap();
        let mut size: u64 = 0;

        let segment_address = unsafe {
            getsegmentdata(header, segment_name.as_ptr(), &mut size)
        };

        if !segment_address.is_null() {
            let start_address = segment_address as usize;
            return SegmentInfo {
                start_address: start_address,
                end_address: start_address + size as usize,
            };
        }
    }
    panic!("libmacaroni_system has not been loaded properly");
});

fn is_part_of_macaroni_libsystem<Addr: Into<usize>>(address: Addr) -> bool {
    let addr: usize = address.into();
    addr >= LIBMACARONI_SYSTEM_INFO.start_address && addr <= LIBMACARONI_SYSTEM_INFO.end_address
}

#[ctor]
fn init() {
    println!("getcwd addr = {:#x}", (get_cwd as *const ()) as usize);
    if is_part_of_macaroni_libsystem(get_cwd as usize) {
        println!("custom getcwd is part of macaroni");
    } else {
        println!("custom getcwd is not part of macaroni");
    }
    if is_part_of_macaroni_libsystem(libc::getcwd as usize) {
        println!("OS getcwd is part of macaroni");
    } else {
        println!("OS getcwd is not part of macaroni");
    }
    println!("init");
}
