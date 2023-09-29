#!/bin/bash

pushd ../substrate/bin/node/cli
cargo build -r --bin substrate-node
popd

../target/release/substrate-node --base-path ~/blockchain --chain=smly3 --validator --name "`hostname -s`" --bootnodes=/dns/twserver.rhi.hi.is/tcp/30333/p2p/$@


