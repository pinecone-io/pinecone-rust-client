build-openapi:
  ./codegen/build-oas.sh
  cargo build -p openapi

build-protos:
  cd protos && cargo build
