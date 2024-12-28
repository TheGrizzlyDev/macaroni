use std::{
    ptr,
    ffi::{c_char, CStr, CString},
    sync::LazyLock,
};

use crate::config::{self, MountOptions};

struct PathRemapper {
    remap_config: config::Config,
}

impl PathRemapper {
    pub fn new(cfg: config::Config) -> Self {
        PathRemapper { remap_config: cfg }
    }

    pub fn remap(&self, original_path: &str) -> String {
        let mut best_prefix_len = 0usize;
        let mut best_host_path: Option<&str> = None;

        for mount in &self.remap_config.mounts {
            if let config::MountOptions::Remap { host_path } = &mount.options {
                let dest = &mount.destination_path;
                if original_path.starts_with(dest) && dest.len() > best_prefix_len {
                    best_prefix_len = dest.len();
                    best_host_path = Some(host_path.as_str());
                }
            }
        }

        if best_prefix_len > 0 {
            let suffix = &original_path[best_prefix_len..];
            let host = best_host_path.expect("prefix_len>0 implies Some(host_path)");
            format!("{}{}", host, suffix)
        } else {
            original_path.to_owned()
        }
    }

    fn inverse_map(&self, remapped_path: &str) -> Option<String> {
        self.remap_config.mounts
            .iter()
            .filter_map(|mount| {
                let MountOptions::Remap{ host_path } = mount.clone().options;
                if remapped_path.starts_with(&host_path) {
                    return Some(host_path)
                }
                None
            })
            .max_by(|a, b| a.len().cmp(&b.len()))
    }

    pub fn relative_remap(&self, at_path: &str, original_path: &str) -> Option<String> {
        if let Some(original_at_path) = self.inverse_map(at_path) {
            let concat_path = format!("{}{}{}", original_at_path, if original_at_path.ends_with('/') { "" } else { "/" }, original_path);
            return Some(self.remap(&concat_path));
        }
        None
    }
}

static REMAPPER: LazyLock<PathRemapper> =
    LazyLock::new(|| PathRemapper::new(config::LIBMACARONI_CONFIG.clone()));

pub fn remap_c_path(path: *const c_char) -> Option<CString> {
    if path.is_null() {
        return None;
    }
    let path_str = unsafe { CStr::from_ptr(path) }.to_str().expect("TODO");

    let remapped = REMAPPER.remap(path_str);

    Some(CString::new(remapped).expect("TODO"))
}

pub fn relative_remap_c_path(at: *const c_char, path: *const c_char) -> Option<CString> {
    if path.is_null() {
        return None;
    }
    
    let at_str = unsafe { CStr::from_ptr(at) }.to_str().expect("TODO");
    let path_str = unsafe { CStr::from_ptr(path) }.to_str().expect("TODO");

    if let Some(remapped) = REMAPPER.relative_remap(at_str, path_str) {
        return Some(CString::new(remapped).expect("TODO"));
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::{Config, MountPoint, MountOptions};

    #[test]
    fn test_no_mounts() {
        let empty_config = Config { mounts: vec![] };
        let remapper = PathRemapper::new(empty_config);
        let original = "/foo/bar";
        assert_eq!(remapper.remap(original), "/foo/bar");
    }

    #[test]
    fn test_single_remap_simple_prefix() {
        let c = Config {
            mounts: vec![
                MountPoint {
                    destination_path: "/foo".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/Volumes/Stuff/foo".to_string(),
                    }
                }
            ]
        };
        let remapper = PathRemapper::new(c);

        let original = "/foo/bar";
        let expected = "/Volumes/Stuff/foo/bar";
        assert_eq!(remapper.remap(original), expected);

        let other = "/baz/foo/bar";
        assert_eq!(remapper.remap(other), "/baz/foo/bar");
    }

    #[test]
    fn test_partial_matches() {
        let c = Config {
            mounts: vec![
                MountPoint {
                    destination_path: "/home/user/projects".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/mnt/userdata/projects".to_string(),
                    }
                }
            ]
        };
        let remapper = PathRemapper::new(c);

        let p1 = "/home/user/projects/myapp";
        let expected1 = "/mnt/userdata/projects/myapp";
        assert_eq!(remapper.remap(p1), expected1);

        let p2 = "/home/user/projectx/data";
        assert_eq!(remapper.remap(p2), p2);
    }

    #[test]
    fn test_config_with_multiple_mounts() {
        let c = Config {
            mounts: vec![
                MountPoint {
                    destination_path: "/foo".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/stuff/foo".to_string(),
                    },
                },
                MountPoint {
                    destination_path: "/bar".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/stuff/bar".to_string(),
                    },
                },
            ]
        };
        let remapper = PathRemapper::new(c);

        assert_eq!(remapper.remap("/foo"), "/stuff/foo");

        assert_eq!(remapper.remap("/foo/sub"), "/stuff/foo/sub");

        assert_eq!(remapper.remap("/bar"), "/stuff/bar");

        assert_eq!(remapper.remap("/bar/sub"), "/stuff/bar/sub");

        assert_eq!(remapper.remap("/other/path"), "/other/path");
    }

    #[test]
    fn test_edge_cases_empty_path() {
        let c = Config {
            mounts: vec![
                MountPoint {
                    destination_path: "".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/empty".to_string(),
                    }
                }
            ]
        };
        let remapper = PathRemapper::new(c);

        assert_eq!(remapper.remap(""), "/empty"); // suffix is "", so /empty
        assert_eq!(remapper.remap("/anything"), "/empty/anything");
    }


    #[test]
    fn test_longest_prefix_match() {
        let cfg = Config {
            mounts: vec![
                MountPoint {
                    destination_path: "/foo".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/X".to_string(),
                    }
                },
                MountPoint {
                    destination_path: "/foo/bar".to_string(),
                    options: MountOptions::Remap {
                        host_path: "/Y".to_string(),
                    }
                },
            ],
        };

        let remapper = PathRemapper::new(cfg);

        let result1 = remapper.remap("/foo/bar/baz");
        assert_eq!(result1, "/Y/baz", "Should match the longer prefix /foo/bar");

        let result2 = remapper.remap("/foo/subdir");
        assert_eq!(result2, "/X/subdir", "Only /foo matches here");
    }
}
