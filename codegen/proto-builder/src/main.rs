use std::fmt::format;

fn main() {
    let current_dir = std::env::current_dir().unwrap();
    let workspace = format!("{}/../..", current_dir.display());

    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&[format!("{}/codegen/apis/_build/2024-07", workspace)])
        .includes(&[format!("{}/codegen/proto-builder/imports", workspace)])
        // Inputs must reside in some of include paths.
        .input(format!(
            "{}/codegen/apis/_build/2024-07/data_2024-07.proto",
            workspace
        ))
        .out_dir(format!("{}/protos", workspace))
        // Specify output directory relative to Cargo output directory.
        // .cargo_out_dir("protos")
        .run_from_script();
}
