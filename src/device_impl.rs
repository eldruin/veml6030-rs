use {hal, Config, Error, SlaveAddr, Veml6030};

struct BitFlags;
impl BitFlags {
    const ALS_SD: u16 = 0x01;
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
