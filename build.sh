#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

EFFECTOR_NAME=ipfs_effector

# This script builds all subprojects and puts all created Wasm modules in one dir
echo "Building effector module..."
fluence module build ./effector --no-input

# We evalutate the CID here and not in build.rs because the second option required to put the resulting wasm file into the crate which
# needlessly increased the size of the crate.
echo "Evaluating CID to build the cid crate..."
mkdir -p cid/artifacts/
ipfs add -Q --only-hash --cid-version 1 --hash sha2-256 --chunker=size-262144 "target/wasm32-wasi/release/$EFFECTOR_NAME.wasm" > cid/artifacts/cidv1
echo "Resulting CID is $(cat cid/artifacts/cidv1)"

echo "Building the library crates.."
cargo build --release
