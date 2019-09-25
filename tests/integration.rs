extern crate embedded_hal_mock as hal;
extern crate veml6030;
use hal::i2c::Transaction as I2cTrans;

mod common;
use common::{destroy, new, BitFlags as BF, Register as Reg, DEV_ADDR};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}
