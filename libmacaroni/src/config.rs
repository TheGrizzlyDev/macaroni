use std::{env, ffi::{c_char, c_void, CStr}, path::PathBuf, sync::LazyLock};

extern "C" {
    fn _dyld_get_image_header(image_index: u32) -> *const c_void;
    fn _dyld_image_count() -> u32;
    fn _dyld_get_image_name(image_index: u32) -> *const c_char;
}

pub static LIBMACARONI_SYSTEM_PATH: LazyLock<String> = LazyLock::new(|| {
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


#[derive(Clone, Debug)]
pub(crate) struct RemapPoint {
    from: String,
    to:   String,
}

#[derive(Clone, Debug)]
pub(crate) struct RemapConfig {
    points: Vec<RemapPoint>,
}

pub static REMAP_CONFIG: LazyLock<RemapConfig> = LazyLock::new(|| {
    let remap_config_str = env::var("MACARONI_REMAP_CONFIG").expect("You must set the env variable MACARONI_REMAP_CONFIG");
    let points: Vec<RemapPoint> = remap_config_str
        .split(';')
        .into_iter()
        .map(|map_point_str| -> RemapPoint {
            let (from, to) = map_point_str
                .split_once('=')
                .expect("TODO: write a useful error message");
            RemapPoint { from: from.to_owned(), to: to.to_owned() }
        })
        .collect();

    RemapConfig { points }
});