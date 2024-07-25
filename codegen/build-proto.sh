#!/bin/bash

version=$1

SCRIPT_DIR=$(dirname "$(realpath "$0")")
outdir="protos"

if [ -z "$version" ]; then
	echo "Version is required"
	exit 1
fi

OUT_DIR=$SCRIPT_DIR/../$outdir

pushd $SCRIPT_DIR/apis
	just build
popd

pushd $SCRIPT_DIR/proto_build
	cargo run -- $OUT_DIR $version
popd
