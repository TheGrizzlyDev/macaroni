use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum MountOptions {
    Remap { host_path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MountPoint {
    destination_path: String,

    #[serde(flatten)]
    options: MountOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    mounts: Vec<MountPoint>,
}
