#![allow(clippy::wildcard_imports)]
#![allow(unused)]

use dec64::Dec64;
use dec64::consts::*;

macro_rules! assert_eq_div {
    ($lhs:expr, $rhs:expr, $result:expr, $msg:expr) => {
        assert_eq!(core::hint::black_box($lhs / $rhs), $result, $msg)
    };
}

#[test]
fn all_c() {
    all_c_tests();
}

pub fn all_c_tests() {
    assert_eq_div!(SIX, THREE, Dec64::new(20000000000000000, -16), "6 / 3");
    assert_eq_div!(NAN_NAN, TWO, NAN, "NAN_NAN / 2");
    assert_eq_div!(NAN, TWO, NAN, "NAN / 2");
    assert_eq_div!(ZERO, TWO, ZERO, "0 / 2");
    assert_eq_div!(ZIP, TWO, ZERO, "ZIP / 2");
    assert_eq_div!(ONE, TWO, HALF, "1 / 2");
    assert_eq_div!(TWO, TWO, ONE, "TWO");
    assert_eq_div!(FOUR, TWO, TWO, "4 / 2");
    assert_eq_div!(TEN, TWO, FIVE, "10 / 2");
    // Discrepancy from C implementation: This calculation should not overflow.
    assert_eq_div!(
        MIN,
        TWO,
        Dec64::from_parts(-18014398509481984, 127),
        "MIN / 2"
    );
    assert_eq_div!(Dec64::new(-2, 0), TWO, Dec64::new(-1, 0), "-2 / 2");
    assert_eq_div!(Dec64::new(-1, 0), TWO, Dec64::new(-5, -1), "-1 / 2");
    assert_eq_div!(NAN, NAN, NAN, "NAN / NAN");
    assert_eq_div!(
        Dec64::new(4195835, 0),
        Dec64::new(3145727, 0),
        Dec64::new(13338204491362410, -16),
        "4195835 / 3145727"
    );
    assert_eq_div!(NAN, THREE, NAN, "NAN / 3");
    assert_eq_div!(NAN_NAN, NAN_NAN, NAN, "NAN_NAN / NAN_NAN");
    assert_eq_div!(NAN_NAN, ONE, NAN, "NAN_NAN / 1");
    // Discrepancy from C implementation: All operations with NAN result in NAN.
    assert_eq_div!(ZERO, NAN, NAN, "0 / NAN");
    // Discrepancy from C implementation: All operations with NAN result in NAN.
    assert_eq_div!(ZERO, NAN_NAN, NAN, "0 / NAN_NAN");
    // Discrepancy from C implementation: Divide by zero yields NAN.
    assert_eq_div!(ZERO, ZIP, NAN, "0 / ZIP");
    // Discrepancy from C implementation: All operations with NAN result in NAN.
    assert_eq_div!(ZIP, NAN, NAN, "ZIP / NAN");
    // Discrepancy from C implementation: All operations with NAN result in NAN.
    assert_eq_div!(ZIP, NAN_NAN, NAN, "ZIP / NAN_NAN");
    // Discrepancy from C implementation: Divide by zero yields NAN.
    assert_eq_div!(ZIP, ZERO, NAN, "ZIP / 0");
    // Discrepancy from C implementation: Divide by zero yields NAN.
    assert_eq_div!(ZIP, ZIP, NAN, "ZIP / ZIP");
    assert_eq_div!(ZERO, ONE, ZERO, "0 / 1");
    // Discrepancy from C implementation: Divide by zero yields NAN.
    assert_eq_div!(ZERO, ZERO, NAN, "0 / 0");
    assert_eq_div!(ONE, ZERO, NAN, "1 / 0");
    assert_eq_div!(ONE, NEG_ONE, Dec64::new(-10000000000000000, -16), "1 / -1");
    assert_eq_div!(NEG_ONE, ONE, Dec64::new(-10000000000000000, -16), "-1 / 1");
    assert_eq_div!(ONE, THREE, Dec64::new(33333333333333333, -17), "1 / 3");
    assert_eq_div!(TWO, THREE, Dec64::new(6666666666666667, -16), "2 / 3");
    assert_eq_div!(
        TWO,
        Dec64::new(30000000000000000, -16),
        Dec64::new(6666666666666667, -16),
        "2 / 3 alias"
    );
    assert_eq_div!(
        Dec64::new(20000000000000000, -16),
        THREE,
        Dec64::new(6666666666666667, -16),
        "2 / 3 alias"
    );
    assert_eq_div!(
        Dec64::new(20000000000000000, -16),
        Dec64::new(30000000000000000, -16),
        Dec64::new(6666666666666667, -16),
        "2 / 3 alias"
    );
    assert_eq_div!(FIVE, THREE, Dec64::new(16666666666666667, -16), "5 / 3");
    assert_eq_div!(
        FIVE,
        Dec64::new(-30000000000000000, -16),
        Dec64::new(-16666666666666667, -16),
        "5 / -3"
    );
    assert_eq_div!(
        Dec64::new(-50000000000000000, -16),
        THREE,
        Dec64::new(-16666666666666667, -16),
        "-5 / 3"
    );
    assert_eq_div!(
        Dec64::new(-50000000000000000, -16),
        Dec64::new(-30000000000000000, -16),
        Dec64::new(16666666666666667, -16),
        "-5 / -3"
    );
    assert_eq_div!(SIX, NAN, NAN, "6 / NAN");
    assert_eq_div!(ZERO, NINE, ZERO, "0 / 9");
    assert_eq_div!(ONE, NINE, Dec64::new(11111111111111111, -17), "1 / 9");
    assert_eq_div!(TWO, NINE, Dec64::new(22222222222222222, -17), "2 / 9");
    assert_eq_div!(THREE, NINE, Dec64::new(33333333333333333, -17), "3 / 9");
    assert_eq_div!(FOUR, NINE, Dec64::new(4444444444444444, -16), "4 / 9");
    assert_eq_div!(FIVE, NINE, Dec64::new(5555555555555556, -16), "5 / 9");
    assert_eq_div!(SIX, NINE, Dec64::new(6666666666666667, -16), "6 / 9");
    assert_eq_div!(SEVEN, NINE, Dec64::new(7777777777777778, -16), "7 / 9");
    assert_eq_div!(EIGHT, NINE, Dec64::new(8888888888888889, -16), "8 / 9");
    assert_eq_div!(NINE, NINE, ONE, "9 / 9");
    assert_eq_div!(ZERO, NEG_NINE, ZERO, "0 / -9");
    assert_eq_div!(ONE, NEG_NINE, Dec64::new(-11111111111111111, -17), "1 / -9");
    assert_eq_div!(TWO, NEG_NINE, Dec64::new(-22222222222222222, -17), "2 / -9");
    assert_eq_div!(
        THREE,
        NEG_NINE,
        Dec64::new(-33333333333333333, -17),
        "3 / -9"
    );
    assert_eq_div!(FOUR, NEG_NINE, Dec64::new(-4444444444444444, -16), "4 / -9");
    assert_eq_div!(FIVE, NEG_NINE, Dec64::new(-5555555555555556, -16), "5 / -9");
    assert_eq_div!(SIX, NEG_NINE, Dec64::new(-6666666666666667, -16), "6 / -9");
    assert_eq_div!(
        SEVEN,
        NEG_NINE,
        Dec64::new(-7777777777777778, -16),
        "7 / -9"
    );
    assert_eq_div!(
        EIGHT,
        NEG_NINE,
        Dec64::new(-8888888888888889, -16),
        "8 / -9"
    );
    assert_eq_div!(NINE, NEG_NINE, NEG_ONE, "9 / -9");
    assert_eq_div!(PI, NEG_PI, Dec64::new(-10000000000000000, -16), "PI / -PI");
    assert_eq_div!(NEG_PI, PI, Dec64::new(-10000000000000000, -16), "-PI / PI");
    assert_eq_div!(
        NEG_PI,
        NEG_PI,
        Dec64::new(10000000000000000, -16),
        "-PI / -PI"
    );
    assert_eq_div!(Dec64::new(-16, 0), TEN, Dec64::new(-16, -1), "-16 / 10");
    assert_eq_div!(
        MAXINT,
        EPSILON,
        Dec64::new(36028797018963967, 16),
        "MAXINT / EPSILON"
    );
    assert_eq_div!(ONE, MAXINT, FRAC_1_MAXINT, "1 / MAXINT");
    assert_eq_div!(ONE, FRAC_1_MAXINT, MAXINT, "1 / 1/MAXINT");
    assert_eq_div!(
        ONE,
        MININT,
        Dec64::new(-27755575615628914, -33),
        "ONE / -MAXINT"
    );
    assert_eq_div!(MAX, EPSILON, NAN, "MAX / EPSILON");
    assert_eq_div!(MAX, MAX, Dec64::new(10000000000000000, -16), "MAX / MAX");
    assert_eq_div!(
        Dec64::new(10, -1),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 1"
    );
    assert_eq_div!(
        Dec64::new(100, -2),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 2"
    );
    assert_eq_div!(
        Dec64::new(1000, -3),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 3"
    );
    assert_eq_div!(
        Dec64::new(10000, -4),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 4"
    );
    assert_eq_div!(
        Dec64::new(100000, -5),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 5"
    );
    assert_eq_div!(
        Dec64::new(1000000, -6),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 6"
    );
    assert_eq_div!(
        Dec64::new(10000000, -7),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 7"
    );
    assert_eq_div!(
        Dec64::new(100000000, -8),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 8"
    );
    assert_eq_div!(
        Dec64::new(1000000000, -9),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 9"
    );
    assert_eq_div!(
        Dec64::new(10000000000, -10),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 10"
    );
    assert_eq_div!(
        Dec64::new(100000000000, -11),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 11"
    );
    assert_eq_div!(
        Dec64::new(1000000000000, -12),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 12"
    );
    assert_eq_div!(
        Dec64::new(10000000000000, -13),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 13"
    );
    assert_eq_div!(
        Dec64::new(100000000000000, -14),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 14"
    );
    assert_eq_div!(
        Dec64::new(1000000000000000, -15),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 15"
    );
    assert_eq_div!(
        Dec64::new(10000000000000000, -16),
        MAXINT,
        FRAC_1_MAXINT,
        "ONE / MAXINT alias 16"
    );
    assert_eq_div!(
        ONE,
        Dec64::from_raw(0x1437EEECD800000),
        Dec64::new(28114572543455208, -31),
        "1/17!"
    );
    assert_eq_div!(
        ONE,
        Dec64::from_raw(0x52D09F700003),
        Dec64::new(28114572543455208, -31),
        "1/17!"
    );
}
