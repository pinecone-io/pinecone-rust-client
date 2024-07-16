build-openapi:
  ./codegen/build-oas.sh
  cargo build -p openapi

build-proto:
  ./codegen/build-proto.sh
