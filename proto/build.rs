use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(&out_dir)
        .file_descriptor_set_path(out_dir.join("management_descriptor.bin"))
        .compile_protos(
            &["proto/management.proto", "proto/sandbox.proto"],
            &["proto"],
        )?;

    println!("cargo:rerun-if-changed=proto/management.proto");
    println!("cargo:rerun-if-changed=proto/sandbox.proto");
    println!("cargo:rerun-if-changed=proto");
    
    Ok(())
}