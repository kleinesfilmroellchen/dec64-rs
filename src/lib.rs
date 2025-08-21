use consts::*;

mod cmp;
pub mod consts;
#[cfg(feature = "std")]
mod fmt;
mod from;
mod math;
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

    #[inline]
    fn coefficient_in_range<T>(coefficient: T) -> bool
    where
        T: From<i64> + PartialOrd,
    {
        (T::from(MIN_COEFFICIENT)..=T::from(MAX_COEFFICIENT)).contains(&coefficient)
    }
}
