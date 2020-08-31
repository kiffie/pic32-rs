#!/bin/bash

BIN=i2c_oled_display

cargo objcopy --release -- -O ihex $BIN.hex
