#!/bin/bash

pushd codegen/apis
	just build
popd

outdir="protos"
rm -rf $outdir
mkdir $outdir

proto_path=$(pwd)/codegen/apis/_build/2024-07
import_path=$(pwd)/codegen/proto-imports

# generate files
pushd codegen/proto-builder
	cargo run
popd
