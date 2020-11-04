use crate::{Gain, IntegrationTime};
// necessary only for targets without math function implementation
#[allow(unused_imports)]
use libm::F64Ext;

/// Calculate raw value for threshold applying compensation if necessary.
///
/// For values higher than 1000 lx and 1/4 or 1/8 gain, the inverse of the
/// compensation formula is applied. This involves quite some math so it
/// may be interesting to calculate the threshold values ahead of time.
pub fn calculate_raw_threshold_value(it: IntegrationTime, gain: Gain, lux: f32) -> u16 {
    let factor = get_lux_raw_conversion_factor(it, gain);
    if (gain == Gain::OneQuarter || gain == Gain::OneEighth) && lux > 1000.0 {
        let lux = inverse_high_lux_correction(f64::from(lux));
        (lux / f64::from(factor)) as u16
    } else {
        (f64::from(lux) / f64::from(factor)) as u16
    }
}

pub(crate) fn get_lux_raw_conversion_factor(it: IntegrationTime, gain: Gain) -> f32 {
    let gain_factor = match gain {
        Gain::Two => 1.0,
        Gain::One => 2.0,
        Gain::OneQuarter => 8.0,
        Gain::OneEighth => 16.0,
    };
    let it_factor = match it {
        IntegrationTime::Ms800 => 0.0036,
        IntegrationTime::Ms400 => 0.0072,
        IntegrationTime::Ms200 => 0.0144,
        IntegrationTime::Ms100 => 0.0288,
        IntegrationTime::Ms50 => 0.0576,
        IntegrationTime::Ms25 => 0.1152,
    };
    gain_factor * it_factor
}

const C0: f64 = 1.0023;
const C1: f64 = 8.1488e-05;
const C2: f64 = -9.3924e-09;
const C3: f64 = 6.0135e-13;

pub(crate) fn correct_high_lux(lux: f64) -> f64 {
    lux * lux * lux * lux * C3 + lux * lux * lux * C2 + lux * lux * C1 + lux * C0
}

fn inverse_high_lux_correction(lux: f64) -> f64 {
    // Inverse of the polinomial used to correct for lux > 1000.
    // `y = 6.0135e-13*(x^4) - 9.3924e-9*(x^3) + 8.1488e-5*(x^2) + 1.0023*x`.
    // This runs into underflow/overlow issues if trying to solve it directly.
    // However, it can be solved for unknown coefficients and then
    // we put in the values.
    -C2 / (4.0 * C3)
        - (C2.powf(2.0) / (4.0 * C3.powf(2.0)) - (2.0 * C1) / (3.0 * C3)
            + (2.0_f64.powf(1.0 / 3.0) * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux))
                / (3.0
                    * C3
                    * (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                        - 27.0 * C2.powf(2.0) * lux
                        + 72.0 * C3 * C1 * lux
                        + (-4.0 * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux).powf(3.0)
                            + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0
                                + 27.0 * C3 * C0.powf(2.0)
                                - 27.0 * C2.powf(2.0) * lux
                                + 72.0 * C3 * C1 * lux)
                                .powf(2.0))
                        .sqrt())
                    .powf(1.0 / 3.0))
            + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                - 27.0 * C2.powf(2.0) * lux
                + 72.0 * C3 * C1 * lux
                + (-4.0 * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux).powf(3.0)
                    + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                        - 27.0 * C2.powf(2.0) * lux
                        + 72.0 * C3 * C1 * lux)
                        .powf(2.0))
                .sqrt())
            .powf(1.0 / 3.0)
                / (3.0 * 2.0_f64.powf(1.0 / 3.0) * C3))
            .sqrt()
            / 2.0
        + (C2.powf(2.0) / (2.0 * C3.powf(2.0))
            - (4.0 * C1) / (3.0 * C3)
            - (2.0_f64.powf(1.0 / 3.0) * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux))
                / (3.0
                    * C3
                    * (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                        - 27.0 * C2.powf(2.0) * lux
                        + 72.0 * C3 * C1 * lux
                        + (-4.0 * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux).powf(3.0)
                            + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0
                                + 27.0 * C3 * C0.powf(2.0)
                                - 27.0 * C2.powf(2.0) * lux
                                + 72.0 * C3 * C1 * lux)
                                .powf(2.0))
                        .sqrt())
                    .powf(1.0 / 3.0))
            - (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                - 27.0 * C2.powf(2.0) * lux
                + 72.0 * C3 * C1 * lux
                + (-4.0 * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux).powf(3.0)
                    + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                        - 27.0 * C2.powf(2.0) * lux
                        + 72.0 * C3 * C1 * lux)
                        .powf(2.0))
                .sqrt())
            .powf(1.0 / 3.0)
                / (3.0 * 2.0_f64.powf(1.0 / 3.0) * C3)
            - (-(C2.powf(3.0) / C3.powf(3.0)) + (4.0 * C2 * C1) / C3.powf(2.0) - (8.0 * C0) / C3)
                / (4.0
                    * (C2.powf(2.0) / (4.0 * C3.powf(2.0)) - (2.0 * C1) / (3.0 * C3)
                        + (2.0_f64.powf(1.0 / 3.0)
                            * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux))
                            / (3.0
                                * C3
                                * (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0
                                    + 27.0 * C3 * C0.powf(2.0)
                                    - 27.0 * C2.powf(2.0) * lux
                                    + 72.0 * C3 * C1 * lux
                                    + (-4.0
                                        * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux)
                                            .powf(3.0)
                                        + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0
                                            + 27.0 * C3 * C0.powf(2.0)
                                            - 27.0 * C2.powf(2.0) * lux
                                            + 72.0 * C3 * C1 * lux)
                                            .powf(2.0))
                                    .sqrt())
                                .powf(1.0 / 3.0))
                        + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0 + 27.0 * C3 * C0.powf(2.0)
                            - 27.0 * C2.powf(2.0) * lux
                            + 72.0 * C3 * C1 * lux
                            + (-4.0
                                * (C1.powf(2.0) - 3.0 * C2 * C0 - 12.0 * C3 * lux).powf(3.0)
                                + (2.0 * C1.powf(3.0) - 9.0 * C2 * C1 * C0
                                    + 27.0 * C3 * C0.powf(2.0)
                                    - 27.0 * C2.powf(2.0) * lux
                                    + 72.0 * C3 * C1 * lux)
                                    .powf(2.0))
                            .sqrt())
                        .powf(1.0 / 3.0)
                            / (3.0 * 2.0_f64.powf(1.0 / 3.0) * C3))
                        .sqrt()))
        .sqrt()
            / 2.0
}

#[cfg(test)]
mod correction_tests {
    use super::*;

    macro_rules! check_correction {
        ($name:ident, $lux:expr) => {
            #[test]
            fn $name() {
                let lux = $lux;
                let corrected = correct_high_lux(lux);
                let inverse_correction = inverse_high_lux_correction(corrected);
                assert!(lux - 0.5 < inverse_correction);
                assert!(lux + 0.5 > inverse_correction);
            }
        };
    }

    check_correction!(_1000, 1000.0);
    check_correction!(_1500, 1500.0);
    check_correction!(_2500, 2500.0);
    check_correction!(_5000, 5000.0);
    check_correction!(_7890, 7890.0);
    check_correction!(_10000, 10000.0);
    check_correction!(_15000, 15000.0);
    check_correction!(_20000, 20000.0);
    check_correction!(_56789, 56789.0);
    check_correction!(_78901, 78901.0);
    check_correction!(_120000, 120_000.0);
}
