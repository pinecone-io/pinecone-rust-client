api_version := "2024-07"

# Build the OpenAPI and Protobuf definitions in `codegen/apis`
build-apis:
  cd codegen/apis && just build

# Generate the control plane OpenAPI code based on the yaml files in `codegen/apis/_build`
build-openapi:
  ./codegen/build-oas.sh {{api_version}}

# Generate the data plane protobuf code based on the yaml files in `codegen/apis/_build`
build-proto:
  ./codegen/build-proto.sh {{api_version}}

# Generate all OpenAPI and protobuf code
build-client: build-apis build-openapi build-proto
  echo "/// Pinecone API version\npub const API_VERSION: &str = \"{{api_version}}\";" > pinecone_sdk/src/version.rs
