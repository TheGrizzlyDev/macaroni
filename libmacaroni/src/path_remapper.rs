use std::{ffi::{c_char, CStr}, sync::LazyLock};

use crate::config;

struct PathRemapper {
    remap_config: config::RemapConfig,
}

impl PathRemapper {
    
}

const REMAPPER: LazyLock<PathRemapper> = LazyLock::new(|| { PathRemapper{ remap_config: (*config::REMAP_CONFIG).clone() } });

pub fn remap_c_path(path: *const c_char) -> *const c_char {
    let patth = unsafe { CStr::from_ptr(path) }.to_str().to_owned();
    todo!()
}