use embedded_hal::i2c::{Error as I2cError, I2c};

use crate::{Error, common::*, low_level::LowLevel};

/// Sensor driver
pub struct Sensor<BUS: I2c> {
    ll: LowLevel<Interface<BUS>>,
}
impl<BUS: I2c> Sensor<BUS> {
    pub fn new(i2c: BUS, addr: Address) -> Self {
        Self {
            ll: LowLevel::new(Interface {
                i2c,
                dev_addr: addr.into(),
            }),
        }
    }

    /// Returns (raw value, measurement count)
    pub fn read_raw(
        &mut self,
        channel: Channel,
        burst: BurstRead,
    ) -> Result<(u32, u8), Error<BUS::Error>> {
        let ch_idx = channel as usize;

        let (exponent, mantissa, count, crc) = if let BurstRead::Enabled = burst {
            let measurement = self.ll.measurement(ch_idx).read()?;

            (
                measurement.exponent(),
                measurement.mantissa(),
                measurement.counter(),
                measurement.crc(),
            )
        } else {
            let measurement_high = self.ll.measurement_high(ch_idx).read()?;
            let measurement_low = self.ll.measurement_low(ch_idx).read()?;

            let mantissa = ((measurement_high.mantissa_h() as u32) << 8)
                + (measurement_low.mantissa_l() as u32);

            (
                measurement_high.exponent(),
                mantissa,
                measurement_low.counter(),
                measurement_low.crc(),
            )
        };

        if crc != calculate_crc(exponent, mantissa, count) {
            return Err(Error::Crc);
        }

        Ok((mantissa << exponent, count))
    }

    /// Sets configuration defined in register 0xB.
    pub fn set_config_a(&mut self, cfg: ConfigA) -> Result<(), Error<BUS::Error>> {
        self.ll.config_a().write(|reg| {
            reg.set_qwake(cfg.qwake);
            reg.set_range(cfg.range);
            reg.set_conv_time(cfg.conv_time);
            reg.set_operating_mode(cfg.operating_mode);
            reg.set_latch(cfg.latch);
            reg.set_int_pol(cfg.int_pol);
            reg.set_fault_count(cfg.fault_count);
        })?;

        Ok(())
    }

    /// Sets configuration defined in register 0xA.
    pub fn set_config_b(&mut self, cfg: ConfigB) -> Result<(), Error<BUS::Error>> {
        self.ll.config_b().write(|reg| {
            reg.set_threshold_ch(cfg.threshold_ch);
            reg.set_int_cfg(cfg.int_cfg);
            reg.set_int_dir(cfg.int_dir);
            reg.set_burst_read(cfg.burst_read);
        })?;

        Ok(())
    }

    /// Returns status
    pub fn read_status(&mut self) -> Result<Status, Error<BUS::Error>> {
        let status = self.ll.status().read()?;

        Ok(Status {
            overload: status.overload(),
            conv_ready: status.conv_ready(),
            flag_h: status.flag_h(),
            flag_l: status.flag_l(),
        })
    }

    /// Returns Device ID (L, H)
    pub fn read_device_id(&mut self) -> Result<(u8, u16), Error<BUS::Error>> {
        let id = self.ll.device_id().read()?;

        Ok((id.didl(), id.didh()))
    }
}

struct Interface<BUS: I2c> {
    i2c: BUS,
    dev_addr: u8,
}

impl<BUS> device_driver::RegisterInterface for Interface<BUS>
where
    BUS: I2c,
    BUS::Error: I2cError,
{
    type Error = Error<BUS::Error>;

    type AddressType = u8;

    fn write_register(
        &mut self,
        reg_addr: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let mut buf = [0u8; 5];

        if (1 + data.len()) > buf.len() {
            return Err(Error::BufferOverflow);
        }

        buf[0] = reg_addr;
        buf[1..1 + data.len()].copy_from_slice(data);

        self.i2c.write(self.dev_addr, &buf[..1 + data.len()])?;

        Ok(())
    }

    fn read_register(
        &mut self,
        reg_addr: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c.write_read(self.dev_addr, &[reg_addr], data)?;

        Ok(())
    }
}

// Gemini generated code, if you are unhappy with this, please send a PR
fn calculate_crc(exponent: u8, mantissa: u32, count: u8) -> u8 {
    let r = mantissa & 0xFFFFF; // 20 bits
    let e = (exponent & 0xF) as u32; // 4 bits
    let c = (count & 0xF) as u32; // 4 bits

    let bit = |v: u32, i: u32| (v >> i) & 1;

    // --- X[0] Calculation ---
    // Logic: XOR of all bits in E, R, and C.
    // Optimization: The XOR sum of bits is effectively the "Parity".
    // If the number of set bits is Odd, the XOR sum is 1. If Even, it's 0.
    let combined = r ^ e ^ c;

    let x0 = (combined.count_ones() % 2) as u8;

    // --- X[1] Calculation ---
    // Logic: XOR(C[1], C[3], R[odd], E[1], E[3])
    let mut x1 = bit(c, 1) ^ bit(c, 3) ^ bit(e, 1) ^ bit(e, 3);

    // XOR all odd bits of R: 1, 3, 5... 19
    for i in (1..=19).step_by(2) {
        x1 ^= bit(r, i);
    }
    let x1 = x1 as u8;

    // --- X[2] Calculation ---
    // Logic: XOR(C[3], R[3], R[7], R[11], R[15], R[19], E[3])
    let x2 = (bit(c, 3) ^ bit(r, 3) ^ bit(r, 7) ^ bit(r, 11) ^ bit(r, 15) ^ bit(r, 19) ^ bit(e, 3))
        as u8;

    // --- X[3] Calculation ---
    // Logic: XOR(R[3], R[11], R[19])
    let x3 = (bit(r, 3) ^ bit(r, 11) ^ bit(r, 19)) as u8;

    // Combine results into a single 4-bit value
    (x3 << 3) | (x2 << 2) | (x1 << 1) | x0
}
