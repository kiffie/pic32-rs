#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
/// Length of config word sector in words
pub const CONFIG_SECTOR_LENGTH: usize = 4;

type USERID = u16;

/// Peripheral Module Disable Configuration
pub enum PMDL1WAY {
    /// Allow only one reconfiguration
    ON = 0x1,

    /// Allow multiple reconfigurations
    OFF = 0x0,
}

/// Peripheral Pin Select Configuration
pub enum IOL1WAY {
    /// Allow only one reconfiguration
    ON = 0x1,

    /// Allow multiple reconfigurations
    OFF = 0x0,
}

/// USB USID Selection
pub enum FUSBIDIO {
    /// Controlled by Port Function
    OFF = 0x0,

    /// Controlled by the USB Module
    ON = 0x1,
}

/// USB VBUS ON Selection
pub enum FVBUSONIO {
    /// Controlled by Port Function
    OFF = 0x0,

    /// Controlled by USB Module
    ON = 0x1,
}

/// PLL Input Divider
pub enum FPLLIDIV {
    /// 1x Divider
    DIV_1 = 0x0,

    /// 2x Divider
    DIV_2 = 0x1,

    /// 3x Divider
    DIV_3 = 0x2,

    /// 4x Divider
    DIV_4 = 0x3,

    /// 5x Divider
    DIV_5 = 0x4,

    /// 6x Divider
    DIV_6 = 0x5,

    /// 10x Divider
    DIV_10 = 0x6,

    /// 12x Divider
    DIV_12 = 0x7,
}

/// PLL Multiplier
pub enum FPLLMUL {
    /// 15x Multiplier
    MUL_15 = 0x0,

    /// 16x Multiplier
    MUL_16 = 0x1,

    /// 17x Multiplier
    MUL_17 = 0x2,

    /// 18x Multiplier
    MUL_18 = 0x3,

    /// 19x Multiplier
    MUL_19 = 0x4,

    /// 20x Multiplier
    MUL_20 = 0x5,

    /// 21x Multiplier
    MUL_21 = 0x6,

    /// 24x Multiplier
    MUL_24 = 0x7,
}

/// USB PLL Input Divider
pub enum UPLLIDIV {
    /// 1x Divider
    DIV_1 = 0x0,

    /// 2x Divider
    DIV_2 = 0x1,

    /// 3x Divider
    DIV_3 = 0x2,

    /// 4x Divider
    DIV_4 = 0x3,

    /// 5x Divider
    DIV_5 = 0x4,

    /// 6x Divider
    DIV_6 = 0x5,

    /// 10x Divider
    DIV_10 = 0x6,

    /// 12x Divider
    DIV_12 = 0x7,
}

/// USB PLL Enable
pub enum UPLLEN {
    /// Enabled
    ON = 0x0,

    /// Disabled and Bypassed
    OFF = 0x1,
}

/// System PLL Output Clock Divider
pub enum FPLLODIV {
    /// PLL Divide by 1
    DIV_1 = 0x0,

    /// PLL Divide by 2
    DIV_2 = 0x1,

    /// PLL Divide by 4
    DIV_4 = 0x2,

    /// PLL Divide by 8
    DIV_8 = 0x3,

    /// PLL Divide by 16
    DIV_16 = 0x4,

    /// PLL Divide by 32
    DIV_32 = 0x5,

    /// PLL Divide by 64
    DIV_64 = 0x6,

    /// PLL Divide by 256
    DIV_256 = 0x7,
}

/// Oscillator Selection Bits
pub enum FNOSC {
    /// Fast RC Osc (FRC)
    FRC = 0x0,

    /// Fast RC Osc with PLL
    FRCPLL = 0x1,

    /// Primary Osc (XT,HS,EC)
    PRI = 0x2,

    /// Primary Osc w/PLL (XT+,HS+,EC+PLL)
    PRIPLL = 0x3,

    /// Low Power Secondary Osc (SOSC)
    SOSC = 0x4,

    /// Low Power RC Osc (LPRC)
    LPRC = 0x5,

    /// Fast RC Osc w/Div-by-16 (FRC/16)
    FRCDIV16 = 0x6,

    /// Fast RC Osc w/Div-by-N (FRCDIV)
    FRCDIV = 0x7,
}

/// Secondary Oscillator Enable
pub enum FSOSCEN {
    /// Disabled
    OFF = 0x0,

