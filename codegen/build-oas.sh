#!/bin/bash

SCRIPT_DIR=$(dirname "$(realpath "$0")")
PROJECT_DIR=$(realpath "$SCRIPT_DIR/..")

pushd $PROJECT_DIR/codegen/apis
	just build
popd

tempdir=".openapi-crate"
outdir="src/openapi"

docker run --rm -v $(pwd):/workspace openapitools/openapi-generator-cli:v7.6.0 generate \
	--input-spec /workspace/codegen/apis/_build/2024-07/control_2024-07.oas.yaml \
	--generator-name rust \
	--output /workspace/$tempdir \
	--additional-properties "packageVersion=0.0.1"

# copy source files from the crate to the module (outdir)
echo "Copying source files from $tempdir to $outdir"
pushd $PROJECT_DIR
	# copy the readme to the outdir
	cp $tempdir/README.md $outdir/README.md

	# copy everything in src to the outdir
	cp -r $tempdir/src/* $outdir

	# rename the lib.rs file to mod.rs
	mv $outdir/lib.rs $outdir/mod.rs

	# in each source file, replace "crate::" with "crate::openapi::"
	for f in $(find $outdir -type f); do
		sed -i "" 's/crate::/crate::openapi::/g' $f
	done
popd

echo "All done!"
