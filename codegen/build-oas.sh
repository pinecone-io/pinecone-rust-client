#!/bin/bash

pushd codegen/apis
	just build
popd

outdir="openapi"

docker run --rm -v $(pwd):/workspace openapitools/openapi-generator-cli:v7.6.0 generate \
	--input-spec /workspace/codegen/apis/_build/2024-07/control_2024-07.oas.yaml \
	--generator-name rust \
	--output /workspace/$outdir \
	--additional-properties "packageVersion=0.0.1"
