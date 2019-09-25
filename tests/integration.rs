extern crate embedded_hal_mock as hal;
extern crate veml6030;
use hal::i2c::Transaction as I2cTrans;
use veml6030::{Gain, IntegrationTime as IT};

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

cfg_test!(
    set_it_25,
    set_integration_time,
    CFG_DEFAULT | (0b1100 << 6),
    IT::Ms25
);
cfg_test!(
    set_it_50,
    set_integration_time,
    CFG_DEFAULT | (0b1000 << 6),
    IT::Ms50
);
cfg_test!(set_it_100, set_integration_time, CFG_DEFAULT, IT::Ms100);
cfg_test!(
    set_it_200,
    set_integration_time,
    CFG_DEFAULT | (0b0001 << 6),
    IT::Ms200
);
cfg_test!(
    set_it_400,
    set_integration_time,
    CFG_DEFAULT | (0b0010 << 6),
    IT::Ms400
);
cfg_test!(
    set_it_800,
    set_integration_time,
    CFG_DEFAULT | (0b0011 << 6),
    IT::Ms800
);

cfg_test!(set_gain_1, set_gain, CFG_DEFAULT, Gain::One);
cfg_test!(set_gain_2, set_gain, CFG_DEFAULT | 1 << 11, Gain::Two);
cfg_test!(
    set_gain_one_eighth,
    set_gain,
    CFG_DEFAULT | 2 << 11,
    Gain::OneEighth
);
cfg_test!(
    set_gain_one_quarter,
    set_gain,
    CFG_DEFAULT | 3 << 11,
    Gain::OneQuarter
);
