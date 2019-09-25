use {hal, Config, Error, IntegrationTime, SlaveAddr, Veml6030};

struct Register;
impl Register {
    const ALS_CONF: u8 = 0x00;
}

struct BitFlags;
impl BitFlags {
    const ALS_SD: u16 = 0x01;
    const ALS_INT_EN: u16 = 0x02;
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
