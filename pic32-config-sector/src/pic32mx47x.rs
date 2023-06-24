#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
/// Length of config word sector in words
pub const CONFIG_SECTOR_LENGTH: usize = 4;

type USERID = u16;

/// Shadow Register Set Priority Select
pub enum FSRSSEL {
    /// SRS Priority 0
    PRIORITY_0 = 0x0,

    /// SRS Priority 1
    PRIORITY_1 = 0x1,

    /// SRS Priority 2
    PRIORITY_2 = 0x2,

    /// SRS Priority 3
    PRIORITY_3 = 0x3,

    /// SRS Priority 4
    PRIORITY_4 = 0x4,

    /// SRS Priority 5
    PRIORITY_5 = 0x5,

    /// SRS Priority 6
    PRIORITY_6 = 0x6,

    /// SRS Priority 7
    PRIORITY_7 = 0x7,
}

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
}

/// Program Flash Write Protect
pub enum PWP {
    /// Disable
    OFF = 0xff,

    /// First 4K
    PWP4K = 0xfe,

    /// First 8K
    PWP8K = 0xfd,

    /// First 12K
    PWP12K = 0xfc,

    /// First 16K
    PWP16K = 0xfb,

    /// First 20K
    PWP20K = 0xfa,

    /// First 24K
    PWP24K = 0xf9,

    /// First 28K
    PWP28K = 0xf8,

    /// First 32K
    PWP32K = 0xf7,

    /// First 36K
    PWP36K = 0xf6,

    /// First 40K
    PWP40K = 0xf5,

    /// First 44K
    PWP44K = 0xf4,

    /// First 48K
    PWP48K = 0xf3,

    /// First 52K
    PWP52K = 0xf2,

    /// First 56K
    PWP56K = 0xf1,

    /// First 60K
    PWP60K = 0xf0,

    /// First 64K
    PWP64K = 0xef,

    /// First 68K
    PWP68K = 0xee,

    /// First 72K
    PWP72K = 0xed,

    /// First 76K
    PWP76K = 0xec,

    /// First 80K
    PWP80K = 0xeb,

    /// First 84K
    PWP84K = 0xea,

    /// First 88K
    PWP88K = 0xe9,

    /// First 92K
    PWP92K = 0xe8,

    /// First 96K
    PWP96K = 0xe7,

    /// First 100K
    PWP100K = 0xe6,

    /// First 104K
    PWP104K = 0xe5,

    /// First 108K
    PWP108K = 0xe4,

    /// First 112K
    PWP112K = 0xe3,

    /// First 116K
    PWP116K = 0xe2,

    /// First 120K
    PWP120K = 0xe1,

    /// First 124K
    PWP124K = 0xe0,

    /// First 128K
    PWP128K = 0xdf,

    /// First 132K
    PWP132K = 0xde,

    /// First 136K
    PWP136K = 0xdd,

    /// First 140K
    PWP140K = 0xdc,

    /// First 144K
    PWP144K = 0xdb,

    /// First 148K
    PWP148K = 0xda,

    /// First 152K
    PWP152K = 0xd9,

    /// First 156K
    PWP156K = 0xd8,

    /// First 160K
    PWP160K = 0xd7,

    /// First 164K
    PWP164K = 0xd6,

    /// First 168K
    PWP168K = 0xd5,

    /// First 172K
    PWP172K = 0xd4,

    /// First 176K
    PWP176K = 0xd3,

    /// First 180K
    PWP180K = 0xd2,

    /// First 184K
    PWP184K = 0xd1,

    /// First 188K
    PWP188K = 0xd0,

    /// First 192K
    PWP192K = 0xcf,

    /// First 196K
    PWP196K = 0xce,

    /// First 200K
    PWP200K = 0xcd,

    /// First 204K
    PWP204K = 0xcc,

    /// First 208K
    PWP208K = 0xcb,

    /// First 212K
    PWP212K = 0xca,

    /// First 216K
    PWP216K = 0xc9,

    /// First 220K
    PWP220K = 0xc8,

    /// First 224K
    PWP224K = 0xc7,

    /// First 228K
    PWP228K = 0xc6,

    /// First 232K
    PWP232K = 0xc5,

