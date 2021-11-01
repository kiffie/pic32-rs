#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
/// Length of config word sector in words
pub const CONFIG_SECTOR_LENGTH: usize = 16;

type USERID = u16;

/// Ethernet RMII/MII Enable
pub enum FMIIEN {
    /// RMII Enabled
    OFF = 0x0,

    /// MII Enabled
    ON = 0x1,
}

/// Ethernet I/O Pin Select
pub enum FETHIO {
    /// Alternate Ethernet I/O
    OFF = 0x0,

    /// Default Ethernet I/O
    ON = 0x1,
}

/// Permission Group Lock One Way Configuration
pub enum PGL1WAY {
    /// Allow only one reconfiguration
    ON = 0x1,

    /// Allow multiple reconfigurations
    OFF = 0x0,
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

/// USB USBID Selection
pub enum FUSBIDIO {
    /// Controlled by Port Function
    OFF = 0x0,

    /// Controlled by the USB Module
    ON = 0x1,
}

/// System PLL Input Divider
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

    /// 7x Divider
    DIV_7 = 0x6,

    /// 8x Divider
    DIV_8 = 0x7,
}

/// System PLL Input Range
pub enum FPLLRNG {
    /// Bypass
    RANGE_BYPASS = 0x0,

    /// 5-10 MHz Input
    RANGE_5_10_MHZ = 0x1,

    /// 8-16 MHz Input
    RANGE_8_16_MHZ = 0x2,

    /// 13-26 MHz Input
    RANGE_13_26_MHZ = 0x3,

    /// 21-42 MHz Input
    RANGE_21_42_MHZ = 0x4,

    /// 34-68 MHz Input
    RANGE_34_68_MHZ = 0x5,
}

/// System PLL Input Clock Selection
pub enum FPLLICLK {
    /// FRC is input to the System PLL
    PLL_FRC = 0x1,

    /// POSC is input to the System PLL
    PLL_POSC = 0x0,
}

/// System PLL Multiplier
pub enum FPLLMULT {
    /// PLL Multiply by 1
    MUL_1 = 0x0,

    /// PLL Multiply by 2
    MUL_2 = 0x1,

    /// PLL Multiply by 3
    MUL_3 = 0x2,

    /// PLL Multiply by 4
    MUL_4 = 0x3,

    /// PLL Multiply by 5
    MUL_5 = 0x4,

    /// PLL Multiply by 6
    MUL_6 = 0x5,

    /// PLL Multiply by 7
    MUL_7 = 0x6,

    /// PLL Multiply by 8
    MUL_8 = 0x7,

    /// PLL Multiply by 9
    MUL_9 = 0x8,

    /// PLL Multiply by 10
    MUL_10 = 0x9,

    /// PLL Multiply by 11
    MUL_11 = 0xa,

    /// PLL Multiply by 12
    MUL_12 = 0xb,

    /// PLL Multiply by 13
    MUL_13 = 0xc,

    /// PLL Multiply by 14
    MUL_14 = 0xd,

    /// PLL Multiply by 15
    MUL_15 = 0xe,

    /// PLL Multiply by 16
    MUL_16 = 0xf,

    /// PLL Multiply by 17
    MUL_17 = 0x10,

    /// PLL Multiply by 18
    MUL_18 = 0x11,

    /// PLL Multiply by 19
    MUL_19 = 0x12,

    /// PLL Multiply by 20
    MUL_20 = 0x13,

    /// PLL Multiply by 21
    MUL_21 = 0x14,

    /// PLL Multiply by 22
    MUL_22 = 0x15,

    /// PLL Multiply by 23
    MUL_23 = 0x16,

    /// PLL Multiply by 24
    MUL_24 = 0x17,

    /// PLL Multiply by 25
    MUL_25 = 0x18,

    /// PLL Multiply by 26
    MUL_26 = 0x19,

    /// PLL Multiply by 27
    MUL_27 = 0x1a,

    /// PLL Multiply by 28
    MUL_28 = 0x1b,

    /// PLL Multiply by 29
    MUL_29 = 0x1c,

    /// PLL Multiply by 30
    MUL_30 = 0x1d,

    /// PLL Multiply by 31
    MUL_31 = 0x1e,

    /// PLL Multiply by 32
    MUL_32 = 0x1f,

    /// PLL Multiply by 33
    MUL_33 = 0x20,

