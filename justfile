api_version := "2024-07"

# Lint files and run tests with optional specific test cases
test *tests: lint
  # Runs the specified tests, or all tests if none specified
  cargo test {{tests}} --verbose

# Update git submodules recursively
update:
  git submodule update --init --recursive

# Run linting tools (cargo fmt and clippy)
lint:
  cargo fmt && cargo clippy

# Run cargo build
build:
  cargo build

# Build the OpenAPI and Protobuf definitions in the `codegen/apis` submodule
gen-build-submodule-apis:
  cd codegen/apis && just build

# Generate the control plane OpenAPI code based on the yaml files in `codegen/apis/_build`
gen-openapi: gen-build-submodule-apis gen-version_file
  ./codegen/build-oas.sh {{api_version}}

# Generate the data plane protobuf code based on the yaml files in `codegen/apis/_build`
gen-proto: gen-build-submodule-apis gen-version_file
  ./codegen/build-proto.sh {{api_version}}

# Generate all OpenAPI and protobuf code
gen-client: gen-build-submodule-apis gen-version_file
  ./codegen/build-oas.sh {{api_version}}
  ./codegen/build-proto.sh {{api_version}}

# Generate version file
gen-version_file:
  echo "/// Pinecone API version\npub const API_VERSION: &str = \"{{api_version}}\";" > src/version.rs