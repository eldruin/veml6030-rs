use super::correction::{correct_high_lux, get_lux_raw_conversion_factor};
use crate::{
    calculate_raw_threshold_value, Config, Error, FaultCount, Gain, IntegrationTime,
    InterruptStatus, PowerSavingMode, SlaveAddr, Veml6030,
};

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c as AsyncI2c;

struct Register;
impl Register {
    const ALS_CONF: u8 = 0x00;
    const ALS_WH: u8 = 0x01;
    const ALS_WL: u8 = 0x02;
    const PSM: u8 = 0x03;
    const ALS: u8 = 0x04;
    const WHITE: u8 = 0x05;
    const ALS_INT: u8 = 0x06;
}

struct BitFlags;
impl BitFlags {
    const ALS_SD: u16 = 0x01;
    const ALS_INT_EN: u16 = 0x02;
    const PSM_EN: u16 = 0x01;
    const INT_TH_LOW: u16 = 1 << 15;
    const INT_TH_HIGH: u16 = 1 << 14;
}

impl Config {
    fn with_high(self, mask: u16) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u16) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
}

impl<I2C> Veml6030<I2C> {
    /// Create new instance of the VEML6030 device.
    pub fn new(i2c: I2C, address: SlaveAddr) -> Self {
        Veml6030 {
            i2c,
            address: address.addr(),
            config: Config {
                bits: BitFlags::ALS_SD,
            },
            gain: Gain::One,
            it: IntegrationTime::Ms100,
        }
    }

    /// Destroy driver instance, return I²C bus instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

#[maybe_async_cfg::maybe(
    sync(
        cfg(not(feature = "async")),
        self = "Veml6030",
        idents(AsyncI2c(sync = "I2c"))
    ),
    async(feature = "async", keep_self)
)]