    /// Enabled
    ON = 0x1,
}

/// Internal/External Switch Over
pub enum IESO {
    /// Disabled
    OFF = 0x0,

    /// Enabled
    ON = 0x1,
}

/// Primary Oscillator Configuration
pub enum POSCMOD {
    /// External clock mode
    EC = 0x0,

    /// XT osc mode
    XT = 0x1,

    /// HS osc mode
    HS = 0x2,

    /// Primary osc disabled
    OFF = 0x3,
}

/// CLKO Output Signal Active on the OSCO Pin
pub enum OSCIOFNC {
    /// Disabled
    OFF = 0x1,

    /// Enabled
    ON = 0x0,
}

/// Peripheral Clock Divisor
pub enum FPBDIV {
    /// Pb_Clk is Sys_Clk/1
    DIV_1 = 0x0,

    /// Pb_Clk is Sys_Clk/2
    DIV_2 = 0x1,

    /// Pb_Clk is Sys_Clk/4
    DIV_4 = 0x2,

    /// Pb_Clk is Sys_Clk/8
    DIV_8 = 0x3,
}

/// Clock Switching and Monitor Selection
pub enum FCKSM {
    /// Clock Switch Enable, FSCM Enabled
    CSECME = 0x0,

    /// Clock Switch Enable, FSCM Disabled
    CSECMD = 0x1,

    /// Clock Switch Disable, FSCM Disabled
    CSDCMD = 0x3,
}

/// Watchdog Timer Postscaler
pub enum WDTPS {
    /// 1:1
    PS1 = 0x0,

    /// 1:2
    PS2 = 0x1,

    /// 1:4
    PS4 = 0x2,

    /// 1:8
    PS8 = 0x3,

    /// 1:16
    PS16 = 0x4,

    /// 1:32
    PS32 = 0x5,

    /// 1:64
    PS64 = 0x6,

    /// 1:128
    PS128 = 0x7,

    /// 1:256
    PS256 = 0x8,

    /// 1:512
    PS512 = 0x9,

    /// 1:1024
    PS1024 = 0xa,

    /// 1:2048
    PS2048 = 0xb,

    /// 1:4096
    PS4096 = 0xc,

    /// 1:8192
    PS8192 = 0xd,

    /// 1:16384
    PS16384 = 0xe,

    /// 1:32768
    PS32768 = 0xf,

    /// 1:65536
    PS65536 = 0x10,

    /// 1:131072
    PS131072 = 0x11,

    /// 1:262144
    PS262144 = 0x12,

    /// 1:524288
    PS524288 = 0x13,

    /// 1:1048576
    PS1048576 = 0x14,
}

/// Watchdog Timer Window Enable
pub enum WINDIS {
    /// Watchdog Timer is in Window Mode
    ON = 0x0,

    /// Watchdog Timer is in Non-Window Mode
    OFF = 0x1,
}

/// Watchdog Timer Enable
pub enum FWDTEN {
    /// WDT Disabled (SWDTEN Bit Controls)
    OFF = 0x0,

    /// WDT Enabled
    ON = 0x1,
}

/// Watchdog Timer Window Size
pub enum FWDTWINSZ {
    /// Window Size is 75%
    WINSZ_75 = 0x0,

    /// Window Size is 50%
    WINSZ_50 = 0x1,

    /// Window Size is 37.5%
    WINSZ_37 = 0x2,

    /// Window Size is 25%
    WINSZ_25 = 0x3,
}

/// Background Debugger Enable
pub enum DEBUG {
    /// Debugger is Enabled
    ON = 0x0,

    /// Debugger is Disabled
    OFF = 0x3,
}

/// JTAG Enable
pub enum JTAGEN {
    /// JTAG Port Enabled
    ON = 0x1,

    /// JTAG Disabled
    OFF = 0x0,
}

/// ICE/ICD Comm Channel Select
pub enum ICESEL {
    /// Communicate on PGEC1/PGED1
    ICS_PGx1 = 0x3,

    /// Communicate on PGEC2/PGED2
    ICS_PGx2 = 0x2,

    /// Communicate on PGEC3/PGED3
    ICS_PGx3 = 0x1,

    /// Reserved
    RESERVED = 0x0,
}

