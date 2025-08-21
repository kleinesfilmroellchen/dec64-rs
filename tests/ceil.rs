#![allow(clippy::wildcard_imports)]

use dec64::Dec64;
use dec64::consts::*;

macro_rules! assert_eq_ceil {
    ($lhs:expr, $result:expr, $msg:expr) => {
        assert_eq!($lhs.ceil(), $result, $msg)
    };
}

#[test]
fn all_c() {
    assert_eq_ceil!(NAN, NAN, "NAN");
    assert_eq_ceil!(NONNORMAL_NAN, NAN, "NAN_NAN");
    assert_eq_ceil!(ZERO, ZERO, "ZERO");
    assert_eq_ceil!(ZIP, ZERO, "ZIP");
    // Discrepancy from C implementation: Minimum is already an integer
    assert_eq_ceil!(MIN, MIN, "MIN");
    assert_eq_ceil!(EPSILON, ONE, "EPSILON");
    assert_eq_ceil!(NEGATIVE_EPSILON, ZERO, "NEGATIVE_EPSILON");
    assert_eq_ceil!(CENT, ONE, "CENT");
    assert_eq_ceil!(HALF, ONE, "HALF");
    assert_eq_ceil!(ONE, ONE, "ONE");
    assert_eq_ceil!(NEGATIVE_ONE, NEGATIVE_ONE, "NEGATIVE_ONE");
    assert_eq_ceil!(
        Dec64::new(10000000000000001, -16),
        TWO,
        "1.0000000000000001"
    );
    assert_eq_ceil!(
        Dec64::new(-10000000000000001, -16),
        NEGATIVE_ONE,
        "-1.0000000000000001"
    );
    assert_eq_ceil!(Dec64::new(20000000000000000, -16), TWO, "TWO");
    assert_eq_ceil!(E, THREE, "e");
    assert_eq_ceil!(PI, FOUR, "PI");
    assert_eq_ceil!(NEGATIVE_PI, Dec64::new(-3, 0), "-PI");
    assert_eq_ceil!(MAXINT, MAXINT, "MAXINT");
    assert_eq_ceil!(MAX, MAX, "MAX");
    assert_eq_ceil!(MININT, MININT, "MININT");
    assert_eq_ceil!(Dec64::new(11111111111111111, -17), ONE, "0.1...");
    assert_eq_ceil!(Dec64::new(22222222222222222, -17), ONE, "0.2...");
    assert_eq_ceil!(Dec64::new(33333333333333333, -17), ONE, "0.3...");
    assert_eq_ceil!(Dec64::new(4444444444444444, -16), ONE, "0.4...");
    assert_eq_ceil!(Dec64::new(5555555555555556, -16), ONE, "0.5...");
    assert_eq_ceil!(Dec64::new(6666666666666667, -16), ONE, "0.6...");
    assert_eq_ceil!(Dec64::new(7777777777777778, -16), ONE, "0.7...");
    assert_eq_ceil!(Dec64::new(8888888888888889, -16), ONE, "0.8...");
    assert_eq_ceil!(Dec64::new(10000000000000000, -16), ONE, "1");
    assert_eq_ceil!(
        Dec64::new(-12500000000000000, -16),
        Dec64::new(-1, 0),
        "-1.25"
    );
    assert_eq_ceil!(
        Dec64::new(-1500000000000000, -15),
        Dec64::new(-1, 0),
        "-1.5"
    );
    assert_eq_ceil!(
        Dec64::new(-1560000000000000, -15),
        Dec64::new(-1, 0),
        "-1.56"
    );
    assert_eq_ceil!(Dec64::new(-11111111111111111, -17), ZERO, "-0.1...");
    assert_eq_ceil!(Dec64::new(-22222222222222222, -17), ZERO, "-0.2...");
    assert_eq_ceil!(Dec64::new(-33333333333333333, -17), ZERO, "-0.3...");
    assert_eq_ceil!(Dec64::new(-4444444444444444, -16), ZERO, "-0.4...");
    assert_eq_ceil!(Dec64::new(-5555555555555556, -16), ZERO, "-0.5...");
    assert_eq_ceil!(Dec64::new(-6666666666666667, -16), ZERO, "-0.6...");
    assert_eq_ceil!(Dec64::new(-7777777777777778, -16), ZERO, "-0.7...");
    assert_eq_ceil!(Dec64::new(-8888888888888889, -16), ZERO, "-0.8...");
    assert_eq_ceil!(
        Dec64::new(-10000000000000000, -16),
        NEGATIVE_ONE,
        "-10000000000000000e-16"
    );
    assert_eq_ceil!(Dec64::new(449, -2), FIVE, "4.49");
    assert_eq_ceil!(Dec64::new(-449, -2), Dec64::new(-4, 0), "-4.49");
    assert_eq_ceil!(Dec64::new(450, -2), FIVE, "4.50");
    assert_eq_ceil!(Dec64::new(-450, -2), Dec64::new(-4, 0), "-4.50");
    assert_eq_ceil!(Dec64::new(9, -1), ONE, "0.9");
    assert_eq_ceil!(Dec64::new(-9, -1), ZERO, "-0.9");
    assert_eq_ceil!(ALMOST_ONE, ONE, "ALMOST_ONE");
    assert_eq_ceil!(ALMOST_NEGATIVE_ONE, ZERO, "ALMOST_NEGATIVE_ONE");
    assert_eq_ceil!(Dec64::new(-999999999999999, -15), ZERO, "-0.9...");
    assert_eq_ceil!(Dec64::new(-9999999999999998, -16), ZERO, "-0.9...8");
}
