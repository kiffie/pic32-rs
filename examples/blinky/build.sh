#!/bin/bash

BIN=blinky

cargo build --release || exit
cargo objcopy --release $* -- -O ihex $BIN.hex
