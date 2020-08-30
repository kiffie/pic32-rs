#!/bin/bash

BIN=usb_serial

cargo objcopy --release -- -O ihex $BIN.hex