    /// PLL Multiply by 34
    MUL_34 = 0x21,

    /// PLL Multiply by 35
    MUL_35 = 0x22,

    /// PLL Multiply by 36
    MUL_36 = 0x23,

    /// PLL Multiply by 37
    MUL_37 = 0x24,

    /// PLL Multiply by 38
    MUL_38 = 0x25,

    /// PLL Multiply by 39
    MUL_39 = 0x26,

    /// PLL Multiply by 40
    MUL_40 = 0x27,

    /// PLL Multiply by 41
    MUL_41 = 0x28,

    /// PLL Multiply by 42
    MUL_42 = 0x29,

    /// PLL Multiply by 43
    MUL_43 = 0x2a,

    /// PLL Multiply by 44
    MUL_44 = 0x2b,

    /// PLL Multiply by 45
    MUL_45 = 0x2c,

    /// PLL Multiply by 46
    MUL_46 = 0x2d,

    /// PLL Multiply by 47
    MUL_47 = 0x2e,

    /// PLL Multiply by 48
    MUL_48 = 0x2f,

    /// PLL Multiply by 49
    MUL_49 = 0x30,

    /// PLL Multiply by 50
    MUL_50 = 0x31,

    /// PLL Multiply by 51
    MUL_51 = 0x32,

    /// PLL Multiply by 52
    MUL_52 = 0x33,

    /// PLL Multiply by 53
    MUL_53 = 0x34,

    /// PLL Multiply by 54
    MUL_54 = 0x35,

    /// PLL Multiply by 55
    MUL_55 = 0x36,

    /// PLL Multiply by 56
    MUL_56 = 0x37,

    /// PLL Multiply by 57
    MUL_57 = 0x38,

    /// PLL Multiply by 58
    MUL_58 = 0x39,

    /// PLL Multiply by 59
    MUL_59 = 0x3a,

    /// PLL Multiply by 60
    MUL_60 = 0x3b,

    /// PLL Multiply by 61
    MUL_61 = 0x3c,

    /// PLL Multiply by 62
    MUL_62 = 0x3d,

    /// PLL Multiply by 63
    MUL_63 = 0x3e,

    /// PLL Multiply by 64
    MUL_64 = 0x3f,

    /// PLL Multiply by 65
    MUL_65 = 0x40,

    /// PLL Multiply by 66
    MUL_66 = 0x41,

    /// PLL Multiply by 67
    MUL_67 = 0x42,

    /// PLL Multiply by 68
    MUL_68 = 0x43,

    /// PLL Multiply by 69
    MUL_69 = 0x44,

    /// PLL Multiply by 70
    MUL_70 = 0x45,

    /// PLL Multiply by 71
    MUL_71 = 0x46,

    /// PLL Multiply by 72
    MUL_72 = 0x47,

    /// PLL Multiply by 73
    MUL_73 = 0x48,

    /// PLL Multiply by 74
    MUL_74 = 0x49,

    /// PLL Multiply by 75
    MUL_75 = 0x4a,

    /// PLL Multiply by 76
    MUL_76 = 0x4b,

    /// PLL Multiply by 77
    MUL_77 = 0x4c,

    /// PLL Multiply by 78
    MUL_78 = 0x4d,

    /// PLL Multiply by 79
    MUL_79 = 0x4e,

    /// PLL Multiply by 80
    MUL_80 = 0x4f,

    /// PLL Multiply by 81
    MUL_81 = 0x50,

    /// PLL Multiply by 82
    MUL_82 = 0x51,

    /// PLL Multiply by 83
    MUL_83 = 0x52,

    /// PLL Multiply by 84
    MUL_84 = 0x53,

    /// PLL Multiply by 85
    MUL_85 = 0x54,

    /// PLL Multiply by 86
    MUL_86 = 0x55,

    /// PLL Multiply by 87
    MUL_87 = 0x56,

    /// PLL Multiply by 88
    MUL_88 = 0x57,

    /// PLL Multiply by 89
    MUL_89 = 0x58,

    /// PLL Multiply by 90
    MUL_90 = 0x59,

    /// PLL Multiply by 91
    MUL_91 = 0x5a,

    /// PLL Multiply by 92
    MUL_92 = 0x5b,

    /// PLL Multiply by 93
    MUL_93 = 0x5c,

    /// PLL Multiply by 94
    MUL_94 = 0x5d,