impl<I2C, E> Veml6030<I2C>
where
    I2C: AsyncI2c<Error = E>,
{
    /// Enable the device.
    ///
    /// Note that when activating the sensor a wait time of 4 ms should be
    /// observed before the first measurement is picked up to allow for a
    /// correct start of the signal processor and oscillator.
    pub async fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_low(BitFlags::ALS_SD);
        self.set_config(config).await
    }

    /// Disable the device (shutdown).
    pub async fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BitFlags::ALS_SD);
        self.set_config(config).await
    }

    /// Set the integration time.
    pub async fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<E>> {
        let mask = match it {
            IntegrationTime::Ms25 => 0b1100,
            IntegrationTime::Ms50 => 0b1000,
            IntegrationTime::Ms100 => 0b0000,
            IntegrationTime::Ms200 => 0b0001,
            IntegrationTime::Ms400 => 0b0010,
            IntegrationTime::Ms800 => 0b0011,
        };
        let config = self.config.bits & !(0b1111 << 6) | (mask << 6);
        self.set_config(Config { bits: config }).await?;
        self.it = it;
        Ok(())
    }

    /// Set the gain.
    pub async fn set_gain(&mut self, gain: Gain) -> Result<(), Error<E>> {
        let mask = match gain {
            Gain::One => 0,
            Gain::Two => 1,
            Gain::OneEighth => 2,
            Gain::OneQuarter => 3,
        };
        let config = self.config.bits & !(0b11 << 11) | (mask << 11);
        self.set_config(Config { bits: config }).await?;
        self.gain = gain;
        Ok(())
    }

    /// Set the number of times a threshold crossing must happen consecutively
    /// to trigger an interrupt.
    pub async fn set_fault_count(&mut self, fc: FaultCount) -> Result<(), Error<E>> {
        let mask = match fc {
            FaultCount::One => 0,
            FaultCount::Two => 1,
            FaultCount::Four => 2,
            FaultCount::Eight => 3,
        };
        let config = self.config.bits & !(0b11 << 4) | (mask << 4);
        self.set_config(Config { bits: config }).await
    }

    /// Enable interrupt generation.
    pub async fn enable_interrupts(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BitFlags::ALS_INT_EN);
        self.set_config(config).await
    }

    /// Disable interrupt generation.
    pub async fn disable_interrupts(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_low(BitFlags::ALS_INT_EN);
        self.set_config(config).await
    }

    /// Set the ALS high threshold in raw format
    pub async fn set_high_threshold_raw(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_register(Register::ALS_WH, threshold).await
    }

    /// Set the ALS low threshold in raw format
    pub async fn set_low_threshold_raw(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_register(Register::ALS_WL, threshold).await
    }

    /// Set the ALS high threshold in lux.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the inverse of the compensation formula is applied (this involves
    /// quite some math).
    pub async fn set_high_threshold_lux(&mut self, lux: f32) -> Result<(), Error<E>> {
        let raw = self.calculate_raw_threshold_value(lux);
        self.write_register(Register::ALS_WH, raw).await
    }

    /// Set the ALS low threshold in lux.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the inverse of the compensation formula is applied (this involves
    /// quite some math).
    pub async fn set_low_threshold_lux(&mut self, lux: f32) -> Result<(), Error<E>> {
        let raw = self.calculate_raw_threshold_value(lux);
        self.write_register(Register::ALS_WL, raw).await
    }

    /// Calculate raw value for threshold applying compensation if necessary.
    ///
    /// This takes into consideration the configured integration time and gain
    /// and compensates the lux value if necessary.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain, the inverse of the
    /// compensation formula is applied. This involves quite some math so it
    /// may be interesting to calculate the threshold values ahead of time.
    pub fn calculate_raw_threshold_value(&self, lux: f32) -> u16 {
        calculate_raw_threshold_value(self.it, self.gain, lux)
    }

    /// Enable the power-saving mode
    pub async fn enable_power_saving(&mut self, psm: PowerSavingMode) -> Result<(), Error<E>> {
        let mask = match psm {
            PowerSavingMode::One => 0,
            PowerSavingMode::Two => 1,
            PowerSavingMode::Three => 2,
            PowerSavingMode::Four => 3,
        };
        let value = BitFlags::PSM_EN | (mask << 1);
        self.write_register(Register::PSM, value).await
    }

    /// Disable the power-saving mode
    pub async fn disable_power_saving(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::PSM, 0).await
    }

    async fn set_config(&mut self, config: Config) -> Result<(), Error<E>> {
        self.write_register(Register::ALS_CONF, config.bits).await?;
        self.config = config;
        Ok(())
    }

    async fn write_register(&mut self, register: u8, value: u16) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, value as u8, (value >> 8) as u8])
            .await
            .map_err(Error::I2C)
    }

    /// Read whether an interrupt has occurred.
    ///
    /// Note that the interrupt status is updated at the same rate as the
    /// measurements. Once triggered, flags will stay true until a measurement
    /// is taken which does not exceed the threshold.
    pub async fn read_interrupt_status(&mut self) -> Result<InterruptStatus, Error<E>> {
        let data = self.read_register(Register::ALS_INT).await?;
        Ok(InterruptStatus {
            was_too_low: (data & BitFlags::INT_TH_LOW) != 0,
            was_too_high: (data & BitFlags::INT_TH_HIGH) != 0,
        })
    }

    /// Read ALS high resolution output data in raw format
    pub async fn read_raw(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::ALS).await
    }

    /// Read ALS high resolution output data converted to lux
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the following compensation formula is applied:
    /// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
    pub async fn read_lux(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_register(Register::ALS).await?;
        Ok(self.convert_raw_als_to_lux(raw))
    }

    /// Calculate lux value for a raw ALS measurement.
    ///
    /// This takes into consideration the configured integration time and gain
    /// and compensates the lux value if necessary.
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the following compensation formula is applied:
    /// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
    pub fn convert_raw_als_to_lux(&self, raw_als: u16) -> f32 {
        convert_raw_als_to_lux(self.it, self.gain, raw_als)
    }

    /// Read white channel measurement
    pub async fn read_white(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::WHITE).await
    }

    async fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .await
            .map_err(Error::I2C)
            .and(Ok(u16::from(data[0]) | (u16::from(data[1]) << 8)))
    }
}

/// Calculate lux value for a raw ALS measurement.
///
/// For values higher than 1000 lx and 1/4 or 1/8 gain,
/// the following compensation formula is applied:
/// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
pub fn convert_raw_als_to_lux(it: IntegrationTime, gain: Gain, raw_als: u16) -> f32 {
    let factor = get_lux_raw_conversion_factor(it, gain);
    let lux = f64::from(raw_als) * f64::from(factor);
    if (gain == Gain::OneQuarter || gain == Gain::OneEighth) && lux > 1000.0 {
        correct_high_lux(lux) as f32
    } else {
        lux as f32
    }
}
