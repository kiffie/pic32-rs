//! 10-bit Analog-to-Digital Converter (ADC)

use crate::pac::ADC;
use core::marker::PhantomData;

/// Marker for unsigned 32-bit formats
pub struct Unsigned32;

/// Marker for signed 32-bit formats
pub struct Signed32;

/// Marker for unsigned 16-bit formats
pub struct Unsigned16;

/// Marker for signed 16-bit formats
pub struct Signed16;

/// Conversion trigger configuration
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ConversionTrigger {
    /// Internal counter ends sampling and starts conversion (auto convert).
    /// Sample time: u8_value * Tad, u8_value must be greater than 0 and less than 32.
    Auto(u8), // SSRC = 0b111

    /// CTMU ends sampling and starts conversion
    Cmtu, // SSRC = 0b011,

    /// Timer 3 period match ends sampling and starts conversion
    Timer3, // SSRC = 0b010,

    ///  Active transition on INT0 pin ends sampling and starts conversion
    Int0, // SSRC = 0b001,

    /// Clearing SAMP bit ends sampling and starts conversion (manual)
    Manual, // SSRC = 0b000,
}

/// Voltage reference configuration
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum VoltageReference {
    /// Vrefh = AVDD, Vrefl = AVSS
    AvddAvss = 0b000,
    /// Vrefh = external Vref+ pin, Vrefl = AVSS
    ExtAvss = 0b001,
    /// Vrefh = AVDD, Vrefl = external Vref- pin
    AvddExt = 0b010,
    /// Vrefh = external Vref+ pin, Vrefl = external Vref- pin
    ExtExt = 0b11,
}

/// ADC result buffer mode
#[derive(Clone, Copy, Debug)]
pub enum ResultBufferMode {
    /// Buffer configured as one 16-word buffer ADC1BUFF-ADC1BUF0
    SingleBuffer,
    /// Buffer configured as two 8-word buffers, ADC1BUF7-ADC1BUF0, ADC1BUFF-ADCBUF8
    DoubleBuffer,
}

/// ADC Conversion clock configuration
#[derive(Debug)]
pub enum ConversionClock {
    /// Half of the FRC clock frequency
    Frc,
    /// From peripheral bus clock: PB / (2 * u8_value + 1)
    Pb(u8),
}

/// Negative input selection
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum NegativeInput {
    /// Low ADC reference voltage Vrefl
    Vrefl = 0b0,
    /// Analog input AN1
    An1 = 0b1,
}

/// Input scan configuration
#[derive(Debug)]
pub enum InputScan {
    /// Scan mode disabled.
    /// The u8 value selects the analog input.
    Off(u8),

    /// Scan mode enabled.
    /// The u32 value is a bit mask for selecting the inputs to be scanned.
    On(u32),
}

/// ADC configuration
pub struct AdcConfiguration {
    conversion_trigger: ConversionTrigger,
    auto_sample: bool,
    voltage_reference: VoltageReference,
    offset_calibration: bool,
    conversions_per_irq: u8,
    alt_sample_mode: bool,
    result_buffer_mode: ResultBufferMode,
    conversion_clock: ConversionClock,
}

impl Default for AdcConfiguration {
    /// Create a configuration for manually sampling a single channel
    fn default() -> Self {
        AdcConfiguration {
            conversion_trigger: ConversionTrigger::Auto(31),
            auto_sample: false,
            voltage_reference: VoltageReference::AvddAvss,
            offset_calibration: false,
            conversions_per_irq: 1,
            alt_sample_mode: false,
            result_buffer_mode: ResultBufferMode::SingleBuffer,
            conversion_clock: ConversionClock::Frc,
        }
    }
}

impl AdcConfiguration {
    /// Set the conversion trigger. Panics if the a a value outside 1..., 31 is indicated
    /// for auto conversion
    pub fn conversion_trigger(&mut self, conversion_trigger: ConversionTrigger) -> &mut Self {
        if let ConversionTrigger::Auto(delay) = conversion_trigger {
            assert!(delay > 0 && delay <= 31);
        }
        self.conversion_trigger = conversion_trigger;
        self
    }

