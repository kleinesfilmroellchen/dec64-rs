//! Implementation of traits from `std::ops`.

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{MAX_EXP, MIN_EXP};

use super::{COEFFICIENT_MASK, Dec64, NAN, ZERO};

impl Add for Dec64 {
    type Output = Dec64;

    fn add(self, other: Dec64) -> Dec64 {
        if self.is_nan() || other.is_nan() {
            return NAN;
        }

        if self.exponent() == 0 && other.exponent() == 0 {
            // If the two exponents are both zero (which is usually the case for integers)
            // we can take the fast path. Since the exponents are both zero, we can simply
            // add the numbers together and check for overflow.
            let (sum, overflow) = self.0.overflowing_add(other.0);
            if !overflow {
                return Dec64::from_raw(sum);
            }
        } else {
            // The slow path is taken if the two operands do not both have zero exponents.
            if self.exponent() == other.exponent() {
                // The exponents match so we may add now. Zero out the exponents so there
                // will be no carry into the coefficients when the coefficients are added.
                // If the result is zero, then return the normal zero.
                let (sum, overflow) =
                    (self.0 & COEFFICIENT_MASK).overflowing_add(other.0 & COEFFICIENT_MASK);
                if !overflow {
                    return Dec64::from_parts(sum >> 8, self.exponent());
                }
            } else {
                // The slower path is taken when neither operand is nan, and their
                // exponents are different.

                // Before addition can take place, the exponents
                // must be made to match.
                let (hi, lo) = if self.exponent() > other.exponent() {
                    (self, other)
                } else {
                    (other, self)
                };

                let mut lo_coefficient = lo.coefficient();
                // If lower value has zero coefficient return the higher.
                if lo_coefficient == 0 {
                    return hi;
                }
                let mut lo_exponent = lo.exponent();
                let mut hi_coefficient = hi.coefficient();
                let mut hi_exponent = hi.exponent();

                // First we will try to decrease the high exponent. When we decrease the exponent
                // by 1, we must also multiply the coefficient by 10. We can do this as long as
                // there is no overflow. We have 8 extra bits to work with, so we can do this
                // at least twice, possibly more.
                loop {
                    // Before decrementing the exponent, multiply.
                    let (hi_coefficient_mul_10, overflow) = hi_coefficient.overflowing_mul(10);
                    if overflow {
                        // We cannot decrease exponent any more.
                        break;
                    }

                    hi_exponent -= 1;
                    hi_coefficient = hi_coefficient_mul_10;

                    // Are the exponents equal yet?
                    if hi_exponent == lo_exponent {
                        // We can sum & then pack.
                        let sum = hi_coefficient + lo_coefficient;
                        return Self::new(sum, hi_exponent as i32);
                    }
                }

                // If we're still not done yet and we cannot decrease the first exponent any more,
                // so we must instead try to increase the second exponent, which will result in
                // a loss of significance.
                // That is the heartbreak of floating point.
                loop {
                    lo_coefficient /= 10;
                    lo_exponent += 1;

                    // Are the exponents equal yet?
                    if hi_exponent == lo_exponent {
                        // We can sum & then pack.
                        let sum = hi_coefficient + lo_coefficient;
                        return Self::new(sum, lo_exponent as i32);
                    }
                }
            }
        }

        // Sum had an overflow.
        // This path happens only when both exponents are the same.
        // Re-add shifted coefficients (this won't overflow) and pack.
        // In original implementation of this path is much more elegant,
        // But here we don't have access to the carry flag.
        let sum = self.coefficient() + other.coefficient();

        Self::new(sum, self.exponent() as i32)
    }
}

/// Currently this doesn't have specialized implementation.
/// Just add negated number.
impl Sub for Dec64 {
    type Output = Dec64;

    #[inline]
    fn sub(self, other: Dec64) -> Dec64 {
        self.add(-other)
    }
}

impl Neg for Dec64 {
    type Output = Dec64;

