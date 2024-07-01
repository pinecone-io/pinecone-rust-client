use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path: &Path = "../codegen/apis/_build/2024-07/data_2024-07.proto".as_ref();

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    let include_dirs = [proto_dir, "imports".as_ref()];

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&[proto_path], &include_dirs[..])?;

    Ok(())
}