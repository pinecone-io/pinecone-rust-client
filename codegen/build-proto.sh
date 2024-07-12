#!/bin/bash

# pushd codegen/apis
# 	just build
# popd

outdir="protos"
rm -rf $outdir
mkdir $outdir

# export OUT_DIR as pwd/../outdir with the parent!!!
export OUT_DIR=$(pwd)/$outdir

pushd codegen/proto_build
	cargo run
popd
