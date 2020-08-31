# pic32-rs-oled-demo
Shows a moving Rust logo on an OLED display

Before the logo is shown text strings in different font sizes are displayed for
a few seconds.

This example also demonstrates some logging via the UART (port RPA0 at pin2 on
28 pin devices, 115200 bits/s).

The program can be build with cargo; see also the script `build.sh`.
When switching to a different target MCU, e.g. the 32MX274F256B, a different
linker script must be used and `.cargo/config` needs to be adapted.

![Pic 32 OLED](https://raw.githubusercontent.com/kiffie/pic32-rs/master/examples/i2c_oled_display/doc/pic32-oled.jpg)