/// Program Flash Write Protect
pub enum PWP {
    /// Disable
    OFF = 0x1ff,

    /// First 1K
    PWP1K = 0x1fe,

    /// First 2K
    PWP2K = 0x1fd,

    /// First 3K
    PWP3K = 0x1fc,

    /// First 4K
    PWP4K = 0x1fb,

    /// First 5K
    PWP5K = 0x1fa,

    /// First 6K
    PWP6K = 0x1f9,

    /// First 7K
    PWP7K = 0x1f8,

    /// First 8K
    PWP8K = 0x1f7,

    /// First 9K
    PWP9K = 0x1f6,

    /// First 10K
    PWP10K = 0x1f5,

    /// First 11K
    PWP11K = 0x1f4,

    /// First 12K
    PWP12K = 0x1f3,

    /// First 13K
    PWP13K = 0x1f2,

    /// First 14K
    PWP14K = 0x1f1,

    /// First 15K
    PWP15K = 0x1f0,

    /// First 16K
    PWP16K = 0x1ef,

    /// First 17K
    PWP17K = 0x1ee,

    /// First 18K
    PWP18K = 0x1ed,

    /// First 19K
    PWP19K = 0x1ec,

    /// First 20K
    PWP20K = 0x1eb,

    /// First 21K
    PWP21K = 0x1ea,

    /// First 22K
    PWP22K = 0x1e9,

    /// First 23K
    PWP23K = 0x1e8,

    /// First 24K
    PWP24K = 0x1e7,

    /// First 25K
    PWP25K = 0x1e6,

    /// First 26K
    PWP26K = 0x1e5,

    /// First 27K
    PWP27K = 0x1e4,

    /// First 28K
    PWP28K = 0x1e3,

    /// First 29K
    PWP29K = 0x1e2,

    /// First 30K
    PWP30K = 0x1e1,

    /// First 31K
    PWP31K = 0x1e0,

    /// First 32K
    PWP32K = 0x1df,

    /// First 33K
    PWP33K = 0x1de,

    /// First 34K
    PWP34K = 0x1dd,

    /// First 35K
    PWP35K = 0x1dc,

    /// First 36K
    PWP36K = 0x1db,

    /// First 37K
    PWP37K = 0x1da,

    /// First 38K
    PWP38K = 0x1d9,

    /// First 39K
    PWP39K = 0x1d8,

    /// First 40K
    PWP40K = 0x1d7,

    /// First 41K
    PWP41K = 0x1d6,

    /// First 42K
    PWP42K = 0x1d5,

    /// First 43K
    PWP43K = 0x1d4,

    /// First 44K
    PWP44K = 0x1d3,

    /// First 45K
    PWP45K = 0x1d2,

    /// First 46K
    PWP46K = 0x1d1,

    /// First 47K
    PWP47K = 0x1d0,

    /// First 48K
    PWP48K = 0x1cf,

    /// First 49K
    PWP49K = 0x1ce,

    /// First 50K
    PWP50K = 0x1cd,

    /// First 51K
    PWP51K = 0x1cc,

    /// First 52K
    PWP52K = 0x1cb,

    /// First 53K
    PWP53K = 0x1ca,

    /// First 54K
    PWP54K = 0x1c9,

    /// First 55K
    PWP55K = 0x1c8,

    /// First 56K
    PWP56K = 0x1c7,

    /// First 57K
    PWP57K = 0x1c6,

    /// First 58K
    PWP58K = 0x1c5,

    /// First 59K
    PWP59K = 0x1c4,

    /// First 60K
    PWP60K = 0x1c3,

    /// First 61K
    PWP61K = 0x1c2,

    /// First 62K
    PWP62K = 0x1c1,

    /// First 63K
    PWP63K = 0x1c0,

    /// First 64K
    PWP64K = 0x1bf,

    /// First 65K
    PWP65K = 0x1be,

    /// First 66K
    PWP66K = 0x1bd,

    /// First 67K
    PWP67K = 0x1bc,

    /// First 68K
    PWP68K = 0x1bb,

    /// First 69K
    PWP69K = 0x1ba,

    /// First 70K
    PWP70K = 0x1b9,

    /// First 71K
    PWP71K = 0x1b8,

    /// First 72K
    PWP72K = 0x1b7,

    /// First 73K
    PWP73K = 0x1b6,

