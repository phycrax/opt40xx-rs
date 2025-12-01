#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Configuration settings for register A.
///
/// Contains main configuration options including measurement range,
/// conversion time, and interrupt behavior.
pub struct ConfigA {
    pub qwake: QuickWake,
    /// Measurement range setting for light detection.
    pub range: Range,
    pub conv_time: ConversionTime,
    pub operating_mode: OperatingMode,
    pub latch: Latch,
    pub int_pol: IntPolarity,
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
pub struct Status {
    pub overload: bool,
    pub conv_ready: bool,
    pub flag_h: bool,
    pub flag_l: bool,
}

/// Possible device addresses
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Address {
    #[default]
    Gnd,
    Vdd,
    Sda,
    Scl,
    PicoStar,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Channel {
    /// (default)
    #[default]
    Ch0,
    Ch1,
    Ch2,
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
    ForcedOneShot,
    RegularOneShot,
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
    Input,
    /// (default)
    #[default]
    Output,
}

/// I2C Burst Read mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstRead {
    Disabled,
    /// (default)
    #[default]
    Enabled,
}

/// Quick Wake
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum QuickWake {
    /// (default)
    #[default]
    Disabled,
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
