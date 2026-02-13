use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile all `.proto` dependencies using `buf`
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let export_dir = out_dir.join("proto_export");

    let status = Command::new("buf")
        .arg("export")
        .arg("proto")
        .arg("--output")
        .arg(&export_dir)
        .status()?;

    if !status.success() {
        panic!("Failed to run `buf export`. Ensure buf is installed and path is correct.");
    }

    tonic_prost_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "proto/google/pprof/profile.proto",
                "proto/parca/debuginfo/v1alpha1/debuginfo.proto",
                "proto/parca/profilestore/v1alpha1/profilestore.proto",
            ], // The files to compile
            &["proto/", export_dir.to_str().unwrap()], // The include path (where to search for imports)
        )?;

    // Rerun if buf.yaml or the proto file changes
    println!("cargo:rerun-if-changed=proto/buf.yaml");
    println!("cargo:rerun-if-changed=proto/buf.lock");
    println!("cargo:rerun-if-changed=proto/google/pprof/profile.proto");
    println!("cargo:rerun-if-changed=proto/parca/debuginfo/v1alpha1/debuginfo.proto");
    println!("cargo:rerun-if-changed=proto/parca/profilestore/v1alpha1/profilestore.proto");

    Ok(())
}
