#!/usr/bin/env bash

cargo build --release
cd ./target/release || exit
./treads "$@" # pass all arguments to the binary
