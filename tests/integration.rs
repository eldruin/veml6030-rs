use embedded_hal_mock::i2c::Transaction as I2cTrans;
use veml6030::{
    FaultCount as FC, Gain, IntegrationTime as IT, InterruptStatus, PowerSavingMode as PSM,
};

mod common;
use crate::common::{destroy, new, BitFlags as BF, Register as Reg, CFG_DEFAULT, DEV_ADDR};

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
cfg_test!(set_gain_2, set_gain, CFG_DEFAULT | (1 << 11), Gain::Two);
cfg_test!(
    set_gain_one_eighth,
    set_gain,
    CFG_DEFAULT | (2 << 11),
    Gain::OneEighth
);
cfg_test!(
    set_gain_one_quarter,
    set_gain,
    CFG_DEFAULT | (3 << 11),
    Gain::OneQuarter
);

cfg_test!(set_fc_1, set_fault_count, CFG_DEFAULT, FC::One);
cfg_test!(set_fc_2, set_fault_count, CFG_DEFAULT | (1 << 4), FC::Two);
cfg_test!(set_fc_4, set_fault_count, CFG_DEFAULT | (2 << 4), FC::Four);
cfg_test!(set_fc_8, set_fault_count, CFG_DEFAULT | (3 << 4), FC::Eight);

set_test!(enable_psm_1, enable_power_saving, PSM, BF::PSM_EN, PSM::One);
set_test!(
    enable_psm_2,
    enable_power_saving,
    PSM,
    BF::PSM_EN | (1 << 1),
    PSM::Two
);
set_test!(
    enable_psm_3,
    enable_power_saving,
    PSM,
    BF::PSM_EN | (2 << 1),
    PSM::Three
);
set_test!(
    enable_psm_4,
    enable_power_saving,
    PSM,
    BF::PSM_EN | (3 << 1),
    PSM::Four
);
set_test!(disable_psm, disable_power_saving, PSM, 0);

macro_rules! get_test {
    ($name:ident, $method:ident, $register:ident, $value:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [I2cTrans::write_read(
                DEV_ADDR,
                vec![Reg::$register],
                vec![$value as u8, ($value >> 8) as u8],
            )];
            let mut sensor = new(&transactions);
            let result = sensor.$method().unwrap();
            assert_eq!($expected, result);
            destroy(sensor);
        }
    };
}

get_test!(
    int_status_none,
    read_interrupt_status,
    ALS_INT,
    0,
    InterruptStatus {
        was_too_low: false,
        was_too_high: false
    }
);
get_test!(
    int_status_too_low,
    read_interrupt_status,
    ALS_INT,
    BF::INT_TH_LOW,
    InterruptStatus {
        was_too_low: true,
        was_too_high: false
    }
);
get_test!(
    int_status_too_high,
    read_interrupt_status,
    ALS_INT,
    BF::INT_TH_HIGH,
    InterruptStatus {
        was_too_low: false,
        was_too_high: true
    }
);
get_test!(
    int_status_both,
    read_interrupt_status,
    ALS_INT,
    BF::INT_TH_HIGH | BF::INT_TH_LOW,
    InterruptStatus {
        was_too_low: true,
        was_too_high: true
    }
);
get_test!(read_als_raw, read_raw, ALS, 0xABCD_u16, 0xABCD);
get_test!(read_white, read_white, WHITE, 0xABCD_u16, 0xABCD);

macro_rules! read_lux_test {
    ($name:ident, $it:ident, $gain:ident, $config1:expr, $config2:expr, $als:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let transactions = [
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Reg::ALS_CONF, $config1 as u8, ($config1 >> 8) as u8],
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Reg::ALS_CONF, $config2 as u8, ($config2 >> 8) as u8],
                ),
                I2cTrans::write_read(
                    DEV_ADDR,
                    vec![Reg::ALS],
                    vec![$als as u8, ($als >> 8) as u8],
                ),
            ];
            let mut sensor = new(&transactions);
            sensor.set_integration_time(IT::$it).unwrap();
            sensor.set_gain(Gain::$gain).unwrap();
            let result = sensor.read_lux().unwrap();
            assert!($expected - 0.5 < result);
            assert!($expected + 0.5 > result);
            destroy(sensor);
        }
    };
}

read_lux_test!(
    it100g1,
    Ms100,
    One,
    CFG_DEFAULT,
    CFG_DEFAULT,
    1480_u16,
    85.248
);
read_lux_test!(
    it100g14,
    Ms100,
    OneQuarter,
    CFG_DEFAULT,
    CFG_DEFAULT | (3 << 11),
    1480_u16,
    341.0
);
read_lux_test!(
    min,
    Ms800,
    Two,
    CFG_DEFAULT | (0b0011 << 6),
    CFG_DEFAULT | (0b0011 << 6) | (1 << 11),
    1480_u16,
    5.328
);
read_lux_test!(
    max_applies_correction,
    Ms25,
    OneEighth,
    CFG_DEFAULT | (0b1100 << 6),
    CFG_DEFAULT | (0b1100 << 6) | (2 << 11),
    1480_u16,
    3183.247
);
read_lux_test!(
    it50g1,
    Ms50,
    One,
    CFG_DEFAULT | (0b1000 << 6),
    CFG_DEFAULT | (0b1000 << 6),
    1480_u16,
    170.496
);
read_lux_test!(
    it200g14,
    Ms200,
    OneQuarter,
    CFG_DEFAULT | (0b0001 << 6),
    CFG_DEFAULT | (0b0001 << 6) | (3 << 11),
    1480_u16,
    170.496
);
read_lux_test!(
    it400g18,
    Ms400,
    OneEighth,
    CFG_DEFAULT | (0b0010 << 6),
    CFG_DEFAULT | (0b0010 << 6) | (2 << 11),
    1480_u16,
    170.496
);

set_test!(
    high_th_raw,
    set_high_threshold_raw,
    ALS_WH,
    0xABCD_u16,
    0xABCD
);
set_test!(
    low_th_raw,
    set_low_threshold_raw,
    ALS_WL,
    0xABCD_u16,
    0xABCD
);
set_test!(highth_lux, set_high_threshold_lux, ALS_WH, 1480_u16, 85.248);
set_test!(low_th_lux, set_low_threshold_lux, ALS_WL, 1480_u16, 85.248);

macro_rules! set_th_test {
    ($name:ident, $method:ident, $register:ident) => {
        #[test]
        fn $name() {
            let config1 = CFG_DEFAULT | (0b1100 << 6);
            let config2 = config1 | 2 << 11;
            let als_raw = 1479;
            let transactions = [
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Reg::ALS_CONF, config1 as u8, (config1 >> 8) as u8],
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Reg::ALS_CONF, config2 as u8, (config2 >> 8) as u8],
                ),
                I2cTrans::write(
                    DEV_ADDR,
                    vec![Reg::$register, als_raw as u8, (als_raw >> 8) as u8],
                ),
            ];
            let mut sensor = new(&transactions);
            sensor.set_integration_time(IT::Ms25).unwrap();
            sensor.set_gain(Gain::OneEighth).unwrap();
            sensor.$method(3183.247).unwrap();
            destroy(sensor);
        }
    };
}

set_th_test!(
    high_th_lux_applies_compensation,
    set_high_threshold_lux,
    ALS_WH
);
set_th_test!(
    low_th_lux_applies_compensation,
    set_low_threshold_lux,
    ALS_WL
);
