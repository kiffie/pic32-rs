#!/bin/bash

BIN=usb_serial

cargo build --release || exit
cargo objcopy --release -- -O ihex $BIN.hex
