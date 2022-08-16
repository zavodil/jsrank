#!/usr/bin/env bash

# Exit script as soon as a command fails.
set -e

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

DOCKER_NAME="build_contract"
FOLDER="js-hackerrank"

if docker ps -a --format '{{.Names}}' | grep -Eq "^${DOCKER_NAME}\$"; then
    echo "Container exists"
else
docker create \
     --mount type=bind,source=$DIR/..,target=/host \
     --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
     --name=$DOCKER_NAME \
     -w /host/$FOLDER \
     -e RUSTFLAGS='-C link-arg=-s' \
     -it \
     contract-builder \
     /bin/bash
fi

docker start $DOCKER_NAME
docker exec -it $DOCKER_NAME /bin/bash -c "rustup toolchain install nightly-2022-06-20; rustup default nightly-2022-06-20; rustup target add wasm32-unknown-unknown; cargo build --target wasm32-unknown-unknown --manifest-path ./../$FOLDER/Cargo.toml --release"

mkdir -p res
cp $DIR/target/wasm32-unknown-unknown/release/contract.wasm $DIR/res/release.wasm