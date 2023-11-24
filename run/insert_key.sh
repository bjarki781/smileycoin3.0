#!/bin/bash -x

../target/release/substrate-node key insert \
  --chain=smly3 \
  --scheme Sr25519 \
  --suri "$*" \
  --key-type babe

../target/release/substrate-node key insert  \
  --chain=smly3 \
  --scheme Ed25519 \
  --suri "$*" \
  --key-type gran

