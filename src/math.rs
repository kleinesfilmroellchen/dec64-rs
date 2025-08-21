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

    /// Return the largest DEC64 integer which is not larger than this number; i.e. round down towards negative infinity.
    /// If this number is already an integer, it is returned itself.
    #[inline]
    pub fn floor(self) -> Self {
        if self.is_nan() {
            return NAN;
        }
        // Number is already an integer
        if self.exponent() >= 0 {
            return self;
        }
        let mut new_exponent = self.exponent();
        // Obtain the number 1 scaled to the same exponent as the coefficient.
        let (scaled_one, did_overflow) = 10i64.overflowing_pow((-self.exponent()) as u32);
        // In case the properly-scaled 1 overflows 64-bit integers, it can be zero, which we need to avoid.
        let remainder = self.coefficient().abs() % scaled_one.max(1);
        // Slice off everything below the exponent by subtracting coefficient % 1.
        let mut new_coefficient = self.coefficient().abs() - remainder;
        // If negative, we rounded in the wrong direction, so subtract 1.
        if self.sign() == -1 && remainder > 0 {
            new_coefficient += scaled_one;
        }
        if did_overflow {
            // Calculating the scaled 1 overflowed.
            // Since the coefficient is below 10^15, and the scaled 1 can go up to 10^19,
            // we’re definitely small enough to round to 0 or -1.
            new_coefficient = if self.sign() > 0 { 0 } else { 1 };
            new_exponent = 0;
        } else if remainder == 0 {
            // We’re already an integer, so the modulus method doesn’t work.
            // (Importantly, this case will break in case of scaled 1 overflow)
            new_coefficient = self.coefficient().abs();
        }
        // println!(
        //     "old coeff {} abs {} new coeff {new_coefficient} scaled one {scaled_one} => {}",
        //     self.coefficient(),
        //     self.coefficient().abs(),
        //     new_coefficient * self.sign() as i64
        // );
        Self::new(new_coefficient * self.sign() as i64, new_exponent as i32)
    }

    /// Return the smallest DEC64 integer which is not smaller than this number; i.e. round up towards negative infinity.
    /// If this number is already an integer, it is returned itself.
    #[inline]
    pub fn ceil(self) -> Self {
        if self.is_nan() {
            return NAN;
        }
        // Number is already an integer
        if self.exponent() >= 0 {
            return self;
        }
        let mut new_exponent = self.exponent();
        // Obtain the number 1 scaled to the same exponent as the coefficient.
        let (scaled_one, did_overflow) = 10i64.overflowing_pow((-self.exponent()) as u32);
        // In case the properly-scaled 1 overflows 64-bit integers, it can be zero, which we need to avoid.
        let remainder = self.coefficient().abs() % scaled_one.max(1);
        // Slice off everything below the exponent by subtracting coefficient % 1.
        let mut new_coefficient = self.coefficient().abs() + (scaled_one - remainder);
        // If negative, we rounded in the wrong direction, so add 1.
        if self.sign() == -1 && remainder > 0 {
            new_coefficient -= scaled_one;
        }

        if did_overflow {
            // Calculating the scaled 1 overflowed.
            // Since the coefficient is below 10^15, and the scaled 1 can go up to 10^19,
            // we’re definitely small enough to round to 1 or 0.
            new_coefficient = if self.sign() > 0 { 1 } else { 0 };
            new_exponent = 0;
        } else if remainder == 0 {
            // We’re already an integer, so the modulus method doesn’t work.
            // (Importantly, this case will break in case of scaled 1 overflow)
            new_coefficient = self.coefficient().abs();
        }
        println!(
            "old coeff {} abs {} new coeff {new_coefficient} scaled one {scaled_one} => {}",
            self.coefficient(),
            self.coefficient().abs(),
            new_coefficient * self.sign() as i64
        );
        Self::new(new_coefficient * self.sign() as i64, new_exponent as i32)
    }
}