    /// Activate or deactivate automatic sampling
    pub fn auto_sample(&mut self, auto_sample: bool) -> &mut Self {
        self.auto_sample = auto_sample;
        self
    }
    /// Configure the voltage reference sources
    pub fn voltage_reference(&mut self, voltage_reference: VoltageReference) -> &mut Self {
        self.voltage_reference = voltage_reference;
        self
    }

    /// Activate or deactivate the offset calibration mode
    pub fn offset_calibration(&mut self, offset_calibration: bool) -> &mut Self {
        self.offset_calibration = offset_calibration;
        self
    }

    /// Set the number of conversions per IRQ. The value must be between 1 and 16; otherwise,
    /// this method will panic.
    pub fn conversions_per_irq(&mut self, conversions_per_irq: u8) -> &mut Self {
        assert!(conversions_per_irq > 0 && conversions_per_irq <= 16);
        self.conversions_per_irq = conversions_per_irq;
        self
    }

    /// Activate or deactivate the Alternate Sample Mode
    pub fn alt_sample_mode(&mut self, alt_sample_mode: bool) -> &mut Self {
        self.alt_sample_mode = alt_sample_mode;
        self
    }

    /// Set result buffer mode
    pub fn result_buffer_mode(&mut self, result_buffer_mode: ResultBufferMode) -> &mut Self {
        self.result_buffer_mode = result_buffer_mode;
        self
    }

    /// Set the conversion clock
    pub fn conversion_clock(&mut self, conversion_clock: ConversionClock) -> &mut Self {
        self.conversion_clock = conversion_clock;
        self
    }
}

pub struct Adc<F> {
    adc: ADC,
    _format: PhantomData<F>,
}

impl Adc<Unsigned32> {
    /// Create an Adc instance (unsigned 32-bit data format).
    /// `fractional == true` selects a fractional number format.
    pub fn new_u32(adc: ADC, fractional: bool) -> Self {
        let mut adc = Adc {
            adc,
            _format: PhantomData,
        };
        adc.init(0b100, fractional);
        adc
    }

    /// Read from the ADC buffer.
    /// `index` must be less than or equal to 15.
    pub fn read(&self, index: usize) -> u32 {
        let regs = unsafe { core::slice::from_raw_parts(self.adc.buf0.as_ptr(), 16 * 4) };
        regs[4 * index]
    }
}

impl Adc<Signed32> {
    /// Create an Adc instance (signed 32-bit data format).
    /// `fractional == true` selects a fractional number format.
    pub fn new_i32(adc: ADC, fractional: bool) -> Self {
        let mut adc = Adc {
            adc,
            _format: PhantomData,
        };
        adc.init(0b101, fractional);
        adc
    }

    /// Read from the ADC buffer.
    /// `index` must be less than or equal to 15.
    pub fn read(&self, index: usize) -> i32 {
        let regs =
            unsafe { core::slice::from_raw_parts(self.adc.buf0.as_ptr() as *const i32, 16 * 4) };
        regs[4 * index]
    }
}

impl Adc<Unsigned16> {
    /// Create an Adc instance (unsigned 16-bit data format).
    /// `fractional == true` selects a fractional number format.
    pub fn new_u16(adc: ADC, fractional: bool) -> Self {
        let mut adc = Adc {
            adc,
            _format: PhantomData,
        };
        adc.init(0b000, fractional);
        adc
    }

    /// Read from the ADC buffer.
    /// `index` must be less than or equal to 15.
    pub fn read(&self, index: usize) -> u16 {
        let regs =
            unsafe { core::slice::from_raw_parts(self.adc.buf0.as_ptr() as *const u16, 16 * 8) };
        regs[8 * index]
    }
}

impl Adc<Signed16> {
    /// Create an Adc instance (signed 16-bit data format).
    /// `fractional == true` selects a fractional number format.
    pub fn new_i16(adc: ADC, fractional: bool) -> Self {
        let mut adc = Adc {
            adc,
            _format: PhantomData,
        };
        adc.init(0b001, fractional);
        adc
    }

