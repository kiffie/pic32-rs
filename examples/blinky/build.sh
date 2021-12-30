#!/bin/bash

BIN=blinky

cargo objcopy --release $* -- -O ihex $BIN.hex
