use embedded_hal::i2c::Error as I2cError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// All possible errors
pub enum Error<E: I2cError> {
    I2c(E),
    Conversion(ConversionError),
    BufferOverflow,
    Crc,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Driver data conversion error
pub enum ConversionError {
    ConversionTime(u8),
    OperatingMode(u8),
    Range(u8),
    FaultCount(u8),
    Latch(u8),
    IntPolarity(u8),
    IntDirection(u8),
    Channel(u8),
    BurstRead(u8),
    QuickWake(u8),
}

impl<E: I2cError> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::I2c(e)
    }
}

impl<E: I2cError> From<ConversionError> for Error<E> {
    fn from(e: ConversionError) -> Self {
        Self::Conversion(e)
    }
}
