#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

if [[ ! -e cid/artifacts/cidv1 ]]
then
	echo "You need to run ./build.sh first"
	exit 1
fi

WASM_LOG=debug cargo nextest run --release --no-fail-fast --nocapture
