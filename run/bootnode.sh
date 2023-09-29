#!/bin/bash

pushd ../substrate/bin/node/cli
cargo build -r --bin substrate-node
popd

../target/release/substrate-node --base-path ~/blockchain --chain=smly3  --validator --name "bn-`hostname -s`" --unsafe-rpc-external --rpc-cors=all $@

