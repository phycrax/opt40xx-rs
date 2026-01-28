#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Configuration settings for register A.
///
/// Contains main configuration options including measurement range,
/// conversion time, and interrupt behavior.
pub struct ConfigA {
    /// Quick Wake mode.
    pub qwake: QuickWake,
    /// Measurement range setting for light detection.
    pub range: Range,
    /// Conversion time.
    pub conv_time: ConversionTime,
    /// Operating mode.
    pub operating_mode: OperatingMode,
    /// Interrupt latch mode.
    pub latch: Latch,
    /// Interrupt polarity.
    pub int_pol: IntPolarity,
    /// Fault count for interrupt triggering.
    pub fault_count: FaultCount,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Configuration settings for register B.
///
/// Contains secondary configuration options including threshold management,
/// interrupt configuration, and i2c burst read mode.
pub struct ConfigB {
    /// Channel to be used with threshold detection
    ///
    /// Sensor IC dependent, refer to datasheet
    pub threshold_ch: Channel,

    /// Interrupt Pin Direction
    pub int_dir: IntDirection,

    /// Interrupt Mechanism
    ///
    /// Sensor IC dependent, refer to datasheet
    pub int_cfg: u8,

    /// I2C Burst Read mode
    pub burst_read: BurstRead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Status register
pub struct Status {
    /// Overload flag.
    pub overload: bool,
    /// Conversion ready flag.
    pub conv_ready: bool,
    /// High threshold flag.
    pub flag_h: bool,
    /// Low threshold flag.
    pub flag_l: bool,
}

/// Possible device addresses
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Address {
    #[default]
    /// GND
    Gnd,
    /// VDD
    Vdd,
    /// SDA
    Sda,
    /// SCL
    Scl,
    /// PicoStar
    PicoStar,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Input channel
pub enum Channel {
    /// (default)
    #[default]
    Ch0,
    /// Channel 1
    Ch1,
    /// Channel 2
    Ch2,
    /// Channel 3
    Ch3,
}

/// Light range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Range {
    /// Manual
    Manual(u8),
    /// Automatic (default)
    #[default]
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Conversion time
pub enum ConversionTime {
    /// 600 μs
    Us600,
    /// 1 ms
    Ms1,
    /// 1.8 ms
    Ms1_8,
    /// 3.4 ms
    Ms3_4,
    /// 6.5 ms
    Ms6_5,
    /// 12.7 ms
    Ms12_7,
    /// 25 ms
    Ms25,
    /// 50 ms
    Ms50,
    /// 100 ms (default)
    #[default]
    Ms100,
    /// 200 ms
    Ms200,
    /// 400 ms
    Ms400,
    /// 800 ms
    Ms800,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Operating mode
pub enum OperatingMode {
    /// (default)
    #[default]
    PowerDown,
    /// Forced one-shot mode
    ForcedOneShot,
    /// Regular one-shot mode
    RegularOneShot,
    /// Continuous mode
    Continuous,
}

/// Interrupt reporting mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latch {
    /// Transparent hysteresis mode
    TransparentHysteresis,
    /// Latched window mode (default)
    #[default]
    LatchedWindow,
}

/// Interrupt pin polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntPolarity {
    /// Active low (default)
    #[default]
    Low,
    /// Active high
    High,
}

/// Interrupt pin direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntDirection {
    /// Input
    Input,
    /// (default)
    #[default]
    Output,
}

/// I2C Burst Read mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstRead {
    /// Disabled
    Disabled,
    /// (default)
    #[default]
    /// Enabled
    Enabled,
}

/// Quick Wake
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QuickWake {
    /// (default)
    #[default]
    /// Disabled
    Disabled,
    /// Enabled
    Enabled,
}

/// Fault count
///
/// Number of consecutive fault events required to trigger the
/// threshold mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FaultCount {
    /// One (default)
    #[default]
    One,
    /// Two
    Two,
    /// Four
    Four,
    /// Eight
    Eight,
}
