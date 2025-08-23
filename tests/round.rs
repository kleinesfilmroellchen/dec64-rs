#![allow(clippy::wildcard_imports)]

use dec64::Dec64;
use dec64::consts::*;

macro_rules! assert_eq_round {
    ($val:expr, $places:expr, $result:expr, $msg:expr) => {
        assert_eq!($val.round_to_places($places), $result, $msg)
    };
}

#[test]
fn all_c() {
    // Discrepancy from C implementation: Rounding by a non-integer makes no sense, so these cases are all dropped.
    assert_eq_round!(NAN, 0, NAN, "NAN");
    assert_eq_round!(NONNORMAL_NAN, 0, NAN, "NONNORMAL_NAN 0");
    assert_eq_round!(PI, 0, THREE, "PI");
    assert_eq_round!(ZERO, 0, ZERO, "ZERO ZIP");
    assert_eq_round!(ZIP, 0, ZERO, "ZIP ZERO");
    assert_eq_round!(PI, -18, PI, "PI -18");
    assert_eq_round!(PI, -16, Dec64::new(31415926535897932, -16), "PI -16");
    assert_eq_round!(PI, -15, Dec64::new(3141592653589793, -15), "PI -15");
    assert_eq_round!(PI, -14, Dec64::new(314159265358979, -14), "PI -14");
    assert_eq_round!(PI, -13, Dec64::new(31415926535898, -13), "PI -13");
    assert_eq_round!(PI, -12, Dec64::new(314159265359, -11), "PI -12");
    assert_eq_round!(PI, -11, Dec64::new(314159265359, -11), "PI -11");
    assert_eq_round!(PI, -10, Dec64::new(31415926536, -10), "PI -10");
    assert_eq_round!(PI, -9, Dec64::new(3141592654, -9), "PI -9");
    assert_eq_round!(PI, -8, Dec64::new(314159265, -8), "PI -8");
    assert_eq_round!(PI, -7, Dec64::new(31415927, -7), "PI -7");
    assert_eq_round!(PI, -6, Dec64::new(3141593, -6), "PI -6");
    assert_eq_round!(PI, -5, Dec64::new(314159, -5), "PI -5");
    assert_eq_round!(PI, -4, Dec64::new(31416, -4), "PI -4");
    assert_eq_round!(PI, -3, Dec64::new(3142, -3), "PI -3");
    assert_eq_round!(PI, -2, Dec64::new(314, -2), "PI -2");
    assert_eq_round!(PI, -1, Dec64::new(31, -1), "PI -1");
    assert_eq_round!(PI, 0, THREE, "PI 0");
    assert_eq_round!(NEGATIVE_PI, -18, NEGATIVE_PI, "-PI -18");
    assert_eq_round!(NEGATIVE_PI, -17, NEGATIVE_PI, "-PI -17");
    assert_eq_round!(
        NEGATIVE_PI,
        -16,
        Dec64::new(-31415926535897932, -16),
        "-PI -16"
    );
    assert_eq_round!(
        NEGATIVE_PI,
        -15,
        Dec64::new(-3141592653589793, -15),
        "-PI -15"
    );
    assert_eq_round!(
        NEGATIVE_PI,
        -14,
        Dec64::new(-314159265358979, -14),
        "-PI -14"
    );
    assert_eq_round!(
        NEGATIVE_PI,
        -13,
        Dec64::new(-31415926535898, -13),
        "-PI -13"
    );
    assert_eq_round!(NEGATIVE_PI, -12, Dec64::new(-314159265359, -11), "-PI -12");
    assert_eq_round!(NEGATIVE_PI, -11, Dec64::new(-314159265359, -11), "-PI -11");
    assert_eq_round!(NEGATIVE_PI, -10, Dec64::new(-31415926536, -10), "-PI -10");
    assert_eq_round!(NEGATIVE_PI, -9, Dec64::new(-3141592654, -9), "-PI -9");
    assert_eq_round!(NEGATIVE_PI, -8, Dec64::new(-314159265, -8), "-PI -8");
    assert_eq_round!(NEGATIVE_PI, -7, Dec64::new(-31415927, -7), "-PI -7");
    assert_eq_round!(NEGATIVE_PI, -6, Dec64::new(-3141593, -6), "-PI -6");
    assert_eq_round!(NEGATIVE_PI, -5, Dec64::new(-314159, -5), "-PI -5");
    assert_eq_round!(NEGATIVE_PI, -4, Dec64::new(-31416, -4), "-PI -4");
    assert_eq_round!(NEGATIVE_PI, -3, Dec64::new(-3142, -3), "-PI -3");
    assert_eq_round!(NEGATIVE_PI, -2, Dec64::new(-314, -2), "-PI -2");
    assert_eq_round!(NEGATIVE_PI, -1, Dec64::new(-31, -1), "-PI -1");
    assert_eq_round!(NEGATIVE_PI, 0, NEGATIVE_THREE, "-PI 0");
    assert_eq_round!(Dec64::new(449, -2), -2, Dec64::new(449, -2), "4.49 -2");
    assert_eq_round!(Dec64::new(449, -2), -1, Dec64::new(45, -1), "4.49 -1");
    assert_eq_round!(Dec64::new(449, -2), 0, FOUR, "4.49 0");
    assert_eq_round!(Dec64::new(450, -2), 0, FIVE, "4.50 0");
    assert_eq_round!(Dec64::new(-449, -2), -2, Dec64::new(-449, -2), "-4.49 -2");
    assert_eq_round!(Dec64::new(-449, -2), -1, Dec64::new(-45, -1), "-4.49 -1");
    assert_eq_round!(Dec64::new(-449, -2), 0, NEGATIVE_FOUR, "-4.49 0");
    assert_eq_round!(Dec64::new(-450, -2), 0, NEGATIVE_FIVE, "-4.50 0");
    assert_eq_round!(MAXINT, -1, MAXINT, "MAXINT -1");
    assert_eq_round!(MAXINT, 0, MAXINT, "MAXINT 0");
    assert_eq_round!(MAXINT, 1, Dec64::new(3602879701896397, 1), "MAXINT 1");
    assert_eq_round!(MAXINT, 2, Dec64::new(3602879701896400, 1), "MAXINT 2");
    assert_eq_round!(MAXINT, 3, Dec64::new(3602879701896400, 1), "MAXINT 3");
    assert_eq_round!(MAXINT, 4, Dec64::from(36028797018960000i64), "MAXINT 4");
    assert_eq_round!(MAXINT, 5, Dec64::from(36028797019000000i64), "MAXINT 5");
    assert_eq_round!(MAXINT, 6, Dec64::from(36028797019000000i64), "MAXINT 6");
    assert_eq_round!(MAXINT, 7, Dec64::from(36028797020000000i64), "MAXINT 7");
    assert_eq_round!(MAXINT, 8, Dec64::from(36028797000000000i64), "MAXINT 8");
    assert_eq_round!(MAXINT, 9, Dec64::from(36028797000000000i64), "MAXINT 9");
    assert_eq_round!(MAXINT, 10, Dec64::from(36028800000000000i64), "MAXINT 10");
    assert_eq_round!(MAXINT, 11, Dec64::from(36028800000000000i64), "MAXINT 11");
    assert_eq_round!(MAXINT, 12, Dec64::from(36029000000000000i64), "MAXINT 12");
    assert_eq_round!(MAXINT, 13, Dec64::from(36030000000000000i64), "MAXINT 13");
    assert_eq_round!(MAXINT, 14, Dec64::from(36000000000000000i64), "MAXINT 14");
    assert_eq_round!(MAXINT, 15, Dec64::from(36000000000000000i64), "MAXINT 15");
    assert_eq_round!(MAXINT, 16, Dec64::from(40000000000000000i64), "MAXINT 16");
    assert_eq_round!(MAXINT, 17, ZERO, "MAXINT 17");
    assert_eq_round!(
        Dec64::from(34999999999999999i64),
        0,
        Dec64::from(34999999999999999i64),
        "34999999999999999 0"
    );
    assert_eq_round!(
        Dec64::from(34999999999999995i64),
        1,
        Dec64::from(35000000000000000i64),
        "34999999999999995 1"
    );
    assert_eq_round!(
        Dec64::from(34999999999999994i64),
        1,
        Dec64::from(34999999999999990i64),
        "34999999999999994 1"
    );
    assert_eq_round!(
        Dec64::from(34999999999999950i64),
        2,
        Dec64::from(35000000000000000i64),
        "34999999999999950 2"
    );
    assert_eq_round!(
        Dec64::from(34999999999999949i64),
        2,
        Dec64::from(34999999999999900i64),
        "34999999999999949 2"
    );
    assert_eq_round!(
        Dec64::from(34999999999999500i64),
        3,
        Dec64::from(35000000000000000i64),
        "34999999999999500 3"
    );
    assert_eq_round!(
        Dec64::from(34999999999999499i64),
        3,
        Dec64::from(34999999999999000i64),
        "34999999999999499 3"
    );
    assert_eq_round!(
        Dec64::from(34999999999995000i64),
        4,
        Dec64::from(35000000000000000i64),
        "34999999999995000 4"
    );
    assert_eq_round!(
        Dec64::from(34999999999994999i64),
        4,
        Dec64::from(34999999999990000i64),
        "34999999999994999 4"
    );
    assert_eq_round!(
        Dec64::from(34999999999950000i64),
        5,
        Dec64::from(35000000000000000i64),
        "34999999999950000 5"
    );
    assert_eq_round!(
        Dec64::from(34999999999949999i64),
        5,
        Dec64::from(34999999999900000i64),
        "34999999999949999 5"
    );
    assert_eq_round!(
        Dec64::from(34999999999500000i64),
        6,
        Dec64::from(35000000000000000i64),
        "34999999999500000 6"
    );
    assert_eq_round!(
        Dec64::from(34999999999499999i64),
        6,
        Dec64::from(34999999999000000i64),
        "34999999999499999 6"
    );
    assert_eq_round!(
        Dec64::from(34999999995000000i64),
        7,
        Dec64::from(35000000000000000i64),
        "34999999995000000 7"
    );
    assert_eq_round!(
        Dec64::from(34999999994999999i64),
        7,
        Dec64::from(34999999990000000i64),
        "34999999994999999 7"
    );
    assert_eq_round!(
        Dec64::from(34999999950000000i64),
        8,
        Dec64::from(35000000000000000i64),
        "34999999950000000 8"
    );
    assert_eq_round!(
        Dec64::from(34999999949999999i64),
        8,
        Dec64::from(34999999900000000i64),
        "34999999949999999 8"
    );
    assert_eq_round!(
        Dec64::from(34999999500000000i64),
        9,
        Dec64::from(35000000000000000i64),
        "34999999950000000 9"
    );
    assert_eq_round!(
        Dec64::from(34999999499999999i64),
        9,
        Dec64::from(34999999000000000i64),
        "34999999949999999 9"
    );
    assert_eq_round!(
        Dec64::from(34999995000000000i64),
        10,
        Dec64::from(35000000000000000i64),
        "34999999950000000 10"
    );
    assert_eq_round!(
        Dec64::from(34999994999999999i64),
        10,
        Dec64::from(34999990000000000i64),
        "34999999949999999 10"
    );
    assert_eq_round!(
        Dec64::from(34999950000000000i64),
        11,
        Dec64::from(35000000000000000i64),
        "34999950000000000 11"
    );
    assert_eq_round!(
        Dec64::from(34999949999999999i64),
        11,
        Dec64::from(34999900000000000i64),
        "34999949999999999 11"
    );
    assert_eq_round!(
        Dec64::from(34999500000000000i64),
        12,
        Dec64::from(35000000000000000i64),
        "34999500000000000 12"
    );
    assert_eq_round!(
        Dec64::from(34999499999999999i64),
        12,
        Dec64::from(34999000000000000i64),
        "34999499999999999 12"
    );
    assert_eq_round!(
        Dec64::from(34995000000000000i64),
        13,
        Dec64::from(35000000000000000i64),
        "34995000000000000 13"
    );
    assert_eq_round!(
        Dec64::from(34994999999999999i64),
        13,
        Dec64::from(34990000000000000i64),
        "34994999999999999 13"
    );
    assert_eq_round!(
        Dec64::from(34950000000000000i64),
        14,
        Dec64::from(35000000000000000i64),
        "34950000000000000 14"
    );
    assert_eq_round!(
        Dec64::from(34949999999999999i64),
        14,
        Dec64::from(34900000000000000i64),
        "34949999999999999 14"
    );
    assert_eq_round!(
        Dec64::from(34500000000000000i64),
        15,
        Dec64::from(35000000000000000i64),
        "34500000000000000 15"
    );
    assert_eq_round!(
        Dec64::from(34499999999999999i64),
        15,
        Dec64::from(34000000000000000i64),
        "34499999999999999 15"
    );
    assert_eq_round!(
        Dec64::from(34999999999999999i64),
        16,
        Dec64::from(30000000000000000i64),
        "34999999999999999 16"
    );
    assert_eq_round!(
        Dec64::from(34999999999999999i64),
        17,
        ZERO,
        "34999999999999999 17"
    );
    assert_eq_round!(
        Dec64::from(-34999999999999999i64),
        17,
        ZERO,
        "-34999999999999999 17"
    );
    assert_eq_round!(
        Dec64::from(-34999950000000000i64),
        11,
        Dec64::from(-35000000000000000i64),
        "-34999950000000000 11"
    );
    assert_eq_round!(
        Dec64::from(-34999949999999999i64),
        11,
        Dec64::from(-34999900000000000i64),
        "-34999949999999999 11"
    );
    assert_eq_round!(
        Dec64::from(-34999500000000000i64),
        12,
        Dec64::from(-35000000000000000i64),
        "-34999500000000000 12"
    );
    assert_eq_round!(
        Dec64::from(-34999499999999999i64),
        12,
        Dec64::from(-34999000000000000i64),
        "-34999499999999999 12"
    );
    assert_eq_round!(
        Dec64::from(-34995000000000000i64),
        13,
        Dec64::from(-35000000000000000i64),
        "-34995000000000000 13"
    );
    assert_eq_round!(
        Dec64::from(-34994999999999999i64),
        13,
        Dec64::from(-34990000000000000i64),
        "-34994999999999999 13"
    );
    assert_eq_round!(
        Dec64::from(-34950000000000000i64),
        14,
        Dec64::from(-35000000000000000i64),
        "-34950000000000000 14"
    );
    assert_eq_round!(
        Dec64::from(-34949999999999999i64),
        14,
        Dec64::from(-34900000000000000i64),
        "-34949999999999999 14"
    );
    assert_eq_round!(
        Dec64::from(-34500000000000000i64),
        15,
        Dec64::from(-35000000000000000i64),
        "-34500000000000000 15"
    );
    assert_eq_round!(
        Dec64::from(-34499999999999999i64),
        15,
        Dec64::from(-34000000000000000i64),
        "-34499999999999999 15"
    );
    assert_eq_round!(
        Dec64::from(-34999999999999999i64),
        16,
        Dec64::from(-30000000000000000i64),
        "-34999999999999999 16"
    );
    assert_eq_round!(
        Dec64::from(-34999999999999999i64),
        17,
        ZERO,
        "-34999999999999999 17"
    );
}