    /// First 74K
    PWP74K = 0x1b5,

    /// First 75K
    PWP75K = 0x1b4,

    /// First 76K
    PWP76K = 0x1b3,

    /// First 77K
    PWP77K = 0x1b2,

    /// First 78K
    PWP78K = 0x1b1,

    /// First 79K
    PWP79K = 0x1b0,

    /// First 80K
    PWP80K = 0x1af,

    /// First 81K
    PWP81K = 0x1ae,

    /// First 82K
    PWP82K = 0x1ad,

    /// First 83K
    PWP83K = 0x1ac,

    /// First 84K
    PWP84K = 0x1ab,

    /// First 85K
    PWP85K = 0x1aa,

    /// First 86K
    PWP86K = 0x1a9,

    /// First 87K
    PWP87K = 0x1a8,

    /// First 88K
    PWP88K = 0x1a7,

    /// First 89K
    PWP89K = 0x1a6,

    /// First 90K
    PWP90K = 0x1a5,

    /// First 91K
    PWP91K = 0x1a4,

    /// First 92K
    PWP92K = 0x1a3,

    /// First 93K
    PWP93K = 0x1a2,

    /// First 94K
    PWP94K = 0x1a1,

    /// First 95K
    PWP95K = 0x1a0,

    /// First 96K
    PWP96K = 0x19f,

    /// First 97K
    PWP97K = 0x19e,

    /// First 98K
    PWP98K = 0x19d,

    /// First 99K
    PWP99K = 0x19c,

    /// First 100K
    PWP100K = 0x19b,

    /// First 101K
    PWP101K = 0x19a,

    /// First 102K
    PWP102K = 0x199,

    /// First 103K
    PWP103K = 0x198,

    /// First 104K
    PWP104K = 0x197,

    /// First 105K
    PWP105K = 0x196,

    /// First 106K
    PWP106K = 0x195,

    /// First 107K
    PWP107K = 0x194,

    /// First 108K
    PWP108K = 0x193,

    /// First 109K
    PWP109K = 0x192,

    /// First 110K
    PWP110K = 0x191,

    /// First 111K
    PWP111K = 0x190,

    /// First 112K
    PWP112K = 0x18f,

    /// First 113K
    PWP113K = 0x18e,

    /// First 114K
    PWP114K = 0x18d,

    /// First 115K
    PWP115K = 0x18c,

    /// First 116K
    PWP116K = 0x18b,

    /// First 117K
    PWP117K = 0x18a,

    /// First 118K
    PWP118K = 0x189,

    /// First 119K
    PWP119K = 0x188,

    /// First 120K
    PWP120K = 0x187,

    /// First 121K
    PWP121K = 0x186,

    /// First 122K
    PWP122K = 0x185,

    /// First 123K
    PWP123K = 0x184,

    /// First 124K
    PWP124K = 0x183,

    /// First 125K
    PWP125K = 0x182,

    /// First 126K
    PWP126K = 0x181,

    /// First 127K
    PWP127K = 0x180,

    /// First 128K
    PWP128K = 0x17f,

    /// First 129K
    PWP129K = 0x17e,

    /// First 130K
    PWP130K = 0x17d,

    /// First 131K
    PWP131K = 0x17c,

    /// First 132K
    PWP132K = 0x17b,

    /// First 133K
    PWP133K = 0x17a,

    /// First 134K
    PWP134K = 0x179,

    /// First 135K
    PWP135K = 0x178,

    /// First 136K
    PWP136K = 0x177,

    /// First 137K
    PWP137K = 0x176,

    /// First 138K
    PWP138K = 0x175,

    /// First 139K
    PWP139K = 0x174,

    /// First 140K
    PWP140K = 0x173,

    /// First 141K
    PWP141K = 0x172,

    /// First 142K
    PWP142K = 0x171,

    /// First 143K
    PWP143K = 0x170,

    /// First 144K
    PWP144K = 0x16f,

    /// First 145K
    PWP145K = 0x16e,

    /// First 146K
    PWP146K = 0x16d,

    /// First 147K
    PWP147K = 0x16c,

    /// First 148K
    PWP148K = 0x16b,

    /// First 149K
    PWP149K = 0x16a,

    /// First 150K
    PWP150K = 0x169,

    /// First 151K
    PWP151K = 0x168,

    /// First 152K
    PWP152K = 0x167,

