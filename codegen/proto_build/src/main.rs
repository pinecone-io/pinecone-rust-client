use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    let out_dir: &str;
    let version: &str;
    if args.len() == 3 {
        out_dir = &args[1];
        version = &args[2];
        println!("OUT_DIR: {:?}", out_dir);
        println!("version: {:?}", version);
    } else {
        return Err("Required 2 arguments: out_dir version".into());
    }

    let proto_path = format!("../apis/_build/{version}/data_{version}.proto");
    let proto_path: &Path = proto_path.as_ref();

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    // include directory for the google protos
    let include_dir: &Path = "../apis/vendor/protos".as_ref();

    let include_dirs = [proto_dir, include_dir];

    tonic_build::configure()
        .out_dir(out_dir)
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile(&[proto_path], &include_dirs[..])?;

    Ok(())
}