    /// PLL Multiply by 95
    MUL_95 = 0x5e,

    /// PLL Multiply by 96
    MUL_96 = 0x5f,

    /// PLL Multiply by 97
    MUL_97 = 0x60,

    /// PLL Multiply by 98
    MUL_98 = 0x61,

    /// PLL Multiply by 99
    MUL_99 = 0x62,

    /// PLL Multiply by 100
    MUL_100 = 0x63,

    /// PLL Multiply by 101
    MUL_101 = 0x64,

    /// PLL Multiply by 102
    MUL_102 = 0x65,

    /// PLL Multiply by 103
    MUL_103 = 0x66,

    /// PLL Multiply by 104
    MUL_104 = 0x67,

    /// PLL Multiply by 105
    MUL_105 = 0x68,

    /// PLL Multiply by 106
    MUL_106 = 0x69,

    /// PLL Multiply by 107
    MUL_107 = 0x6a,

    /// PLL Multiply by 108
    MUL_108 = 0x6b,

    /// PLL Multiply by 109
    MUL_109 = 0x6c,

    /// PLL Multiply by 110
    MUL_110 = 0x6d,

    /// PLL Multiply by 111
    MUL_111 = 0x6e,

    /// PLL Multiply by 112
    MUL_112 = 0x6f,

    /// PLL Multiply by 113
    MUL_113 = 0x70,

    /// PLL Multiply by 114
    MUL_114 = 0x71,

    /// PLL Multiply by 115
    MUL_115 = 0x72,

    /// PLL Multiply by 116
    MUL_116 = 0x73,

    /// PLL Multiply by 117
    MUL_117 = 0x74,

    /// PLL Multiply by 118
    MUL_118 = 0x75,

    /// PLL Multiply by 119
    MUL_119 = 0x76,

    /// PLL Multiply by 120
    MUL_120 = 0x77,

    /// PLL Multiply by 121
    MUL_121 = 0x78,

    /// PLL Multiply by 122
    MUL_122 = 0x79,

    /// PLL Multiply by 123
    MUL_123 = 0x7a,

    /// PLL Multiply by 124
    MUL_124 = 0x7b,

    /// PLL Multiply by 125
    MUL_125 = 0x7c,

    /// PLL Multiply by 126
    MUL_126 = 0x7d,

    /// PLL Multiply by 127
    MUL_127 = 0x7e,

    /// PLL Multiply by 128
    MUL_128 = 0x7f,
}

/// System PLL Output Clock Divider
pub enum FPLLODIV {
    /// 2x Divider
    DIV_2 = 0x1,

    /// 4x Divider
    DIV_4 = 0x2,

    /// 8x Divider
    DIV_8 = 0x3,

    /// 16x Divider
    DIV_16 = 0x4,

    /// 32x Divider
    DIV_32 = 0x7,
}

/// USB PLL Input Frequency Selection
pub enum UPLLFSEL {
    /// USB PLL input is 24 MHz
    FREQ_24MHZ = 0x1,

    /// USB PLL input is 12 MHz
    FREQ_12MHZ = 0x0,
}

/// Oscillator Selection Bits
pub enum FNOSC {
    /// Fast RC Osc w/Div-by-N (FRCDIV)
    FRCDIV = 0x7,

    /// System PLL
    SPLL = 0x1,

    /// Primary Osc (HS,EC)
    POSC = 0x2,

    /// Low Power Secondary Osc (SOSC)
    SOSC = 0x4,

    /// Low Power RC Osc (LPRC)
    LPRC = 0x5,
}

/// DMT Count Window Interval
pub enum DMTINTV {
    /// Window/Interval value is zero
    WIN_0 = 0x0,

    /// Window/Interval value is 1/2 counter value
    WIN_1_2 = 0x1,

    /// Window/Interval value is 3/4 counter value
    WIN_3_4 = 0x2,

    /// Window/Interval value is 7/8 counter value
    WIN_7_8 = 0x3,

    /// Window/Interval value is 15/16 counter value
    WIN_15_16 = 0x4,

    /// Window/Interval value is 31/32 counter value
    WIN_31_32 = 0x5,

    /// Window/Interval value is 63/64 counter value
    WIN_63_64 = 0x6,

    /// Window/Interval value is 127/128 counter value
    WIN_127_128 = 0x7,
}

