#![allow(clippy::wildcard_imports)]

use dec64::Dec64;
use dec64::consts::*;

macro_rules! assert_eq_floor {
    ($lhs:expr, $result:expr, $msg:expr) => {
        assert_eq!($lhs.floor(), $result, $msg)
    };
}

#[test]
fn all_c() {
    assert_eq_floor!(NAN, NAN, "NAN");
    assert_eq_floor!(NAN_NAN, NAN, "NAN_NAN");
    assert_eq_floor!(ZERO, ZERO, "ZERO");
    assert_eq_floor!(ZIP, ZERO, "ZERO");
    // Discrepancy from C implementation: The minimum number is an integer, so flooring it has no effect.
    assert_eq_floor!(MIN, MIN, "MIN");
    assert_eq_floor!(EPSILON, ZERO, "EPSILON");
    assert_eq_floor!(NEG_EPSILON, NEG_ONE, "NEG_EPSILON");
    assert_eq_floor!(CENT, ZERO, "CENT");
    assert_eq_floor!(HALF, ZERO, "HALF");
    assert_eq_floor!(ONE, ONE, "ONE");
    assert_eq_floor!(NEG_ONE, NEG_ONE, "NEG_ONE");
    assert_eq_floor!(Dec64::new(10000000000000001, -16), ONE, "1.0000000000000001");
    assert_eq_floor!(
        Dec64::new(-10000000000000001, -16),
        Dec64::new(-2, 0),
        "-1.0000000000000001"
    );
    assert_eq_floor!(Dec64::new(20000000000000000, -16), TWO, "TWO");
    assert_eq_floor!(E, TWO, "e");
    assert_eq_floor!(PI, THREE, "PI");
    assert_eq_floor!(NEG_PI, Dec64::new(-4, 0), "-PI");
    assert_eq_floor!(MAXINT, MAXINT, "MAXINT");
    assert_eq_floor!(MAX, MAX, "MAX");
    assert_eq_floor!(MININT, MININT, "MININT");
    assert_eq_floor!(Dec64::new(11111111111111111, -17), ZERO, "0.1...");
    assert_eq_floor!(Dec64::new(22222222222222222, -17), ZERO, "0.2...");
    assert_eq_floor!(Dec64::new(33333333333333333, -17), ZERO, "0.3...");
    assert_eq_floor!(Dec64::new(4444444444444444, -16), ZERO, "0.4...");
    assert_eq_floor!(Dec64::new(5555555555555556, -16), ZERO, "0.5...");
    assert_eq_floor!(Dec64::new(6666666666666667, -16), ZERO, "0.6...");
    assert_eq_floor!(Dec64::new(7777777777777778, -16), ZERO, "0.7...");
    assert_eq_floor!(Dec64::new(8888888888888889, -16), ZERO, "0.8...");
    assert_eq_floor!(Dec64::new(9999999999999999, -16), ZERO, "0.9...");
    assert_eq_floor!(Dec64::new(10000000000000000, -16), ONE, "1");
    assert_eq_floor!(
        Dec64::new(-12500000000000000, -16),
        Dec64::new(-2, 0),
        "-1.25"
    );
    assert_eq_floor!(Dec64::new(-1500000000000000, -15), Dec64::new(-2, 0), "-1.5");
    assert_eq_floor!(Dec64::new(-1560000000000000, -15), Dec64::new(-2, 0), "-1.56");
    assert_eq_floor!(Dec64::new(-11111111111111111, -17), NEG_ONE, "-0.1...");
    assert_eq_floor!(Dec64::new(-22222222222222222, -17), NEG_ONE, "-0.2...");
    assert_eq_floor!(Dec64::new(-33333333333333333, -17), NEG_ONE, "-0.3...");
    assert_eq_floor!(Dec64::new(-4444444444444444, -16), NEG_ONE, "-0.4...");
    assert_eq_floor!(Dec64::new(-5555555555555556, -16), NEG_ONE, "-0.5...");
    assert_eq_floor!(Dec64::new(-6666666666666667, -16), NEG_ONE, "-0.6...");
    assert_eq_floor!(Dec64::new(-7777777777777778, -16), NEG_ONE, "-0.7...");
    assert_eq_floor!(Dec64::new(-8888888888888889, -16), NEG_ONE, "-0.8...");
    assert_eq_floor!(Dec64::new(-9999999999999999, -16), NEG_ONE, "-0.9...");
    assert_eq_floor!(Dec64::new(-10000000000000000, -16), NEG_ONE, "-1.0...");
    assert_eq_floor!(Dec64::new(449, -2), FOUR, "4.49");
    assert_eq_floor!(Dec64::new(-449, -2), Dec64::new(-5, 0), "-4.49");
    assert_eq_floor!(Dec64::new(450, -2), FOUR, "4.50");
    assert_eq_floor!(Dec64::new(-450, -2), Dec64::new(-5, 0), "-4.50");
    assert_eq_floor!(Dec64::new(-400, -2), Dec64::new(-4, 0), "-4.00");
    assert_eq_floor!(Dec64::new(-400, -3), NEG_ONE, "-0.400");
    assert_eq_floor!(Dec64::new(-1, -127), NEG_ONE, "-1e-127");
    // Extra test: 1e-127 should round down to 0.
    assert_eq_floor!(Dec64::new(1, -127), ZERO, "1e-127");
    assert_eq_floor!(Dec64::new(-1, -13), NEG_ONE, "-1e-13");
    assert_eq_floor!(Dec64::new(1, -12), ZERO, "1e-12");
    assert_eq_floor!(Dec64::new(-1, -12), NEG_ONE, "-1e-12");
    assert_eq_floor!(Dec64::new(-1, -11), NEG_ONE, "-1e-11");
    assert_eq_floor!(Dec64::new(-11, -11), NEG_ONE, "-11e-11");
    assert_eq_floor!(Dec64::new(-111, -11), NEG_ONE, "-111e-11");
    assert_eq_floor!(Dec64::new(-22, -11), NEG_ONE, "-22e-11");
    assert_eq_floor!(Dec64::new(-1, -1), NEG_ONE, "-1e-1");
    assert_eq_floor!(Dec64::new(-10, -3), NEG_ONE, "-10e-3");
    assert_eq_floor!(Dec64::new(9, -1), ZERO, "0.9");
    assert_eq_floor!(Dec64::new(-9, -1), NEG_ONE, "-0.9");
    assert_eq_floor!(ALMOST_ONE, ZERO, "ALMOST_ONE");
    assert_eq_floor!(ALMOST_NEG_ONE, NEG_ONE, "ALMOST_NEG_ONE");
    assert_eq_floor!(Dec64::new(-999999999999999, -15), NEG_ONE, "-0.9...");
    assert_eq_floor!(Dec64::new(-9999999999999998, -16), NEG_ONE, "-0.9...8");
}
