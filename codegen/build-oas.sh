#!/bin/bash

version=$1

if [ -z "$version" ]; then
	echo "Version is required"
	exit 1
fi

pushd codegen/apis
	just build
popd

outdir="openapi"

docker run --rm -v $(pwd):/workspace openapitools/openapi-generator-cli:v7.6.0 generate \
	--input-spec /workspace/codegen/apis/_build/$version/control_$version.oas.yaml \
	--generator-name rust \
	--output /workspace/$outdir \
	--additional-properties "packageVersion=0.0.1"