/// Secondary Oscillator Enable
pub enum FSOSCEN {
    /// Disable SOSC
    OFF = 0x0,

    /// Enable SOSC
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

/// Clock Switching and Monitor Selection
pub enum FCKSM {
    /// Clock Switch Disabled, FSCM Disabled
    CSDCMD = 0x0,

    /// Clock Switch Enabled, FSCM Disabled
    CSECMD = 0x1,

    /// Clock Switch Disabled, FSCM Enabled
    CSDCME = 0x2,

    /// Clock Switch Enabled, FSCM Enabled
    CSECME = 0x3,
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

/// Watchdog Timer Stop During Flash Programming
pub enum WDTSPGM {
    /// WDT runs during Flash programming
    RUN = 0x0,

    /// WDT stops during Flash programming
    STOP = 0x1,
}

/// Watchdog Timer Window Mode
pub enum WINDIS {
    /// Watchdog Timer is in non-Window mode
    NORMAL = 0x1,

    /// Watchdog Timer is in Window mode
    WINDOW = 0x0,
}

/// Watchdog Timer Enable
pub enum FWDTEN {
    /// WDT Disabled
    OFF = 0x0,

    /// WDT Enabled
    ON = 0x1,
}

/// Watchdog Timer Window Size
pub enum FWDTWINSZ {
    /// Window size is 25%
    WINSZ_25 = 0x3,

    /// Window size is 37.5%
    WINSZ_37 = 0x2,

    /// Window size is 50%
    WINSZ_50 = 0x1,

    /// Window size is 75%
    WINSZ_75 = 0x0,
}

/// Deadman Timer Count Selection
pub enum DMTCNT {
    /// 2^8 (256)
    DMT8 = 0x0,

    /// 2^9 (512)
    DMT9 = 0x1,

    /// 2^10 (1024)
    DMT10 = 0x2,

    /// 2^11 (2048)
    DMT11 = 0x3,

    /// 2^12 (4096)
    DMT12 = 0x4,

    /// 2^13 (8192)
    DMT13 = 0x5,

    /// 2^14 (16384)
    DMT14 = 0x6,

    /// 2^15 (32768)
    DMT15 = 0x7,

    /// 2^16 (65536)
    DMT16 = 0x8,

    /// 2^17 (131072)
    DMT17 = 0x9,

    /// 2^18 (262144)
    DMT18 = 0xa,

    /// 2^19 (524288)
    DMT19 = 0xb,

    /// 2^20 (1048576)
    DMT20 = 0xc,

    /// 2^21 (2097152)
    DMT21 = 0xd,

    /// 2^22 (4194304)
    DMT22 = 0xe,

    /// 2^23 (8388608)
    DMT23 = 0xf,

    /// 2^24 (16777216)
    DMT24 = 0x10,

    /// 2^25 (33554432)
    DMT25 = 0x11,

    /// 2^26 (67108864)
    DMT26 = 0x12,

    /// 2^27 (134217728)
    DMT27 = 0x13,

    /// 2^28 (268435456)
    DMT28 = 0x14,

    /// 2^29 (536870912)
    DMT29 = 0x15,

    /// 2^30 (1073741824)
    DMT30 = 0x16,

    /// 2^31 (2147483648)
    DMT31 = 0x17,
}

/// Deadman Timer Enable
pub enum FDMTEN {
    /// Deadman Timer is enabled
    ON = 0x1,

    /// Deadman Timer is disabled
    OFF = 0x0,
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
}

/// Trace Enable
pub enum TRCEN {
    /// Trace features in the CPU are enabled
    ON = 0x1,

    /// Trace features in the CPU are disabled
    OFF = 0x0,
}

/// Boot ISA Selection
pub enum BOOTISA {
    /// Boot code and Exception code is MIPS32
    MIPS32 = 0x1,

    /// Boot code and Exception code is microMIPS
    MICROMIPS = 0x0,
}

/// Dynamic Flash ECC Configuration
pub enum FECCCON {
    /// Flash ECC is enabled (ECCCON bits are locked)
    ON = 0x0,

    /// Dynamic Flash ECC is enabled (ECCCON bits are locked)
    DYNAMIC = 0x1,

    /// ECC and Dynamic ECC are disabled (ECCCON bits are locked)
    OFF_LOCKED = 0x2,

