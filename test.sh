#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

fluence module build ./effector --no-input

LOCAL_IPFS_MULTIADDR=${LOCAL_IPFS_MULTIADDR:="/ip4/0.0.0.0/tcp/5001"}

if ipfs id --api $LOCAL_IPFS_MULTIADDR >/dev/null
then
    echo "IPFS deamon is running. Tests will be run"
else
	echo "IPFS daemon isn't running. Run IPFS daemon and try again. Stopping.."
	exit 1
fi

WASM_LOG=debug cargo nextest run --release --no-fail-fast --nocapture
