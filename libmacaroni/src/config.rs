use std::{ffi::{c_char, c_void, CStr}, path::PathBuf, sync::LazyLock};

extern "C" {
    static _mh_execute_header: c_void;

    fn getsegmentdata(mh: *const c_void, segname: *const c_char, size: *mut u64) -> *const c_void;
    fn _dyld_get_image_header(image_index: u32) -> *const c_void;
    fn _dyld_image_count() -> u32;
    fn _dyld_get_image_name(image_index: u32) -> *const c_char;
}

static LIBMACARONI_SYSTEM_PATH: LazyLock<String> = LazyLock::new(|| {
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
                return image_name.to_owned();
            }
        }
    }
    panic!("libmacaroni_system has not been loaded properly");
});

struct RemapPoint {
    from: PathBuf,
    to:   PathBuf,
}

struct RemapConfig {
    points: Vec<RemapPoint>,
}

static REMAP_CONFIG: LazyLock<RemapConfig> = LazyLock::new(|| {
    
});