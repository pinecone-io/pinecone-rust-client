fn main() {
    protobuf_codegen::Codegen::new()
        // optional
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // all inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["../codegen/apis"])
        .includes(&["./src"])
        // inputs must reside in some of include paths.
        .input("../codegen/apis/_build/2024-07/data_2024-07.proto")
        .input("./src/google/api/annotations.proto")
        .input("./src/google/api/http.proto")
        .input("./src/google/api/field_behavior.proto")
        // specify output directory
        .out_dir("./src/data")
        .run_from_script();
}