    /// ECC and Dynamic ECC are disabled (ECCCON bits are writable)
    OFF_UNLOCKED = 0x3,
}

/// Flash Sleep Mode
pub enum FSLEEP {
    /// Flash is powered down when the device is in Sleep mode
    OFF = 0x1,

    /// Flash power down is controlled by the VREGS bit
    VREGS = 0x0,
}

/// Debug Mode CPU Access Permission
pub enum DBGPER {
    /// Allow CPU access to Permission Group 2 permission regions
    ALLOW_PG2 = 0x4,

    /// Allow CPU access to Permission Group 1 permission regions
    ALLOW_PG1 = 0x2,

    /// Allow CPU access to Permission Group 0 permission regions
    ALLOW_PG0 = 0x1,

    /// PG0: Allow PG1: Allow PG2: Deny
    PG_1_0 = 0x3,

    /// PG0: Allow PG1: Deny PG2: Allow
    PG_2_0 = 0x5,

    /// PG0: Deny PG1: Allow PG2: Allow
    PG_2_1 = 0x6,

    /// Allow CPU access to all permission regions
    PG_ALL = 0x7,

    /// Deny CPU access to all permission regions
    PG_NONE = 0x0,
}

/// Soft Master Clear Enable bit
pub enum SMCLR {
    /// MCLR pin generates a normal system Reset
    MCLR_NORM = 0x1,

    /// MCLR pin generates an emulated POR Reset
    MCLR_POR = 0x0,
}

/// Secondary Oscillator Gain Control bits
pub enum SOSCGAIN {
    /// Gain level 3 (Highest)
    GAIN_LEVEL_3 = 0x3,

    /// Gain level 2
    GAIN_LEVEL_2 = 0x2,

    /// Gain level 1
    GAIN_LEVEL_1 = 0x1,

    /// Gain level 0 (Lowest)
    GAIN_LEVEL_0 = 0x0,
}

/// Secondary Oscillator Boost Kick Start Enable bit
pub enum SOSCBOOST {
    /// Boost the kick start of the oscillator
    ON = 0x1,

    /// Normal start of the oscillator
    OFF = 0x0,
}

/// Primary Oscillator Gain Control bits
pub enum POSCGAIN {
    /// Gain level 3 (Highest)
    GAIN_LEVEL_3 = 0x3,

    /// Gain level 2
    GAIN_LEVEL_2 = 0x2,

    /// Gain level 1
    GAIN_LEVEL_1 = 0x1,

    /// Gain level 0 (Lowest)
    GAIN_LEVEL_0 = 0x0,
}

/// Primary Oscillator Boost Kick Start Enable bit
pub enum POSCBOOST {
    /// Boost the kick start of the oscillator
    ON = 0x1,

    /// Normal start of the oscillator
    OFF = 0x0,
}

/// EJTAG Boot
pub enum EJTAGBEN {
    /// Normal EJTAG functionality
    NORMAL = 0x1,

    /// Reduced EJTAG functionality
    REDUCED = 0x0,
}

/// Code Protect
pub enum CP {
    /// Protection Enabled
    ON = 0x0,

    /// Protection Disabled
    OFF = 0x1,
}

/// Boot Flash True Sequence Number
type TSEQ = u16;

/// Boot Flash Complement Sequence Number
type CSEQ = u16;

/// Configuration word sector
#[repr(C)]
pub struct ConfigSector {
    DEVCFG3: u32,
    DEVCFG2: u32,
    DEVCFG1: u32,
    DEVCFG0: u32,
    DEVCP3: u32,
    DEVCP2: u32,
    DEVCP1: u32,
    DEVCP0: u32,
    DEVSIGN3: u32,
    DEVSIGN2: u32,
    DEVSIGN1: u32,
    DEVSIGN0: u32,
    SEQ3: u32,
    SEQ2: u32,
    SEQ1: u32,
    SEQ0: u32,
}

impl ConfigSector {
    /// Create a builder
    pub const fn default() -> ConfigSectorBuilder {
        ConfigSectorBuilder {
            DEVCFG3: 0xffffffff,
            DEVCFG2: 0xffffffff,
            DEVCFG1: 0xffffffff,
            DEVCFG0: 0xfffff7ff,
            DEVCP3: 0xffffffff,
            DEVCP2: 0xffffffff,
            DEVCP1: 0xffffffff,
            DEVCP0: 0xffffffff,
            DEVSIGN3: 0xffffffff,
            DEVSIGN2: 0xffffffff,
            DEVSIGN1: 0xffffffff,
            DEVSIGN0: 0x7fffffff,
            SEQ3: 0xffffffff,
            SEQ2: 0xffffffff,
            SEQ1: 0xffffffff,
            SEQ0: 0xffffffff,
        }
    }

