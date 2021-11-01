# pic32-config-sector

Calculate constant values for PIC32 configuration words.

This crate defines const structures that can be used to calculate values of
configuration words to be stored in the configuration word section of a Flash
memory image for PIC32 microcontrollers. The `build()` method returns a constant
struct to be output to the configuration word section (typically `.configsfrs`).

Example:

```rust
use pic32_config_sfrs::pic32mx2xx::*;

#[link_section = ".configsfrs"]
#[used]
pub static CONFIGSFRS: ConfigSector = ConfigSector::default()
    .FVBUSONIO(FVBUSONIO::OFF)
    .FUSBIDIO(FUSBIDIO::OFF)
    .IOL1WAY(IOL1WAY::OFF)
    .PMDL1WAY(PMDL1WAY::OFF)
    .FPLLIDIV(FPLLIDIV::DIV_2)
    .FPLLMUL(FPLLMUL::MUL_20)
    .FPLLODIV(FPLLODIV::DIV_2)
    .FNOSC(FNOSC::FRCPLL)
    .FSOSCEN(FSOSCEN::OFF)
    .FPBDIV(FPBDIV::DIV_1)
    .FWDTEN(FWDTEN::OFF)
    .JTAGEN(JTAGEN::OFF)
    .ICESEL(ICESEL::ICS_PGx1)
    .build();
```

To support multiple variants of the PIC32 MCU, multiple modules are provided.
The following modules exist:

| Module | PIC32 variant | used .edc file |
|--------|---------------|----------------|
| pic32mx1xx | PIC32MX1xx | PIC32MX170F256B.PIC |
| pic32mx2xx | PIC32MX2xx | PIC32MX270F256B.PIC |
| pic32mx1x4 | PIC32MX1x4 (XLP) | PIC32MX174F256B.PIC |
| pic32mx2x4 | PIC32MX2x4 (XLP) | PIC32MX274F256B.PIC |
| pic32mzef | PIC32MZEF | PIC32MZ2048EFM144.PIC |

The const builder structs are machine generated from XML files distributed by
Microchip as part of their
[Device Support Packs](https://packs.download.microchip.com) under the
Apache-2 license. The names of the used files are indicated in the above table.
