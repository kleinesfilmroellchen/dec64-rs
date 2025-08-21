use crate::Dec64;

use core::cmp::Ordering;

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
    fn partial_cmp(&self, other: &Dec64) -> Option<Ordering> {
        // Trivial and NAN equality.
        if self.0 == other.0 || (self.is_nan() && other.is_nan()) {
            Some(Ordering::Equal)
        } else {
            let diff = *self - *other;
            if diff.is_zero() {
                Some(Ordering::Equal)
            } else if diff.coefficient() > 0 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}
