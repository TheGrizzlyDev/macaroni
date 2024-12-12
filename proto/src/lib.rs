pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("management_descriptor");
pub mod management {
    tonic::include_proto!("management");
}
pub mod sandbox {
    tonic::include_proto!("sandbox");
}