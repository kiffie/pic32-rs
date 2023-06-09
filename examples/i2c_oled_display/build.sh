#!/bin/bash

BIN=i2c_oled_display

cargo build --release || exit
cargo objcopy --release -- -O ihex $BIN.hex
