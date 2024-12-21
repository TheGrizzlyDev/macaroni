use std::{ffi::{c_char, c_void, CStr}, ptr, sync::LazyLock};

use ctor::ctor;
use libc::backtrace;

#[repr(C)]
struct InterposeEntry {
    replacement: *const (),
    original: *const (),
}

unsafe impl Sync for InterposeEntry {}

macro_rules! interposition_table {
    // Entry point: match one or more pairs of `(replacement, original)` separated by semicolons,
    // optionally allowing a trailing semicolon.
    ($($replacement:path , $original:path);+ $(;)?) => {
        interposition_table!(@gen [$($replacement , $original);+]);
    };

    // Generate the static `INTERPOSED_FUNCTIONS` definition.
    (@gen [$($replacement:path , $original:path);+]) => {
        #[used]
        #[link_section = "__DATA,__interpose"]
        static INTERPOSED_FUNCTIONS: [InterposeEntry; interposition_table!(@count $($replacement , $original),*)] = [
            $(
                InterposeEntry {
                    replacement: $replacement as *const (),
                    original: $original as *const (),
                },
            )*
        ];
    };

    // Count how many `(replacement, original)` pairs there are, by expanding each pair to `()`.
    // The length of that slice is our array size.
    (@count $($replacement:path , $original:path),*) => {
        <[()]>::len(&[$({ let _ = $replacement; let _ = $original; () }),*])
    };
}

interposition_table!(
    get_cwd, libc::getcwd;
);

#[no_mangle]
#[link_section = "__TEXT,__macaroni"]
pub extern "C" fn get_cwd(buf: *mut i8, size: usize) -> *mut i8 {
    if is_caller_from_macaroni_libsystem() {
        println!("self call");

        unsafe {
            return libc::getcwd(buf, size);
        }
    }
    
    println!("external call");
    get_cwd(buf, size)
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

        let segment_name = CStr::from_bytes_with_nul(b"__TEXT\0").unwrap();
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

fn is_address_part_of_macaroni_libsystem<Addr: Into<usize>>(address: Addr) -> bool {
    let addr: usize = address.into();
    addr >= LIBMACARONI_SYSTEM_INFO.start_address && addr <= LIBMACARONI_SYSTEM_INFO.end_address
}

/*
TODO:
this is useful to avoid reimplementing high-level APIs to use our replacements
for libSystem functionalities. At some point though we could skip this backtrace
check and just call the correct underlying functions instead.
*/
fn is_caller_from_macaroni_libsystem() -> bool {
    const INITIAL_FRAMES: usize = 128;
    const EXPANSION_FACTOR: usize = 2;

    let mut max_frames = INITIAL_FRAMES;
    let mut buffer: Vec<*mut libc::c_void> = vec![ptr::null_mut(); max_frames];

    unsafe {
        loop {
            // Capture the backtrace
            let frame_count = backtrace(buffer.as_mut_ptr(), max_frames as libc::c_int);

            // Iterate through the frames to check addresses
            // skip this function and caller function
            for i in 2..frame_count as usize {
                let addr = buffer[i];
                if addr.is_null() {
                    continue;
                }

                if is_address_part_of_macaroni_libsystem(addr as usize) {
                    return true; // Found a match, exit immediately
                }
            }

            // If all frames are processed without a match and the buffer is full, expand it
            if frame_count as usize >= max_frames {
                max_frames *= EXPANSION_FACTOR;
                buffer.resize(max_frames, ptr::null_mut());
            } else {
                break; // No need to expand further; all frames processed
            }
        }
    }

    false // No address matched
}

#[ctor]
fn init() {
    println!("getcwd addr = {:#x}", (get_cwd as *const ()) as usize);
    if is_address_part_of_macaroni_libsystem(get_cwd as usize) {
        println!("custom getcwd is part of macaroni");
    } else {
        println!("custom getcwd is not part of macaroni");
    }
    if is_address_part_of_macaroni_libsystem(libc::getcwd as usize) {
        println!("OS getcwd is part of macaroni");
    } else {
        println!("OS getcwd is not part of macaroni");
    }
    println!("init");
}
