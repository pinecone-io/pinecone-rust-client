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

mod_header=$'\n#![allow(missing_docs)]\n'
pushd $PROJECT_DIR/$outdir
	# rename _.rs to mod.rs
	mv _.rs mod.rs
	# add line at the top to disable warnings for undocumented code
	sed -i "" "1 i\\$mod_header" mod.rs
popd
