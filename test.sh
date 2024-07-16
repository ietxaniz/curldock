#!/bin/bash
rm -rf ./test-assets/tmp
mkdir ./test-assets/tmp
export MODE=PROD
export DEVFRONTPORT=2081
export SCRIPTSFOLDER=test-assets
export PORT=2080
nohup ./target/debug/curldock > ./test-assets/tmp/backend.log &
sleep 1
RUSTFLAGS="-A warnings" cargo test -- --quiet --show-output
fuser -k 2080/tcp
