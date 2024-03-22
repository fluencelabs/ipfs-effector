#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

EFFECTOR_NAME="ipfs_effector"

if [[ ! -e cid/artifacts/cidv1 ]]
then
	echo "You need to run ./build.sh first"
	exit 1
fi

echo "Packaging the effector"
# Pack the module
fluence module pack ./effector/ --binding-crate=./imports/ --no-input -d .

echo "Extracting the CID from the package and the crate"
cid_package="$(tar -axf $EFFECTOR_NAME.tar.gz module.yaml -O | grep cid | cut -d' ' -f2)"
cid_crate="$(cut -d' ' -f2 < cid/artifacts/cidv1)"

if [[ "$cid_package" = "$cid_crate" ]]
then
   echo "Validated"
else
   echo "Not validated" >&2
   echo "Fluence Package CID: '${cid_package}'" >&2
   echo "Rust Crate CID: '${cid_crate}'" >&2
   exit 1
fi
