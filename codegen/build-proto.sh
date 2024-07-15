#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
outdir="protos"

pushd codegen/apis
	just build
popd

export OUT_DIR=$SCRIPT_DIR/../$outdir

pushd codegen/proto_build
	cargo run
popd
