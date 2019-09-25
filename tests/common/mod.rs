use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use veml6030::{SlaveAddr, Veml6030};

pub const DEV_ADDR: u8 = 0x10;

pub struct Register;

pub struct BitFlags;

pub fn new(transactions: &[I2cTrans]) -> Veml6030<I2cMock> {
    Veml6030::new(I2cMock::new(&transactions), SlaveAddr::default())
}

pub fn destroy(sensor: Veml6030<I2cMock>) {
    sensor.destroy().done();
}
