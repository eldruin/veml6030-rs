use {
    hal, Config, Error, FaultCount, Gain, IntegrationTime, InterruptStatus, PowerSavingMode,
    SlaveAddr, Veml6030,
};

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

impl<I2C, E> Veml6030<I2C>
where
    I2C: hal::blocking::i2c::Write<Error = E>,
{
    /// Enable the device.
    ///
    /// Note that when activating the sensor a wait time of 4 ms should be
    /// observed before the first measurement is picked up to allow for a
    /// correct start of the signal processor and oscillator.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_low(BitFlags::ALS_SD);
        self.set_config(config)
    }

    /// Disable the device (shutdown).
    pub fn disable(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BitFlags::ALS_SD);
        self.set_config(config)
    }

    /// Set the integration time.
    pub fn set_integration_time(&mut self, it: IntegrationTime) -> Result<(), Error<E>> {
        let mask = match it {
            IntegrationTime::Ms25 => 0b1100,
            IntegrationTime::Ms50 => 0b1000,
            IntegrationTime::Ms100 => 0b0000,
            IntegrationTime::Ms200 => 0b0001,
            IntegrationTime::Ms400 => 0b0010,
            IntegrationTime::Ms800 => 0b0011,
        };
        let config = self.config.bits & !(0b1111 << 6) | (mask << 6);
        self.set_config(Config { bits: config })?;
        self.it = it;
        Ok(())
    }

    /// Set the gain.
    pub fn set_gain(&mut self, gain: Gain) -> Result<(), Error<E>> {
        let mask = match gain {
            Gain::One => 0,
            Gain::Two => 1,
            Gain::OneEighth => 2,
            Gain::OneQuarter => 3,
        };
        let config = self.config.bits & !(0b11 << 11) | mask << 11;
        self.set_config(Config { bits: config })?;
        self.gain = gain;
        Ok(())
    }

    /// Set the number of times a threshold crossing must happen consecutively
    /// to trigger an interrupt.
    pub fn set_fault_count(&mut self, fc: FaultCount) -> Result<(), Error<E>> {
        let mask = match fc {
            FaultCount::One => 0,
            FaultCount::Two => 1,
            FaultCount::Four => 2,
            FaultCount::Eight => 3,
        };
        let config = self.config.bits & !(0b11 << 4) | mask << 4;
        self.set_config(Config { bits: config })
    }

    /// Enable interrupt generation.
    pub fn enable_interrupts(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_high(BitFlags::ALS_INT_EN);
        self.set_config(config)
    }

    /// Disable interrupt generation.
    pub fn disable_interrupts(&mut self) -> Result<(), Error<E>> {
        let config = self.config.with_low(BitFlags::ALS_INT_EN);
        self.set_config(config)
    }

    /// Set the ALS high threshold in raw format
    pub fn set_high_threshold_raw(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_register(Register::ALS_WH, threshold)
    }

    /// Set the ALS low threshold in raw format
    pub fn set_low_threshold_raw(&mut self, threshold: u16) -> Result<(), Error<E>> {
        self.write_register(Register::ALS_WL, threshold)
    }

    /// Enable the power-saving mode
    pub fn enable_power_saving(&mut self, psm: PowerSavingMode) -> Result<(), Error<E>> {
        let mask = match psm {
            PowerSavingMode::One => 0,
            PowerSavingMode::Two => 1,
            PowerSavingMode::Three => 2,
            PowerSavingMode::Four => 3,
        };
        let value = BitFlags::PSM_EN | mask << 1;
        self.write_register(Register::PSM, value)
    }

    /// Disable the power-saving mode
    pub fn disable_power_saving(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::PSM, 0)
    }

    fn set_config(&mut self, config: Config) -> Result<(), Error<E>> {
        self.write_register(Register::ALS_CONF, config.bits)?;
        self.config = config;
        Ok(())
    }

    fn write_register(&mut self, register: u8, value: u16) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, value as u8, (value >> 8) as u8])
            .map_err(Error::I2C)
    }
}

impl<I2C, E> Veml6030<I2C>
where
    I2C: hal::blocking::i2c::WriteRead<Error = E>,
{
    /// Read whether an interrupt has occurred.
    ///
    /// Note that the interrupt status is updated at the same rate as the
    /// measurements. Once triggered, flags will stay true until a measurement
    /// is taken which does not exceed the threshold.
    pub fn read_interrupt_status(&mut self) -> Result<InterruptStatus, Error<E>> {
        let data = self.read_register(Register::ALS_INT)?;
        Ok(InterruptStatus {
            was_too_low: (data & BitFlags::INT_TH_LOW) != 0,
            was_too_high: (data & BitFlags::INT_TH_HIGH) != 0,
        })
    }

    /// Read ALS high resolution output data in raw format
    pub fn read_raw(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::ALS)
    }

    /// Read ALS high resolution output data converted to lux
    ///
    /// For values higher than 1000 lx and 1/4 or 1/8 gain,
    /// the following compensation formula is applied:
    /// `lux = 6.0135e-13*(lux^4) - 9.3924e-9*(lux^3) + 8.1488e-5*(lux^2) + 1.0023*lux`
    pub fn read_lux(&mut self) -> Result<f32, Error<E>> {
        let raw = self.read_register(Register::ALS)?;
        let gain_factor = match self.gain {
            Gain::Two => 1.0,
            Gain::One => 2.0,
            Gain::OneQuarter => 8.0,
            Gain::OneEighth => 16.0,
        };
        let it_factor = match self.it {
            IntegrationTime::Ms800 => 0.0036,
            IntegrationTime::Ms400 => 0.0072,
            IntegrationTime::Ms200 => 0.0144,
            IntegrationTime::Ms100 => 0.0288,
            IntegrationTime::Ms50 => 0.0576,
            IntegrationTime::Ms25 => 0.1152,
        };
        let lux = f64::from(raw) * it_factor * gain_factor;
        if (self.gain == Gain::OneQuarter || self.gain == Gain::OneEighth) && lux > 1000.0 {
            Ok(correct_high_lux(lux) as f32)
        } else {
            Ok(lux as f32)
        }
    }

    /// Read white channel measurement
    pub fn read_white(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::WHITE)
    }

    fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(u16::from(data[0]) | u16::from(data[1]) << 8))
    }
}

fn correct_high_lux(lux: f64) -> f64 {
    const C0: f64 = 1.0023;
    const C1: f64 = 8.1488e-05;
    const C2: f64 = 9.3924e-09;
    const C3: f64 = 6.0135e-13;
    lux * lux * lux * lux * C3 - lux * lux * lux * C2 + lux * lux * C1 + lux * C0
}
