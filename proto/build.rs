fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile_protos(
            &["proto/management.proto", "proto/sandbox.proto"],
            &["proto"],
        )?;

    
    println!("cargo:rerun-if-changed=proto/management.proto");
    println!("cargo:rerun-if-changed=proto/sandbox.proto");
    println!("cargo:rerun-if-changed=proto");
    
    Ok(())
}