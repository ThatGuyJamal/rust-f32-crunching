#!/usr/bin/env bash

cargo build --release
cd ./target/release || exit
./treads