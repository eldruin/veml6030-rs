use hal::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use veml6030::{SlaveAddr, Veml6030};

pub const DEV_ADDR: u8 = 0x10;
pub const CFG_DEFAULT: u16 = BitFlags::ALS_SD;

pub struct Register;
impl Register {
    pub const ALS_CONF: u8 = 0x00;
    pub const ALS_WH: u8 = 0x01;
    pub const ALS_WL: u8 = 0x02;
    pub const PSM: u8 = 0x03;
    pub const ALS: u8 = 0x04;
    pub const WHITE: u8 = 0x05;
    pub const ALS_INT: u8 = 0x06;
}

pub struct BitFlags;
impl BitFlags {
    pub const ALS_SD: u16 = 0x01;
    pub const ALS_INT_EN: u16 = 0x02;
    pub const PSM_EN: u16 = 0x01;
    pub const INT_TH_LOW: u16 = 1 << 15;
    pub const INT_TH_HIGH: u16 = 1 << 14;
}

pub fn new(transactions: &[I2cTrans]) -> Veml6030<I2cMock> {
    Veml6030::new(I2cMock::new(&transactions), SlaveAddr::default())
}

pub fn destroy(sensor: Veml6030<I2cMock>) {
    sensor.destroy().done();
}
