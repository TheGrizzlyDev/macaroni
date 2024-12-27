use std::{env, ffi::{c_char, c_void, CStr}, fs, path::PathBuf, sync::LazyLock};
use serde_derive::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MountOptions {
    Remap { host_path: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountPoint {
    pub destination_path: String,
    #[serde(flatten)]
    pub options: MountOptions,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mounts: Vec<MountPoint>,
}

pub static LIBMACARONI_CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let sandbox_path = match env::var("MACARONI_SANDBOX_PATH") {
        Ok(path_str) => {
            PathBuf::from(path_str)
        }
        Err(e) => {
            panic!("MACARONI_SANDBOX_PATH not set or invalid: {}", e)
        }
    };

    let mut config_path = sandbox_path.clone();
    config_path.push("config.json");

    let config_raw = fs::read_to_string(config_path).unwrap();
    let config: Config = serde_json::from_str(&config_raw).unwrap();

    config
});