    /// Read from the ADC buffer.
    /// `index` must be less than or equal to 15.
    pub fn read(&self, index: usize) -> i16 {
        let regs =
            unsafe { core::slice::from_raw_parts(self.adc.buf0.as_ptr() as *const i16, 16 * 8) };
        regs[8 * index]
    }
}

impl<F> Adc<F> {
    /// Set format and turn off
    fn init(&mut self, format: u8, fractional: bool) {
        let form = if fractional { format | 0b010 } else { format };
        self.adc
            .con1
            .modify(|_, w| unsafe { w.on().clear_bit().form().bits(form) });
    }

    /// configure and activate the ADC
    pub fn configure(&mut self, config: &AdcConfiguration) {
        let (ssrc, samc) = match config.conversion_trigger {
            ConversionTrigger::Auto(samc) => (0b111, samc),
            ConversionTrigger::Cmtu => (0b011, 0),
            ConversionTrigger::Timer3 => (0b010, 0),
            ConversionTrigger::Int0 => (0b001, 0),
            ConversionTrigger::Manual => (0b000, 0),
        };
        unsafe {
            self.adc.con1clr.write_with_zero(|w| w.on().bit(true));
        }
        self.adc
            .con1
            .modify(|_, w| unsafe { w.ssrc().bits(ssrc).asam().bit(config.auto_sample) });
        self.adc.con2.modify(|_, w| unsafe {
            w.vcfg()
                .bits(config.voltage_reference as u8)
                .offcal()
                .bit(config.offset_calibration)
                .bufm()
                .bit(config.result_buffer_mode as u8 != 0)
                .smpi()
                .bits(config.conversions_per_irq)
                .alts()
                .bit(config.alt_sample_mode)
        });
        let (adrc, adcs) = match config.conversion_clock {
            ConversionClock::Pb(adcs) => (false, adcs),
            ConversionClock::Frc => (true, 0),
        };
        self.adc
            .con3
            .modify(|_, w| unsafe { w.adrc().bit(adrc).samc().bits(samc).adcs().bits(adcs) });
        unsafe {
            self.adc.con1set.write_with_zero(|w| w.on().set_bit());
        }
    }

    /// Select positive input for sample A
    pub fn select_pos_input(&mut self, input: InputScan) {
        match input {
            InputScan::Off(channel) => {
                self.adc
                    .chs
                    .modify(|_, w| unsafe { w.ch0sa().bits(channel) });
                unsafe {
                    self.adc.con2clr.write_with_zero(|w| w.cscna().bit(true));
                }
            }
            InputScan::On(mask) => {
                self.adc.cssl.write(|w| unsafe { w.bits(mask) });
                unsafe {
                    self.adc.con2set.write_with_zero(|w| w.cscna().bit(true));
                }
            }
        }
    }

    /// Select negative input for sample A
    pub fn select_neg_input(&mut self, input: NegativeInput) {
        self.adc.chs.modify(|_, w| w.ch0na().bit(input as u8 != 0));
    }

    /// Select positive input for sample B (alternate sample)
    pub fn select_pos_alt_input(&mut self, input: u8) {
        self.adc.chs.modify(|_, w| unsafe { w.ch0sb().bits(input) });
    }

    /// Select negative input for sample B (alternate sample)
    pub fn select_neg_alt_input(&mut self, input: NegativeInput) {
        self.adc.chs.modify(|_, w| w.ch0nb().bit(input as u8 != 0));
    }

    /// Manually start a sampling period.
    /// Use only when auto_sample is not active.
    pub fn start_sampling(&mut self) {
        self.adc
            .con1
            .modify(|_, w| w.samp().set_bit().done().clear_bit());
    }

    /// Manually start a conversion period.
    /// Use only when the ConversionTrigger is Manual.
    pub fn start_conversion(&mut self) {
        unsafe {
            self.adc.con1clr.write_with_zero(|w| w.samp().set_bit());
        }
    }

    /// Check if conversion is complete.
    /// To be used when sampling is stated manually.
    pub fn done(&self) -> bool {
        self.adc.con1.read().done().bit()
    }

    /// return the ADC consuming the Adc instance
    pub fn free(self) -> ADC {
        // turn ADC off
        self.adc.con1clr.write(|w| w.on().bit(true));
        self.adc
    }
}
