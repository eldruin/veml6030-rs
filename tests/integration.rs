extern crate embedded_hal_mock as hal;
extern crate veml6030;
use hal::i2c::Transaction as I2cTrans;

mod common;
use common::{destroy, new, BitFlags as BF, Register as Reg, CFG_DEFAULT, DEV_ADDR};

#[test]
fn can_create_and_destroy() {
    let sensor = new(&[]);
    destroy(sensor);
}

macro_rules! set_test {
    ($name:ident, $method:ident, $register:ident, $value:expr $(, $arg:expr)*) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write(
                DEV_ADDR,
                vec![Reg::$register, $value as u8, ($value >> 8) as u8],
            )];
            let mut sensor = new(&transactions);
            sensor.$method($($arg),*).unwrap();
            destroy(sensor);
        }
    };
}

macro_rules! cfg_test {
    ($name:ident, $method:ident, $value:expr $(, $arg:expr)*) => {
        set_test!($name, $method, ALS_CONF, $value $(, $arg)*);
    };
}

cfg_test!(enable, enable, 0);
cfg_test!(disable, disable, BF::ALS_SD);
cfg_test!(enable_int, enable_interrupts, CFG_DEFAULT | BF::ALS_INT_EN);
cfg_test!(disable_int, disable_interrupts, CFG_DEFAULT);