    /// Convert into a array of 32 bit words consuming this ConfigSector
    pub const fn into_array(self) -> [u32; CONFIG_SECTOR_LENGTH] {
        [
            self.DEVCFG3,
            self.DEVCFG2,
            self.DEVCFG1,
            self.DEVCFG0,
            self.DEVCP3,
            self.DEVCP2,
            self.DEVCP1,
            self.DEVCP0,
            self.DEVSIGN3,
            self.DEVSIGN2,
            self.DEVSIGN1,
            self.DEVSIGN0,
            self.SEQ3,
            self.SEQ2,
            self.SEQ1,
            self.SEQ0,
        ]
    }
}
/// Configuration word sector builder
pub struct ConfigSectorBuilder {
    DEVCFG3: u32,
    DEVCFG2: u32,
    DEVCFG1: u32,
    DEVCFG0: u32,
    DEVCP3: u32,
    DEVCP2: u32,
    DEVCP1: u32,
    DEVCP0: u32,
    DEVSIGN3: u32,
    DEVSIGN2: u32,
    DEVSIGN1: u32,
    DEVSIGN0: u32,
    SEQ3: u32,
    SEQ2: u32,
    SEQ1: u32,
    SEQ0: u32,
}

impl ConfigSectorBuilder {
    pub const fn USERID(mut self, v: USERID) -> Self {
        self.DEVCFG3 &= !0x0000ffff;
        self.DEVCFG3 |= v as u32;
        self
    }

    pub const fn FMIIEN(mut self, v: FMIIEN) -> Self {
        self.DEVCFG3 &= !0x01000000;
        self.DEVCFG3 |= (v as u32) << 24;
        self
    }

    pub const fn FETHIO(mut self, v: FETHIO) -> Self {
        self.DEVCFG3 &= !0x02000000;
        self.DEVCFG3 |= (v as u32) << 25;
        self
    }

