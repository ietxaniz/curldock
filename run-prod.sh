#!/bin/bash

export MODE=PROD
export SCRIPTSFOLDER=rest-examples
export PORT=2080

cargo run --release
