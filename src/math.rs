use crate::consts::NAN;

use super::Dec64;

impl Dec64 {
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
        // We’re +-1, so the modulus method doesn’t work.
        if self.coefficient().abs() == scaled_one {
            new_coefficient = scaled_one;
        } else if did_overflow {
            // Calculating the scaled 1 overflowed.
            // Since the coefficient is below 10^15, and the scaled 1 can go up to 10^19,
            // we’re definitely small enough to round to 0 or -1.
            new_coefficient = if self.sign() > 0 { 0 } else { 1 };
            new_exponent = 0;
        }
        // println!(
        //     "old coeff {} abs {} new coeff {new_coefficient} scaled one {scaled_one} => {}",
        //     self.coefficient(),
        //     self.coefficient().abs(),
        //     new_coefficient * self.sign() as i64
        // );
        Self::new(new_coefficient * self.sign() as i64, new_exponent as i32)
    }
}
