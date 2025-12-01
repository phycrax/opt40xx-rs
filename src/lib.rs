#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! ## Feature flags
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]

mod common;
mod error;
mod low_level;
mod sensor;

pub use {common::*, error::*, sensor::*};

#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    use crate::*;

    const DEFAULT_DEV_ADDR: u8 = 0x44;

    #[test]
    fn set_config_a() {
        let transaction = [Transaction::write(DEFAULT_DEV_ADDR, vec![0x0A, 0x31, 0xB8])];
        let mut i2c = Mock::new(transaction.iter());
        let mut sensor = Sensor::new(&mut i2c, Default::default());

        sensor
            .set_config_a(ConfigA {
                operating_mode: OperatingMode::Continuous,
                conv_time: ConversionTime::Ms25,
                ..Default::default()
            })
            .unwrap();

        i2c.done();
    }
}