    fn neg(self) -> Dec64 {
        if self.is_nan() {
            return NAN;
        }

        // If the coefficient is zero, then zero the exponent too.
        if self.coefficient() == 0 {
            return ZERO;
        }

        // Result of this operation is the exponent and complemented coefficient.
        let neg = COEFFICIENT_MASK ^ self.0;
        // Perform coefficient U2 complement.
        match neg.overflowing_add(1 << 8) {
            // Pass the result.
            (ret, false) => Dec64::from_raw(ret),
            // The coefficient is -36028797018963968, aka. MIN_COEFFICIENT which is the only
            // coefficient that cannot be trivially negated. So we do this the hard way.
            (_, true) => Self::new(-self.coefficient(), self.exponent() as i32),
        }
    }
}

impl Mul for Dec64 {
    type Output = Dec64;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() {
            return NAN;
        }
        if self.is_zero() || rhs.is_zero() {
            return ZERO;
        }

        // (c1 * 10^e1) * (c2 * 10^e2) = c1 * c2 * 10^(e1 + e2)

        // Do multiplication in 128 bits and reduce coefficient later.
        let mut full_coefficient = self.coefficient() as i128 * rhs.coefficient() as i128;
        // last bit that was flushed out during coefficient reduction
        let mut last_flushed_bit = 0;
        let mut new_exponent = self.exponent() as i16 + rhs.exponent() as i16;
        println!("-- {full_coefficient} * 10^{new_exponent}");
        // Reduce coefficient as needed (possibly reducing precision):
        // either coefficient is out of range, or exponent is too small.
        while !Self::coefficient_in_range(full_coefficient) || new_exponent < MIN_EXP.into() {
            last_flushed_bit = full_coefficient.abs() % 10;
            full_coefficient /= 10;
            new_exponent += 1;
            println!("reduced out-of-range coeff {full_coefficient} at exponent {new_exponent}");
        }

        // Reduce exponent as needed while increasing the coefficient (but not past what we did above)
        while new_exponent > MAX_EXP.into() && Self::coefficient_in_range(full_coefficient) {
            last_flushed_bit = 0;
            full_coefficient *= 10;
            new_exponent -= 1;
            println!("increased coeff {full_coefficient} for exponent {new_exponent}");
        }

        // Number is out of range (still), so return nan (if too large) or zero (if too small)
        if new_exponent > MAX_EXP.into() || !Self::coefficient_in_range(full_coefficient) {
            NAN
        } else if new_exponent < MIN_EXP.into() {
            ZERO
        } else {
            // Add 1 to coefficient
            Self::from_parts(
                (full_coefficient
                    + if last_flushed_bit >= 5 {
                        full_coefficient.signum()
                    } else {
                        0
                    }) as _,
                new_exponent as _,
            )
        }
    }
}

impl Div for Dec64 {
    type Output = Dec64;

    fn div(self, rhs: Self) -> Self::Output {
        if self.is_nan() || rhs.is_nan() || rhs.is_zero() {
            return NAN;
        }
        if self.is_zero() {
            return ZERO;
        }

        // (c1 * 10^e1) * (c2 * 10^e2)^-1 = (c1 * c2^-1) * 10^(e1-e2)
        let mut lhs_coefficient = self.coefficient();
        let mut rhs_coefficient = rhs.coefficient();
        let mut maybe_coefficient = lhs_coefficient / rhs_coefficient;
        let mut coefficient_remainder = lhs_coefficient % rhs_coefficient;
        let new_exponent = self.exponent() as i16 - rhs.exponent() as i16;

        // Slow path: Division is inexact, increase coefficient sizes either until limit is reached or until division becomes exact.
        while coefficient_remainder != 0 {
            // Reached coefficient precision limit, stop here.
            if !Self::coefficient_in_range(lhs_coefficient)
                || Self::coefficient_in_range(rhs_coefficient)
            {
                lhs_coefficient /= 10;
                rhs_coefficient /= 10;
                maybe_coefficient = lhs_coefficient / rhs_coefficient;
                break;
            }
            lhs_coefficient *= 10;
            rhs_coefficient *= 10;
            maybe_coefficient = lhs_coefficient / rhs_coefficient;
            coefficient_remainder = lhs_coefficient % rhs_coefficient;
        }
        // Exponent is out of range now, so return nan (if too large) or zero (if too small)
        if new_exponent > MAX_EXP.into() {
            NAN
        } else if new_exponent < MIN_EXP.into() {
            ZERO
        } else {
            Self::from_parts(maybe_coefficient, new_exponent as i8)
        }
    }
}
