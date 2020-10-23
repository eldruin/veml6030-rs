/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
}

/// Integration time
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntegrationTime {
    /// 25 ms
    Ms25,
    /// 50 ms
    Ms50,
    /// 100 ms (default)
    Ms100,
    /// 200 ms
    Ms200,
    /// 400 ms
    Ms400,
    /// 800 ms
    Ms800,
}

impl IntegrationTime {
    /// Return the integration time in milliseconds
    pub fn as_ms(&self) -> u16 {
        match self {
            IntegrationTime::Ms25 => 25,
            IntegrationTime::Ms50 => 50,
            IntegrationTime::Ms100 => 100,
            IntegrationTime::Ms200 => 200,
            IntegrationTime::Ms400 => 400,
            IntegrationTime::Ms800 => 800,
        }
    }

    /// Return the integration time in microseconds
    pub fn as_us(&self) -> u32 {
        (self.as_ms() as u32) * 1000
    }
}

/// Gain
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gain {
    /// 1/8
    OneEighth,
    /// 1/4
    OneQuarter,
    /// 1 (default)
    One,
    /// 2
    Two,
}

/// Fault count
///
/// Number of consecutive fault events necessary to trigger interrupt.
/// This is referred to as "persistence" in the documentation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultCount {
    /// One (default)
    One,
    /// Two
    Two,
    /// Four
    Four,
    /// Eight
    Eight,
}

/// Power-saving mode
///
/// This combined with the integration time determines the repetition rate
/// and the power consumption of the device.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerSavingMode {
    /// One
    One,
    /// Two
    Two,
    /// Three
    Three,
    /// Four
    Four,
}

/// Interrupt status
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InterruptStatus {
    /// Whether the low threshold was exceeded consecutively as many times
    /// as configured as fault count.
    pub was_too_low: bool,
    /// Whether the high threshold was exceeded consecutively as many times
    /// as configured as fault count.
    pub was_too_high: bool,
}

/// Possible slave addresses
#[derive(Debug, Clone, Copy)]
pub enum SlaveAddr {
    /// Default slave address
    Default,
    /// Alternative slave address providing bit value for the ADDR pin
    Alternative(bool),
}

impl Default for SlaveAddr {
    /// Default slave address
    fn default() -> Self {
        SlaveAddr::Default
    }
}

impl SlaveAddr {
    pub(crate) fn addr(self) -> u8 {
        match self {
            SlaveAddr::Default => 0x10,
            SlaveAddr::Alternative(true) => 0x48,
            SlaveAddr::Alternative(false) => 0x10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_default_address() {
        let addr = SlaveAddr::default();
        assert_eq!(0x10, addr.addr());
    }

    #[test]
    fn can_generate_alternative_addresses() {
        assert_eq!(0x10, SlaveAddr::Alternative(false).addr());
        assert_eq!(0x48, SlaveAddr::Alternative(true).addr());
    }

    #[test]
    fn integration_time_as_int() {
        assert_eq!(IntegrationTime::Ms25.as_ms(), 25);
        assert_eq!(IntegrationTime::Ms25.as_us(), 25_000);
        assert_eq!(IntegrationTime::Ms800.as_ms(), 800);
        assert_eq!(IntegrationTime::Ms800.as_us(), 800_000);
    }
}