    /// First 153K
    PWP153K = 0x166,

    /// First 154K
    PWP154K = 0x165,

    /// First 155K
    PWP155K = 0x164,

    /// First 156K
    PWP156K = 0x163,

    /// First 157K
    PWP157K = 0x162,

    /// First 158K
    PWP158K = 0x161,

    /// First 159K
    PWP159K = 0x160,

    /// First 160K
    PWP160K = 0x15f,

    /// First 161K
    PWP161K = 0x15e,

    /// First 162K
    PWP162K = 0x15d,

    /// First 163K
    PWP163K = 0x15c,

    /// First 164K
    PWP164K = 0x15b,

    /// First 165K
    PWP165K = 0x15a,

    /// First 166K
    PWP166K = 0x159,

    /// First 167K
    PWP167K = 0x158,

    /// First 168K
    PWP168K = 0x157,

    /// First 169K
    PWP169K = 0x156,

    /// First 170K
    PWP170K = 0x155,

    /// First 171K
    PWP171K = 0x154,

    /// First 172K
    PWP172K = 0x153,

    /// First 173K
    PWP173K = 0x152,

    /// First 174K
    PWP174K = 0x151,

    /// First 175K
    PWP175K = 0x150,

    /// First 176K
    PWP176K = 0x14f,

    /// First 177K
    PWP177K = 0x14e,

    /// First 178K
    PWP178K = 0x14d,

    /// First 179K
    PWP179K = 0x14c,

    /// First 180K
    PWP180K = 0x14b,

    /// First 181K
    PWP181K = 0x14a,

    /// First 182K
    PWP182K = 0x149,

    /// First 183K
    PWP183K = 0x148,

    /// First 184K
    PWP184K = 0x147,

    /// First 185K
    PWP185K = 0x146,

    /// First 186K
    PWP186K = 0x145,

    /// First 187K
    PWP187K = 0x144,

    /// First 188K
    PWP188K = 0x143,

    /// First 189K
    PWP189K = 0x142,

    /// First 190K
    PWP190K = 0x141,

    /// First 191K
    PWP191K = 0x140,

    /// First 192K
    PWP192K = 0x13f,

    /// First 193K
    PWP193K = 0x13e,

    /// First 194K
    PWP194K = 0x13d,

    /// First 195K
    PWP195K = 0x13c,

    /// First 196K
    PWP196K = 0x13b,

    /// First 197K
    PWP197K = 0x13a,

    /// First 198K
    PWP198K = 0x139,

    /// First 199K
    PWP199K = 0x138,

    /// First 200K
    PWP200K = 0x137,

    /// First 201K
    PWP201K = 0x136,

    /// First 202K
    PWP202K = 0x135,

    /// First 203K
    PWP203K = 0x134,

    /// First 204K
    PWP204K = 0x133,

    /// First 205K
    PWP205K = 0x132,

    /// First 206K
    PWP206K = 0x131,

    /// First 207K
    PWP207K = 0x130,

    /// First 208K
    PWP208K = 0x12f,

    /// First 209K
    PWP209K = 0x12e,

    /// First 210K
    PWP210K = 0x12d,

    /// First 211K
    PWP211K = 0x12c,

    /// First 212K
    PWP212K = 0x12b,

    /// First 213K
    PWP213K = 0x12a,

    /// First 214K
    PWP214K = 0x129,

    /// First 215K
    PWP215K = 0x128,

    /// First 216K
    PWP216K = 0x127,

    /// First 217K
    PWP217K = 0x126,

    /// First 218K
    PWP218K = 0x125,

    /// First 219K
    PWP219K = 0x124,

    /// First 220K
    PWP220K = 0x123,

    /// First 221K
    PWP221K = 0x122,

    /// First 222K
    PWP222K = 0x121,

    /// First 223K
    PWP223K = 0x120,

    /// First 224K
    PWP224K = 0x11f,

    /// First 225K
    PWP225K = 0x11e,

    /// First 226K
    PWP226K = 0x11d,

    /// First 227K
    PWP227K = 0x11c,

    /// First 228K
    PWP228K = 0x11b,

    /// First 229K
    PWP229K = 0x11a,

    /// First 230K
    PWP230K = 0x119,

    /// First 231K
    PWP231K = 0x118,

    /// First 232K
    PWP232K = 0x117,

