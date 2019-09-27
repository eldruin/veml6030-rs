extern crate linux_embedded_hal as hal;
extern crate veml6030;
use veml6030::{SlaveAddr, Veml6030};

fn main() {
    let dev = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Veml6030::new(dev, address);
    sensor.enable().unwrap();
    loop {
        let lux = sensor.read_lux().unwrap();
        println!("lux: {:2}", lux);
    }
}
