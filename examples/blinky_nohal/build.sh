#!/bin/bash

BIN=blinky_nohal

cargo objcopy --release $* -- -O ihex $BIN.hex