    /// First 236K
    PWP236K = 0xc4,

    /// First 240K
    PWP240K = 0xc3,

    /// First 244K
    PWP244K = 0xc2,

    /// First 248K
    PWP248K = 0xc1,

    /// First 252K
    PWP252K = 0xc0,

    /// First 256K
    PWP256K = 0xbf,

    /// First 260K
    PWP260K = 0xbe,

    /// First 264K
    PWP264K = 0xbd,

    /// First 268K
    PWP268K = 0xbc,

    /// First 272K
    PWP272K = 0xbb,

    /// First 276K
    PWP276K = 0xba,

    /// First 280K
    PWP280K = 0xb9,

    /// First 284K
    PWP284K = 0xb8,

    /// First 288K
    PWP288K = 0xb7,

    /// First 292K
    PWP292K = 0xb6,

    /// First 296K
    PWP296K = 0xb5,

    /// First 300K
    PWP300K = 0xb4,

    /// First 304K
    PWP304K = 0xb3,

    /// First 308K
    PWP308K = 0xb2,

    /// First 312K
    PWP312K = 0xb1,

    /// First 316K
    PWP316K = 0xb0,

    /// First 320K
    PWP320K = 0xaf,

    /// First 324K
    PWP324K = 0xae,

    /// First 328K
    PWP328K = 0xad,

    /// First 332K
    PWP332K = 0xac,

    /// First 336K
    PWP336K = 0xab,

    /// First 340K
    PWP340K = 0xaa,

    /// First 344K
    PWP344K = 0xa9,

    /// First 348K
    PWP348K = 0xa8,

    /// First 352K
    PWP352K = 0xa7,

    /// First 356K
    PWP356K = 0xa6,

    /// First 360K
    PWP360K = 0xa5,

    /// First 364K
    PWP364K = 0xa4,

    /// First 368K
    PWP368K = 0xa3,

    /// First 372K
    PWP372K = 0xa2,

    /// First 376K
    PWP376K = 0xa1,

    /// First 380K
    PWP380K = 0xa0,

    /// First 384K
    PWP384K = 0x9f,

    /// First 388K
    PWP388K = 0x9e,

    /// First 392K
    PWP392K = 0x9d,

    /// First 396K
    PWP396K = 0x9c,

    /// First 400K
    PWP400K = 0x9b,

    /// First 404K
    PWP404K = 0x9a,

    /// First 408K
    PWP408K = 0x99,

    /// First 412K
    PWP412K = 0x98,

    /// First 416K
    PWP416K = 0x97,

    /// First 420K
    PWP420K = 0x96,

    /// First 424K
    PWP424K = 0x95,

    /// First 428K
    PWP428K = 0x94,

    /// First 432K
    PWP432K = 0x93,

    /// First 436K
    PWP436K = 0x92,

    /// First 440K
    PWP440K = 0x91,

    /// First 444K
    PWP444K = 0x90,

    /// First 448K
    PWP448K = 0x8f,

    /// First 452K
    PWP452K = 0x8e,

    /// First 456K
    PWP456K = 0x8d,

    /// First 460K
    PWP460K = 0x8c,

    /// First 464K
    PWP464K = 0x8b,

    /// First 468K
    PWP468K = 0x8a,

    /// First 472K
    PWP472K = 0x89,

    /// First 476K
    PWP476K = 0x88,

    /// First 480K
    PWP480K = 0x87,

    /// First 484K
    PWP484K = 0x86,

    /// First 488K
    PWP488K = 0x85,

    /// First 492K
    PWP492K = 0x84,

    /// First 496K
    PWP496K = 0x83,

    /// First 500K
    PWP500K = 0x82,

    /// First 504K
    PWP504K = 0x81,

    /// First 508K
    PWP508K = 0x80,

    /// First 512K
    PWP512K = 0x7f,
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

    pub const fn FSRSSEL(mut self, v: FSRSSEL) -> Self {
        self.DEVCFG3 &= !0x00070000;
        self.DEVCFG3 |= (v as u32) << 16;
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
        self.DEVCFG0 &= !0x000ff000;
        self.DEVCFG0 |= (v as u32) << 12;
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
