#!/bin/sh

set -eux

curl https://sh.rustup.rs -sSf | sh -s -- -y
source $HOME/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cargo build
wasm-pack build
cd www
npm i
npm run build