    pub const fn PGL1WAY(mut self, v: PGL1WAY) -> Self {
        self.DEVCFG3 &= !0x08000000;
        self.DEVCFG3 |= (v as u32) << 27;
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

    pub const fn FPLLIDIV(mut self, v: FPLLIDIV) -> Self {
        self.DEVCFG2 &= !0x00000007;
        self.DEVCFG2 |= v as u32;
        self
    }

    pub const fn FPLLRNG(mut self, v: FPLLRNG) -> Self {
        self.DEVCFG2 &= !0x00000070;
        self.DEVCFG2 |= (v as u32) << 4;
        self
    }

    pub const fn FPLLICLK(mut self, v: FPLLICLK) -> Self {
        self.DEVCFG2 &= !0x00000080;
        self.DEVCFG2 |= (v as u32) << 7;
        self
    }

    pub const fn FPLLMULT(mut self, v: FPLLMULT) -> Self {
        self.DEVCFG2 &= !0x00007f00;
        self.DEVCFG2 |= (v as u32) << 8;
        self
    }

    pub const fn FPLLODIV(mut self, v: FPLLODIV) -> Self {
        self.DEVCFG2 &= !0x00070000;
        self.DEVCFG2 |= (v as u32) << 16;
        self
    }

    pub const fn UPLLFSEL(mut self, v: UPLLFSEL) -> Self {
        self.DEVCFG2 &= !0x40000000;
        self.DEVCFG2 |= (v as u32) << 30;
        self
    }

    pub const fn FNOSC(mut self, v: FNOSC) -> Self {
        self.DEVCFG1 &= !0x00000007;
        self.DEVCFG1 |= v as u32;
        self
    }

    pub const fn DMTINTV(mut self, v: DMTINTV) -> Self {
        self.DEVCFG1 &= !0x00000038;
        self.DEVCFG1 |= (v as u32) << 3;
        self
    }

    pub const fn FSOSCEN(mut self, v: FSOSCEN) -> Self {
        self.DEVCFG1 &= !0x00000040;
        self.DEVCFG1 |= (v as u32) << 6;
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

    pub const fn WDTSPGM(mut self, v: WDTSPGM) -> Self {
        self.DEVCFG1 &= !0x00200000;
        self.DEVCFG1 |= (v as u32) << 21;
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

    pub const fn DMTCNT(mut self, v: DMTCNT) -> Self {
        self.DEVCFG1 &= !0x7c000000;
        self.DEVCFG1 |= (v as u32) << 26;
        self
    }

    pub const fn FDMTEN(mut self, v: FDMTEN) -> Self {
        self.DEVCFG1 &= !0x80000000;
        self.DEVCFG1 |= (v as u32) << 31;
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

    pub const fn TRCEN(mut self, v: TRCEN) -> Self {
        self.DEVCFG0 &= !0x00000020;
        self.DEVCFG0 |= (v as u32) << 5;
        self
    }

    pub const fn BOOTISA(mut self, v: BOOTISA) -> Self {
        self.DEVCFG0 &= !0x00000040;
        self.DEVCFG0 |= (v as u32) << 6;
        self
    }

    pub const fn FECCCON(mut self, v: FECCCON) -> Self {
        self.DEVCFG0 &= !0x00000300;
        self.DEVCFG0 |= (v as u32) << 8;
        self
    }

    pub const fn FSLEEP(mut self, v: FSLEEP) -> Self {
        self.DEVCFG0 &= !0x00000400;
        self.DEVCFG0 |= (v as u32) << 10;
        self
    }

    pub const fn DBGPER(mut self, v: DBGPER) -> Self {
        self.DEVCFG0 &= !0x00007000;
        self.DEVCFG0 |= (v as u32) << 12;
        self
    }

    pub const fn SMCLR(mut self, v: SMCLR) -> Self {
        self.DEVCFG0 &= !0x00008000;
        self.DEVCFG0 |= (v as u32) << 15;
        self
    }

    pub const fn SOSCGAIN(mut self, v: SOSCGAIN) -> Self {
        self.DEVCFG0 &= !0x00030000;
        self.DEVCFG0 |= (v as u32) << 16;
        self
    }

    pub const fn SOSCBOOST(mut self, v: SOSCBOOST) -> Self {
        self.DEVCFG0 &= !0x00040000;
        self.DEVCFG0 |= (v as u32) << 18;
        self
    }

    pub const fn POSCGAIN(mut self, v: POSCGAIN) -> Self {
        self.DEVCFG0 &= !0x00180000;
        self.DEVCFG0 |= (v as u32) << 19;
        self
    }

    pub const fn POSCBOOST(mut self, v: POSCBOOST) -> Self {
        self.DEVCFG0 &= !0x00200000;
        self.DEVCFG0 |= (v as u32) << 21;
        self
    }

    pub const fn EJTAGBEN(mut self, v: EJTAGBEN) -> Self {
        self.DEVCFG0 &= !0x40000000;
        self.DEVCFG0 |= (v as u32) << 30;
        self
    }

    pub const fn CP(mut self, v: CP) -> Self {
        self.DEVCP0 &= !0x10000000;
        self.DEVCP0 |= (v as u32) << 28;
        self
    }

    pub const fn TSEQ(mut self, v: TSEQ) -> Self {
        self.SEQ3 &= !0x0000ffff;
        self.SEQ3 |= v as u32;
        self
    }

    pub const fn CSEQ(mut self, v: CSEQ) -> Self {
        self.SEQ3 &= !0xffff0000;
        self.SEQ3 |= (v as u32) << 16;
        self
    }

    pub const fn build(self) -> ConfigSector {
        ConfigSector {
            DEVCFG3: self.DEVCFG3,
            DEVCFG2: self.DEVCFG2,
            DEVCFG1: self.DEVCFG1,
            DEVCFG0: self.DEVCFG0,
            DEVCP3: self.DEVCP3,
            DEVCP2: self.DEVCP2,
            DEVCP1: self.DEVCP1,
            DEVCP0: self.DEVCP0,
            DEVSIGN3: self.DEVSIGN3,
            DEVSIGN2: self.DEVSIGN2,
            DEVSIGN1: self.DEVSIGN1,
            DEVSIGN0: self.DEVSIGN0,
            SEQ3: self.SEQ3,
            SEQ2: self.SEQ2,
            SEQ1: self.SEQ1,
            SEQ0: self.SEQ0,
        }
    }
}
