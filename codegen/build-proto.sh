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

missing_docs_header=$'#![allow(missing_docs)]'
dead_code_header=$'#![allow(dead_code)]'
mod_header=$"$missing_docs_header\n$dead_code_header\n"
pushd $PROJECT_DIR/$outdir
	# rename _.rs to mod.rs
	mv _.rs mod.rs
	# add line at the top to disable warnings for undocumented and unused code
	echo -e $mod_header | cat - mod.rs > temp && mv temp mod.rs
popd
