/// Implementations of mathematical operations on DEC64.
use super::{Dec64, POWERS_OF_10, SIGN_MASK, consts::NAN};

impl Dec64 {
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
}
