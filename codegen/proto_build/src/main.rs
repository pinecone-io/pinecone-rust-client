use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_path: &Path = "../apis/_build/2024-07/data_2024-07.proto".as_ref();

    // print OUT_DIR env
    println!("OUT_DIR: {:?}", std::env::var("OUT_DIR").unwrap());

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    // include directory for the google protos
    let include_dir: &Path = "../apis/vendor/protos".as_ref();

    let include_dirs = [proto_dir, include_dir];

    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&[proto_path], &include_dirs[..])?;

    Ok(())
}
