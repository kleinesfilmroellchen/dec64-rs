use consts::*;
use core::f32;

pub mod consts;
mod diyfp;
#[cfg(feature = "std")]
mod fmt;
mod grisu2;
mod ops;
#[cfg(feature = "std")]
mod write;

/// Minimum value of DEC64 coefficient.
pub const MIN_COEFFICIENT: i64 = -0x80000000000000;
/// Maximum value of DEC64 coefficient.
pub const MAX_COEFFICIENT: i64 = 0x7FFFFFFFFFFFFF;
/// Minimum value of DEC64 exponent.
pub const MIN_EXP: i8 = -127;
/// Maximum value of DEC64 exponent.
pub const MAX_EXP: i8 = 127;

const EXPONENT_MASK: i64 = 0xff;
const COEFFICIENT_MASK: i64 = !EXPONENT_MASK;
const SIGN_MASK: i64 = 1 << 63;

/// The powers of 10.
#[allow(clippy::inconsistent_digit_grouping)]
const POWERS_OF_10: [u64; 20] = [
    1,                       // 0
    10,                      // 1
    100,                     // 2
    1000,                    // 3
    10000,                   // 4
    100000,                  // 5
    100000_0,                // 6
    100000_00,               // 7
    100000_000,              // 8
    100000_0000,             // 9
    100000_00000,            // 10
    100000_00000_0,          // 11
    100000_00000_00,         // 12
    100000_00000_000,        // 13
    100000_00000_0000,       // 14
    100000_00000_00000,      // 15
    100000_00000_00000_0,    // 16
    100000_00000_00000_00,   // 17
    100000_00000_00000_000,  // 18
    100000_00000_00000_0000, // 19
];

/// Struct holding DEC64 value.
#[derive(Clone, Copy, Default, Eq)]
#[repr(transparent)]
pub struct Dec64(i64);

impl Dec64 {
    /// Construct a Dec64 from raw coefficient and exponent parts.
    ///
    /// This can produce any kind of Dec64, including all varieties
    /// of zeros and NaNs.
    #[inline]
    pub const fn from_parts(coefficient: i64, exponent: i8) -> Self {
        // Double casting on exponent so we don't end up with bunch
        // of `1` bits on the left if the exponent is negative
        Self((coefficient << 8) | (exponent as u8 as i64))
    }

    /// Construct a Dec64 from a raw bitpattern.
    ///
    /// This can produce any kind of Dec64, and never modifies the bitpattern.
    #[inline]
    pub const fn from_raw(value: i64) -> Self {
        Self(value)
    }

    /// Construct a Dec64 from coefficient and exponent values.
    ///
    /// This will produce only standard (coefficient zeroed) kind of zero and NaN.
    ///
    /// Numbers that are too huge to be contained in this format become NaN.
    /// Numbers that are too tiny to be contained in this format become zero.
    pub fn new(mut coefficient: i64, mut exponent: i32) -> Self {
        if coefficient == 0 {
            // If the coefficient is zero, also zero the exponent.
            return ZERO;
        }

        // Is the exponent within supported range?
        if i32::from(MIN_EXP) <= exponent && exponent <= MAX_EXP.into() {
            // Is the coefficient within supported range?
            if (MIN_COEFFICIENT..=MAX_COEFFICIENT).contains(&coefficient) {
                // Coefficient and exponent are OK.
                return Dec64::from_parts(coefficient, exponent as i8);
            } else {
                // The coefficient is too long.
                // Add one to the exponent and Divide the coefficient by 10.
                loop {
                    exponent += 1;
                    if exponent > MAX_EXP.into() {
                        // We cannot fit this number.
                        return NAN;
                    }

                    let rem = coefficient % 10;
                    coefficient /= 10;
                    // Reminder of coefficient division for rounding decision.
                    // Does it fit now?
                    if (MIN_COEFFICIENT..=MAX_COEFFICIENT).contains(&coefficient) {
                        // Examine the remainder to determine if the coefficient should be rounded up
                        // or down. We will shift before adding in the rounding bit to get the cheap
                        // overflow check. If rounding does not cause overflow, pack up and get out.
                        let round_add = if rem <= -5 {
                            -1 << 8
                        } else if rem >= 5 {
                            1 << 8
                        } else {
                            0
                        };

                        // If rounding caused the coefficient to overflow, then go one more time
                        // through the loop. Otherwise return the dec64.
                        // This is extremely unlikely.
                        let (ret_value, overflow) = (coefficient << 8).overflowing_add(round_add);
                        if !overflow {
                            if coefficient == 0 {
                                // If the coefficient is zero, also zero the exponent.
                                return ZERO;
                            }
                            return Dec64(ret_value | ((exponent as u8) as i64));
                        }
                    }
                }
            }
        } else if exponent > MAX_EXP.into() {
            // The exponent is too big. We can attempt to reduce it by scaling back.
            // This can decrease it in a small set of cases.
            loop {
                // try multiplying the coefficient by 10
                let (coefficient_mul_10, overflow) = coefficient.overflowing_mul(10);
                if overflow || !(MIN_COEFFICIENT..=MAX_COEFFICIENT).contains(&coefficient_mul_10) {
                    // We failed to salvage.
                    return NAN;
                }
                coefficient = coefficient_mul_10;

                // decrement the exponent
                exponent -= 1;
                if exponent <= MAX_EXP.into() {
                    return Dec64::from_parts(coefficient, exponent as i8);
                }
            }
        } else if exponent < MIN_EXP.into() {
            // The exponent is too small. We can attempt to increase it by scaling forward.
            // This can increase it in a small set of cases.
            loop {
                let (coefficient_div_10, overflow) = coefficient.overflowing_div(10);
                if overflow || coefficient_div_10 == 0 {
                    // Value is too small to salvage.
                    return ZERO;
                }
                coefficient = coefficient_div_10;

                // increment the exponent
                exponent += 1;
                if exponent >= MIN_EXP.into() {
                    return Dec64::from_parts(coefficient, exponent as i8);
                }
            }
        }

        // We should've accounted for all cases.
        // If we ever reach here then there's a BUG in the implementation.
        unreachable!("Dec64::pack(): BUG");
    }