    /// First 233K
    PWP233K = 0x116,

    /// First 234K
    PWP234K = 0x115,

    /// First 235K
    PWP235K = 0x114,

    /// First 236K
    PWP236K = 0x113,

    /// First 237K
    PWP237K = 0x112,

    /// First 238K
    PWP238K = 0x111,

    /// First 239K
    PWP239K = 0x110,

    /// First 240K
    PWP240K = 0x10f,

    /// First 241K
    PWP241K = 0x10e,

    /// First 242K
    PWP242K = 0x10d,

    /// First 243K
    PWP243K = 0x10c,

    /// First 244K
    PWP244K = 0x10b,

    /// First 245K
    PWP245K = 0x10a,

    /// First 246K
    PWP246K = 0x109,

    /// First 247K
    PWP247K = 0x108,

    /// First 248K
    PWP248K = 0x107,

    /// First 249K
    PWP249K = 0x106,

    /// First 250K
    PWP250K = 0x105,

    /// First 251K
    PWP251K = 0x104,

    /// First 252K
    PWP252K = 0x103,

    /// First 253K
    PWP253K = 0x102,

    /// First 254K
    PWP254K = 0x101,

    /// First 255K
    PWP255K = 0x100,

    /// First 256K
    PWP256K = 0x0ff,
}

/// Boot Flash Write Protect bit
pub enum BWP {
    /// Protection Enabled
    ON = 0x0,

    /// Protection Disabled
    OFF = 0x1,
}

/// Code Protect
pub enum CP {
    /// Protection Enabled
    ON = 0x0,

    /// Protection Disabled
    OFF = 0x1,
}

/// Configuration word sector
#[repr(C)]
pub struct ConfigSector {
    DEVCFG3: u32,
    DEVCFG2: u32,
    DEVCFG1: u32,
    DEVCFG0: u32,
}

impl ConfigSector {
    /// Create a builder
    pub const fn default() -> ConfigSectorBuilder {
        ConfigSectorBuilder {
            DEVCFG3: 0xffffffff,
            DEVCFG2: 0xffffffff,
            DEVCFG1: 0xffffffff,
            DEVCFG0: 0x7fffffff,
        }
    }

    /// Convert into a array of 32 bit words consuming this ConfigSector
    pub const fn into_array(self) -> [u32; CONFIG_SECTOR_LENGTH] {
        [self.DEVCFG3, self.DEVCFG2, self.DEVCFG1, self.DEVCFG0]
    }
}
/// Configuration word sector builder
pub struct ConfigSectorBuilder {
    DEVCFG3: u32,
    DEVCFG2: u32,
    DEVCFG1: u32,
    DEVCFG0: u32,
}

impl ConfigSectorBuilder {
    pub const fn USERID(mut self, v: USERID) -> Self {
        self.DEVCFG3 &= !0x0000ffff;
        self.DEVCFG3 |= v as u32;
        self
    }

    pub const fn PMDL1WAY(mut self, v: PMDL1WAY) -> Self {
        self.DEVCFG3 &= !0x10000000;
        self.DEVCFG3 |= (v as u32) << 28;
        self
    }

    pub const fn IOL1WAY(mut self, v: IOL1WAY) -> Self {
        self.DEVCFG3 &= !0x20000000;
        self.DEVCFG3 |= (v as u32) << 29;
        self
    }

    pub const fn FUSBIDIO(mut self, v: FUSBIDIO) -> Self {
        self.DEVCFG3 &= !0x40000000;
        self.DEVCFG3 |= (v as u32) << 30;
        self
    }

    pub const fn FVBUSONIO(mut self, v: FVBUSONIO) -> Self {
        self.DEVCFG3 &= !0x80000000;
        self.DEVCFG3 |= (v as u32) << 31;
        self
    }

    pub const fn FPLLIDIV(mut self, v: FPLLIDIV) -> Self {
        self.DEVCFG2 &= !0x00000007;
        self.DEVCFG2 |= v as u32;
        self
    }

    pub const fn FPLLMUL(mut self, v: FPLLMUL) -> Self {
        self.DEVCFG2 &= !0x00000070;
        self.DEVCFG2 |= (v as u32) << 4;
        self
    }

