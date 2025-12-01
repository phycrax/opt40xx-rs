//! Contains device-driver implementation and conversions

use crate::{common::*, error::ConversionError};

device_driver::create_device!(
    device_name: LowLevel,
    dsl: {
        config {
            type RegisterAddressType = u8;
            type DefaultByteOrder = BE;
            type DefmtFeature = "defmt";
        }

        register Measurement {
            type Access = RO;
            const ADDRESS = 0x00;
            const SIZE_BITS = 32;
            const REPEAT = {
                count: 4,
                stride: 2,
            };
            const ALLOW_ADDRESS_OVERLAP = true;

            exponent: uint = 28..32,
            mantissa: uint = 8..28,
            counter: uint = 4..8,
            crc: uint = 0..4,
        },

        register MeasurementHigh {
            type Access = RO;
            const ADDRESS = 0x00;
            const SIZE_BITS = 16;
            const REPEAT = {
                count: 4,
                stride: 2,
            };
            const ALLOW_ADDRESS_OVERLAP = true;

            exponent: uint = 12..16,
            mantissa_h: uint = 0..12,
        },

        register MeasurementLow {
            type Access = RO;
            const ADDRESS = 0x01;
            const SIZE_BITS = 16;
            const REPEAT = {
                count: 4,
                stride: 2,
            };

            mantissa_l: uint = 8..16,
            counter: uint = 4..8,
            crc: uint = 0..4,
        },

        register ThresholdLow {
            type Access = RW;
            const ADDRESS = 0x08;
            const SIZE_BITS = 16;

            exponent: uint = 12..16,
            result: uint = 4..8,
        },

        register ThresholdHigh {
            type Access = RW;
            const ADDRESS = 0x09;
            const SIZE_BITS = 16;
            const RESET_VALUE = 0xBFFF;

            exponent: uint = 12..16,
            result: uint = 4..8,
        },

        register ConfigA {
            type Access = RW;
            const ADDRESS = 0x0A;
            const SIZE_BITS = 16;
            const RESET_VALUE = 0x3208;

            qwake: uint as try QuickWake = 15..16,
            range: uint as try Range = 10..14,
            conv_time: uint as try ConversionTime = 6..10,
            operating_mode: uint as try OperatingMode = 4..6,
            latch: uint as try Latch = 3..4,
            int_pol: uint as try IntPolarity = 2..3,
            fault_count: uint as try FaultCount = 0..2,
        },

        register ConfigB {
            type Access = RW;
            const ADDRESS = 0x0B;
            const SIZE_BITS = 16;
            const RESET_VALUE = 0x8011;

            threshold_ch: uint as try Channel = 5..7,
            int_dir: uint as try IntDirection = 4..5,
            int_cfg: uint = 2..4,
            burst_read: uint as try BurstRead = 0..1,
        },

        register Status {
            type Access = RO;
            const ADDRESS = 0x0C;
            const SIZE_BITS = 16;

            overload: bool = 3,
            conv_ready: bool = 2,
            flag_h: bool = 1,
            flag_l: bool = 0,
        },

        register DeviceId {
            type Access = RO;
            const ADDRESS = 0x11;
            const SIZE_BITS = 16;

            didl: uint = 12..14,
            didh: uint = 0..12,
        },
    }
);

impl From<Address> for u8 {
    fn from(value: Address) -> Self {
        match value {
            Address::Gnd => 0b1000100,
            Address::Vdd => 0b1000101,
            Address::Sda => 0b1000110,
            Address::Scl => 0b1000111,
            Address::PicoStar => 0b1000101,
        }
    }
}

impl From<Range> for u8 {
    fn from(value: Range) -> Self {
        match value {
            Range::Manual(val) => val,
            Range::Auto => 12,
        }
    }
}
impl TryFrom<u8> for Range {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..12 => Ok(Range::Manual(value)),
            12 => Ok(Range::Auto),
            e => Err(ConversionError::Range(e)),
        }
    }
}

impl From<ConversionTime> for u8 {
    fn from(value: ConversionTime) -> Self {
        value as u8
    }
}
impl TryFrom<u8> for ConversionTime {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..12 => Ok(unsafe { core::mem::transmute::<u8, ConversionTime>(value) }),
            e => Err(ConversionError::ConversionTime(e)),
        }
    }
}

impl From<OperatingMode> for u8 {
    fn from(value: OperatingMode) -> Self {
        value as u8
    }
}
impl TryFrom<u8> for OperatingMode {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..4 => Ok(unsafe { core::mem::transmute::<u8, OperatingMode>(value) }),
            e => Err(ConversionError::OperatingMode(e)),
        }
    }
}

impl From<Latch> for u8 {
    fn from(value: Latch) -> Self {
        match value {
            Latch::TransparentHysteresis => 0,
            Latch::LatchedWindow => 1,
        }
    }
}
impl TryFrom<u8> for Latch {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TransparentHysteresis),
            1 => Ok(Self::LatchedWindow),
            e => Err(ConversionError::Latch(e)),
        }
    }
}

impl From<IntPolarity> for u8 {
    fn from(value: IntPolarity) -> Self {
        match value {
            IntPolarity::Low => 0,
            IntPolarity::High => 1,
        }
    }
}
impl TryFrom<u8> for IntPolarity {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Low),
            1 => Ok(Self::High),
            e => Err(ConversionError::IntPolarity(e)),
        }
    }
}

impl From<IntDirection> for u8 {
    fn from(value: IntDirection) -> Self {
        match value {
            IntDirection::Input => 0,
            IntDirection::Output => 1,
        }
    }
}
impl TryFrom<u8> for IntDirection {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Input),
            1 => Ok(Self::Output),
            e => Err(ConversionError::IntDirection(e)),
        }
    }
}

impl From<BurstRead> for u8 {
    fn from(value: BurstRead) -> Self {
        match value {
            BurstRead::Disabled => 0,
            BurstRead::Enabled => 1,
        }
    }
}
impl TryFrom<u8> for BurstRead {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::Enabled),
            e => Err(ConversionError::BurstRead(e)),
        }
    }
}

impl From<QuickWake> for u8 {
    fn from(value: QuickWake) -> Self {
        match value {
            QuickWake::Disabled => 0,
            QuickWake::Enabled => 1,
        }
    }
}
impl TryFrom<u8> for QuickWake {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::Enabled),
            e => Err(ConversionError::QuickWake(e)),
        }
    }
}

impl From<FaultCount> for u8 {
    fn from(value: FaultCount) -> Self {
        value as u8
    }
}
impl TryFrom<u8> for FaultCount {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..4 => Ok(unsafe { core::mem::transmute::<u8, FaultCount>(value) }),
            e => Err(ConversionError::FaultCount(e)),
        }
    }
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> Self {
        value as u8
    }
}
impl TryFrom<u8> for Channel {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..4 => Ok(unsafe { core::mem::transmute::<u8, Channel>(value) }),
            e => Err(ConversionError::Channel(e)),
        }
    }
}
