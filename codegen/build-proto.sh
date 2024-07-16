#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
outdir="protos"

OUT_DIR=$SCRIPT_DIR/../$outdir

pushd $SCRIPT_DIR/apis
	just build
popd

pushd $SCRIPT_DIR/proto_build
	cargo run -- $OUT_DIR
popd
