build-openapi:
  ./codegen/build-oas.sh
  cargo build -p openapi

build-protos:
  ./codegen/build-protos.sh
  