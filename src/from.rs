use core::f32;

use crate::Dec64;

mod diyfp;
mod grisu2;

/// Converts an exponent to its corresponding power as a binary floating-point number.
fn exponent_to_power_f64(e: i8) -> f64 {
    const POSITIVE_POWERS: [f64; 23] = [
        1.0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16,
        1e17, 1e18, 1e19, 1e20, 1e21, 1e22,
    ];

    const NEGATIVE_POWERS: [f64; 23] = [
        1.0, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13,
        1e-14, 1e-15, 1e-16, 1e-17, 1e-18, 1e-19, 1e-20, 1e-21, 1e-22,
    ];

    let index = e.unsigned_abs() as usize;

    if index < 23 {
        if e < 0 {
            NEGATIVE_POWERS[index]
        } else {
            POSITIVE_POWERS[index]
        }
    } else {
        // powf is more accurate
        10f64.powf(e as f64)
    }
}

fn exponent_to_power_f32(e: i8) -> f32 {
    const POSITIVE_POWERS: [f32; 16] = [
        1.0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15,
    ];

    const NEGATIVE_POWERS: [f32; 16] = [
        1.0, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13,
        1e-14, 1e-15,
    ];

    let index = e.unsigned_abs() as usize;

    if index < 16 {
        if e < 0 {
            NEGATIVE_POWERS[index]
        } else {
            POSITIVE_POWERS[index]
        }
    } else {
        // powf is more accurate
        10f32.powf(e as f32)
    }
}

impl From<Dec64> for f64 {
    fn from(dec: Dec64) -> f64 {
        (dec.coefficient() as f64) * exponent_to_power_f64(dec.exponent())
    }
}

impl From<Dec64> for f32 {
    fn from(dec: Dec64) -> f32 {
        (dec.coefficient() as f32) * exponent_to_power_f32(dec.exponent())
    }
}

impl From<f64> for Dec64 {
    fn from(float: f64) -> Dec64 {
        if float < 0.0 {
            let (coefficient, exponent) = grisu2::convert(-float);

            Dec64::from_parts(-(coefficient as i64), exponent as i8)
        } else {
            let (coefficient, exponent) = grisu2::convert(float);

            Dec64::from_parts(coefficient as i64, exponent as i8)
        }
    }
}

impl From<f32> for Dec64 {
    fn from(float: f32) -> Dec64 {
        if float < 0.0 {
            let (coefficient, exponent) = grisu2::convert(-float as f64);

            Dec64::from_parts(-(coefficient as i64), exponent as i8)
        } else {
            let (coefficient, exponent) = grisu2::convert(float as f64);

            Dec64::from_parts(coefficient as i64, exponent as i8)
        }
    }
}

macro_rules! impl_integer_small {
    ($( $t:ty ),*) => ($(
        impl From<$t> for Dec64 {
            fn from(num: $t) -> Dec64 {
                Dec64::from_raw((num as i64) << 8)
            }
        }

        impl From<Dec64> for $t {
            fn from(dec: Dec64) -> $t {
                let exponent = dec.exponent();

                if exponent <= 0 {
                    dec.coefficient() as $t
                } else {
                    // This may overflow, which is fine
                    (dec.coefficient() * 10i64.pow(exponent as u32)) as $t
                }
            }
        }
    )*)
}

macro_rules! impl_integer_large_signed {
    ($( $t:ty ),*) => ($(
        impl From<$t> for Dec64 {
            fn from(num: $t) -> Dec64 {
                let mut adjusted_num = num;
                let mut last_digit = 0;
                let mut exponent = 0;
                while (adjusted_num > i64::MAX as $t) || (adjusted_num < i64::MIN as $t) {
                    last_digit = (adjusted_num % 10).abs();
                    adjusted_num /= 10;
                    exponent += 1;
                }
                Dec64::new((adjusted_num + if last_digit >= 5 { adjusted_num.signum() } else { 0 }) as i64, exponent)
            }
        }

        impl From<Dec64> for $t {
            fn from(dec: Dec64) -> $t {
                let exponent = dec.exponent();

                if exponent <= 0 {
                    dec.coefficient() as $t
                } else {
                    // This may overflow, which is fine
                    (dec.coefficient() * 10i64.pow(exponent as u32)) as $t
                }
            }
        }
    )*)
}

macro_rules! impl_integer_large_unsigned {
    ($( $t:ty ),*) => ($(
        impl From<$t> for Dec64 {
            fn from(num: $t) -> Dec64 {
                let mut adjusted_num = num;
                let mut last_digit = 0;
                let mut exponent = 0;
                while adjusted_num > i64::MAX as $t {
                    last_digit = adjusted_num % 10;
                    adjusted_num /= 10;
                    exponent += 1;
                }
                Dec64::new((adjusted_num + if last_digit >= 5 { 1 } else { 0 }) as i64, exponent)
            }
        }

        impl From<Dec64> for $t {
            fn from(dec: Dec64) -> $t {
                let exponent = dec.exponent();

                if exponent <= 0 {
                    dec.coefficient() as $t
                } else {
                    // This may overflow, which is fine
                    (dec.coefficient() * 10i64.pow(exponent as u32)) as $t
                }
            }
        }
    )*)
}

impl_integer_small!(u8, u16, u32, i8, i16, i32);
impl_integer_large_signed!(isize, i64, i128);
impl_integer_large_unsigned!(usize, u64, u128);
