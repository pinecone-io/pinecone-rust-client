#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
PROJECT_DIR=$(realpath "$SCRIPT_DIR/..")

outdir="src/protos"

OUT_DIR=$SCRIPT_DIR/../$outdir

version=$1

if [ -z "$version" ]; then
	echo "Version is required"
	exit 1
fi

pushd $SCRIPT_DIR/proto_build
	cargo run -- $PROJECT_DIR/$outdir $version
popd

pushd $PROJECT_DIR/$outdir
	# rename _.rs to mod.rs
	mv _.rs mod.rs
popd
