use embedded_hal::i2c::Error as I2cError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// All possible errors
pub enum Error<E: I2cError> {
    /// I2C error
    I2c(E),
    /// Conversion error
    Conversion(ConversionError),
    /// Buffer overflow
    BufferOverflow,
    /// CRC error
    Crc,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Driver data conversion error
pub enum ConversionError {
    /// Conversion time error
    ConversionTime(u8),
    /// Operating mode error
    OperatingMode(u8),
    /// Range error
    Range(u8),
    /// Fault count error
    FaultCount(u8),
    /// Latch error
    Latch(u8),
    /// Interrupt polarity error
    IntPolarity(u8),
    /// Interrupt direction error
    IntDirection(u8),
    /// Channel error
    Channel(u8),
    /// Burst read error
    BurstRead(u8),
    /// Quick wake error
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
