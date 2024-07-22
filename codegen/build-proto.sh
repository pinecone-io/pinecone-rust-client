#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
PROJECT_DIR=$(realpath "$SCRIPT_DIR/..")

outdir="src/protos"

pushd $SCRIPT_DIR/apis
	just build
popd

pushd $SCRIPT_DIR/proto_build
	cargo run -- $PROJECT_DIR/$outdir
popd