    pub const fn UPLLIDIV(mut self, v: UPLLIDIV) -> Self {
        self.DEVCFG2 &= !0x00000700;
        self.DEVCFG2 |= (v as u32) << 8;
        self
    }

    pub const fn UPLLEN(mut self, v: UPLLEN) -> Self {
        self.DEVCFG2 &= !0x00008000;
        self.DEVCFG2 |= (v as u32) << 15;
        self
    }

    pub const fn FPLLODIV(mut self, v: FPLLODIV) -> Self {
        self.DEVCFG2 &= !0x00070000;
        self.DEVCFG2 |= (v as u32) << 16;
        self
    }

    pub const fn FNOSC(mut self, v: FNOSC) -> Self {
        self.DEVCFG1 &= !0x00000007;
        self.DEVCFG1 |= v as u32;
        self
    }

    pub const fn FSOSCEN(mut self, v: FSOSCEN) -> Self {
        self.DEVCFG1 &= !0x00000020;
        self.DEVCFG1 |= (v as u32) << 5;
        self
    }

    pub const fn IESO(mut self, v: IESO) -> Self {
        self.DEVCFG1 &= !0x00000080;
        self.DEVCFG1 |= (v as u32) << 7;
        self
    }

    pub const fn POSCMOD(mut self, v: POSCMOD) -> Self {
        self.DEVCFG1 &= !0x00000300;
        self.DEVCFG1 |= (v as u32) << 8;
        self
    }

    pub const fn OSCIOFNC(mut self, v: OSCIOFNC) -> Self {
        self.DEVCFG1 &= !0x00000400;
        self.DEVCFG1 |= (v as u32) << 10;
        self
    }

    pub const fn FPBDIV(mut self, v: FPBDIV) -> Self {
        self.DEVCFG1 &= !0x00003000;
        self.DEVCFG1 |= (v as u32) << 12;
        self
    }

    pub const fn FCKSM(mut self, v: FCKSM) -> Self {
        self.DEVCFG1 &= !0x0000c000;
        self.DEVCFG1 |= (v as u32) << 14;
        self
    }

    pub const fn WDTPS(mut self, v: WDTPS) -> Self {
        self.DEVCFG1 &= !0x001f0000;
        self.DEVCFG1 |= (v as u32) << 16;
        self
    }

    pub const fn WINDIS(mut self, v: WINDIS) -> Self {
        self.DEVCFG1 &= !0x00400000;
        self.DEVCFG1 |= (v as u32) << 22;
        self
    }

    pub const fn FWDTEN(mut self, v: FWDTEN) -> Self {
        self.DEVCFG1 &= !0x00800000;
        self.DEVCFG1 |= (v as u32) << 23;
        self
    }

    pub const fn FWDTWINSZ(mut self, v: FWDTWINSZ) -> Self {
        self.DEVCFG1 &= !0x03000000;
        self.DEVCFG1 |= (v as u32) << 24;
        self
    }

    pub const fn DEBUG(mut self, v: DEBUG) -> Self {
        self.DEVCFG0 &= !0x00000003;
        self.DEVCFG0 |= v as u32;
        self
    }

    pub const fn JTAGEN(mut self, v: JTAGEN) -> Self {
        self.DEVCFG0 &= !0x00000004;
        self.DEVCFG0 |= (v as u32) << 2;
        self
    }

    pub const fn ICESEL(mut self, v: ICESEL) -> Self {
        self.DEVCFG0 &= !0x00000018;
        self.DEVCFG0 |= (v as u32) << 3;
        self
    }

    pub const fn PWP(mut self, v: PWP) -> Self {
        self.DEVCFG0 &= !0x0007fc00;
        self.DEVCFG0 |= (v as u32) << 10;
        self
    }

    pub const fn BWP(mut self, v: BWP) -> Self {
        self.DEVCFG0 &= !0x01000000;
        self.DEVCFG0 |= (v as u32) << 24;
        self
    }

    pub const fn CP(mut self, v: CP) -> Self {
        self.DEVCFG0 &= !0x10000000;
        self.DEVCFG0 |= (v as u32) << 28;
        self
    }

    pub const fn build(self) -> ConfigSector {
        ConfigSector {
            DEVCFG3: self.DEVCFG3,
            DEVCFG2: self.DEVCFG2,
            DEVCFG1: self.DEVCFG1,
            DEVCFG0: self.DEVCFG0,
        }
    }
}