    /// Returns the DEC64 coefficient.
    #[inline]
    pub const fn coefficient(self) -> i64 {
        self.0 >> 8
    }

    /// Returns the DEC64 exponent.
    #[inline]
    pub const fn exponent(self) -> i8 {
        self.0 as i8
    }

    /// Returns the sign of the DEC64 (-1, 0, 1).
    #[inline]
    pub fn sign(self) -> i8 {
        if self.is_zero() {
            0
        } else {
            let raw_sign = self.0 & SIGN_MASK;
            match raw_sign {
                SIGN_MASK => -1,
                0 => 1,
                _ => unsafe { core::hint::unreachable_unchecked() },
            }
        }
    }

    /// Returns `true` if DEC64 is any Not a Number (NaN) and `false` otherwise.
    ///
    /// DEC64 NaN have exponent value of `-128`, and any coefficient.
    ///
    /// Note that NaNs are equal to each other only when their coefficient are
    /// equal too.
    #[inline]
    pub const fn is_nan(self) -> bool {
        self.exponent() == -128
    }

    /// Returns `true` if DEC64 is zero and `false` otherwise.
    ///
    /// DEC64 zeros have coefficient value of 0 and any non-NaN exponent.
    ///
    /// Note that all zeros are equal to each other, regardless of
    /// exponent value.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.coefficient() == 0 && !self.is_nan()
    }

    /// Returns `false` if the DEC64 contains a non-zero fractional part or if it is NaN,
    /// and `true` otherwise.
    #[inline]
    pub const fn is_integer(self) -> bool {
        let zero_coefficient = self.coefficient() == 0;
        if self.is_nan() || (self.exponent() <= -17 && !zero_coefficient) {
            // Extreme negative or positive exponents can never be integer. (This incules NaN).
            false
        } else if self.exponent() >= 0 || zero_coefficient {
            true
        } else if self.coefficient() % POWERS_OF_10[-self.exponent() as usize] as i64 == 0 {
            // Divide coefficient by the power of ten. If the remainder is zero, then return true.
            true
        } else {
            false
        }
    }

    /// Calculates the absolute value of this DEC64. In rare cases, this will lead to precision loss if the positive coefficient becomes too large to fit.
    #[inline]
    pub fn abs(self) -> Self {
        if self.is_nan() {
            return NAN;
        }
        let new_coefficient = self.coefficient().abs();
        Self::new(new_coefficient, self.exponent() as i32)
    }

    #[inline]
    fn coefficient_in_range<T>(coefficient: T) -> bool
    where
        T: From<i64> + PartialOrd,
    {
        (T::from(MIN_COEFFICIENT)..=T::from(MAX_COEFFICIENT)).contains(&coefficient)
    }
}

impl PartialEq<Dec64> for Dec64 {
    /// Compare two DEC64 numbers.
    /// Denormal zeroes are equal but denormal NaNs are not.
    fn eq(&self, other: &Dec64) -> bool {
        // If the numbers are trivally equal, then return true.
        if self.0 == other.0 {
            return true;
        }

        // Zeroes are equal.
        if self.is_zero() && other.is_zero() {
            return true;
        }

        // Do it the hard way by subtracting. Is the difference zero?
        (*self - *other).is_zero()
    }
}

impl PartialOrd<Dec64> for Dec64 {
    fn partial_cmp(&self, other: &Dec64) -> Option<core::cmp::Ordering> {
        // Trivial and NAN equality.
        if self.0 == other.0 || (self.is_nan() && other.is_nan()) {
            Some(core::cmp::Ordering::Equal)
        } else {
            let diff = *self - *other;
            if diff.is_zero() {
                Some(core::cmp::Ordering::Equal)
            } else if diff.coefficient() > 0 {
                Some(core::cmp::Ordering::Greater)
            } else {
                Some(core::cmp::Ordering::Less)
            }
        }
    }
}

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

macro_rules! impl_integer {
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

impl_integer!(
    usize, u8, u16, u32, u64, isize, i8, i16, i32, i64, i128, u128
);
