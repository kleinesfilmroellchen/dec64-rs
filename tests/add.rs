#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn zero() {
    assert_eq!(ZERO + ZERO, ZERO);

    assert_eq!(ZERO + ONE, ONE);
    assert_eq!(ONE + ZERO, ONE);

    assert_eq!(ZERO + MAX, MAX);
    assert_eq!(MAX + ZERO, MAX);

    assert_eq!(ZERO + MIN, MIN);
    assert_eq!(MIN + ZERO, MIN);
}

#[test]
fn zip() {
    assert_eq!(ZERO + ZIP, ZERO);
    assert_eq!(ZIP + ZERO, ZERO);
    assert_eq!(ZIP + ZIP, ZERO);

    assert_eq!(ZIP + ONE, ONE);
    assert_eq!(ONE + ZIP, ONE);

    assert_eq!(ZIP + MAX, MAX);
    assert_eq!(MAX + ZIP, MAX);

    assert_eq!(ZIP + MIN, MIN);
    assert_eq!(MIN + ZIP, MIN);
}

#[test]
fn trivial() {
    assert_eq!(ONE + ONE, TWO);
    assert_eq!(ONE + NEG_ONE, ZERO);
    assert_eq!(NEG_ONE + ONE, ZERO);
}

#[test]
fn same_neg() {
    assert_eq!(NEG_ONE + NEG_ONE, NEG_TWO);
    assert_eq!(NEG_TWO + NEG_TWO, NEG_FOUR);
    assert_eq!(NEG_FOUR + NEG_FOUR, NEG_EIGHT);
}

#[test]
fn nan() {
    assert_eq!(NAN_NAN + NAN_NAN, NAN);

    assert_eq!(NAN_NAN + ZERO, NAN);
    assert_eq!(ZERO + NAN_NAN, NAN);

    assert_eq!(NAN_NAN + ZIP, NAN);
    assert_eq!(ZIP + NAN_NAN, NAN);

    assert_eq!(NAN_NAN + ONE, NAN);
    assert_eq!(ONE + NAN_NAN, NAN);

    assert_eq!(NAN_NAN + MAX, NAN);
    assert_eq!(MAX + NAN_NAN, NAN);

    assert_eq!(NAN_NAN + MIN, NAN);
    assert_eq!(MIN + NAN_NAN, NAN);
}

#[test]
fn neg_tenth() {
    assert_eq!(NEG_TENTH + NEG_TENTH, NEG_FIFTH);
}

#[test]
fn range_overflow() {
    assert_eq!(MAX + MAX, NAN);
    assert_eq!(MIN + MIN, NAN);
}

#[test]
fn minmax() {
    let expect = Dec64::from_parts(-1, 127);

    assert_eq!(MAX + MIN, expect);
    assert_eq!(MIN + MAX, expect);
}

#[test]
fn positive_integer_overflow() {
    let ten = TEN;
    let value = Dec64::from_parts(36028797018963960, 0);
    let expect = Dec64::from_parts(3602879701896397, 1);
    let result = value + ten;

    assert_eq!(result, expect);
}

#[test]
fn negative_integer_overflow() {
    let ten = NEG_TEN;
    let value = Dec64::from_parts(-36028797018963960, 0);
    let expect = Dec64::from_parts(-3602879701896397, 1);
    let result = value + ten;

    assert_eq!(result, expect);
}

#[test]
fn with_ZERO_exponent() {
    let coefficient_a = 333;
    let coefficient_b = 222;
    let exponent = 0;
    let a = Dec64::from_parts(coefficient_a, exponent);
    let b = Dec64::from_parts(coefficient_b, exponent);
    let expect = Dec64::from_parts(coefficient_a + coefficient_b, exponent);
    let result = a + b;

    assert_eq!(result.coefficient(), coefficient_a + coefficient_b);
    assert_eq!(result.exponent(), exponent);
    assert_eq!(result, expect);
}

#[test]
fn with_same_non_zero_exponent() {
    let coefficient_a = 333;
    let coefficient_b = 222;
    let exponent = 11;
    let a = Dec64::from_parts(coefficient_a, exponent);
    let b = Dec64::from_parts(coefficient_b, exponent);
    let expect = Dec64::from_parts(coefficient_a + coefficient_b, exponent);
    let result = a + b;

    assert_eq!(result.coefficient(), coefficient_a + coefficient_b);
    assert_eq!(result.exponent(), exponent);
    assert_eq!(result, expect);
}

#[test]
fn with_reducable_exponent() {
    let coefficient_a = 333;
    let coefficient_b = 222;
    let exponent_a = 20;
    let exponent_b = 10;
    let a = Dec64::from_parts(coefficient_a, exponent_a);
    let b = Dec64::from_parts(coefficient_b, exponent_b);
    let expect = Dec64::from_parts(coefficient_a * 10_000_000_000 + coefficient_b, exponent_b);
    let result = a + b;

    assert_eq!(result, expect);
}

#[test]
fn with_significance_loss() {
    let coefficient_a = 333;
    let coefficient_b = 222;
    let exponent_a = 26;
    let exponent_b = 10;
    let a = Dec64::from_parts(coefficient_a, exponent_a);
    let b = Dec64::from_parts(coefficient_b, exponent_b);
    let expect = Dec64::from_parts(
        coefficient_a * 100_000_000_000_000 + coefficient_b / 100,
        exponent_b + 2,
    );
    let result = a + b;

    assert_eq!(result, expect);
}

#[test]
fn double_max_coefficient() {
    let value = Dec64::from_parts(MAX_COEFFICIENT, 64);
    let expect = Dec64::from_parts(MAX_COEFFICIENT * 2 / 10, 65);

    assert_eq!(value + value, expect);
}

#[test]
fn double_min_coefficient() {
    let value = Dec64::from_parts(MIN_COEFFICIENT, 64);
    // min coefficient is rounded down when scaling exponent since -1.
    let expect = Dec64::from_parts(MIN_COEFFICIENT * 2 / 10 - 1, 65);

    assert_eq!(value + value, expect);
}

macro_rules! assert_eq_add {
    ($first:expr, $second:expr, $result:expr, $msg:expr) => {
        assert_eq!($first + $second, $result, $msg)
    };
}

#[test]
fn all_c() {
    let maxint_plus: Dec64 = MAXINT + ONE;

    assert_eq_add!(NAN, ZERO, NAN, "NAN + ZERO");
    assert_eq_add!(NAN, NAN, NAN, "NAN + NAN");
    assert_eq_add!(NAN_NAN, ONE, NAN, "NAN_NAN + 1");
    assert_eq_add!(NAN_NAN, NAN_NAN, NAN, "NAN_NAN + NAN_NAN");
    assert_eq_add!(ZERO, NAN_NAN, NAN, "0 + NAN_NAN");
    assert_eq_add!(ZERO, ZIP, ZERO, "ZERO + ZIP");
    assert_eq_add!(ZIP, ZERO, ZERO, "ZIP + ZERO");
    assert_eq_add!(ZIP, ZIP, ZERO, "ZIP + ZIP");
    assert_eq_add!(ALMOST_ONE, EPSILON, ONE, "ALMOST_ONE + EPSILON");
    assert_eq_add!(ALMOST_ONE, NINE, TEN, "ALMOST_ONE + 1");
    assert_eq_add!(ONE, NAN, NAN, "ONE + NAN");
    assert_eq_add!(ONE, ONE, TWO, "ONE + ONE");
    assert_eq_add!(ONE, CENT, Dec64::from_parts(101, -2), "ONE + cent");
    assert_eq_add!(
        ONE,
        EPSILON,
        Dec64::from_parts(10000000000000001, -16),
        "1 + EPSILON"
    );
    assert_eq_add!(THREE, FOUR, SEVEN, "three + four");
    assert_eq_add!(
        FOUR,
        EPSILON,
        Dec64::from_parts(4000000000000000, -15),
        "4 + EPSILON"
    );
    assert_eq_add!(
        Dec64::from_parts(1, 2),
        Dec64::from_parts(-1, -2),
        Dec64::from_parts(9999, -2),
        "100 - 0.01"
    );
    assert_eq_add!(
        Dec64::from_parts(10, 10),
        Dec64::from_parts(20, 10),
        Dec64::from_parts(30, 10),
        "10e10 + 20e10"
    );
    assert_eq_add!(
        Dec64::from_parts(199, -2),
        Dec64::from_parts(299, -2),
        Dec64::from_parts(498, -2),
        "1.99 + 2.99"
    );
    assert_eq_add!(
        Dec64::from_parts(36028797018963967, 126),
        Dec64::from_parts(36028797018963967, 126),
        Dec64::from_parts(7205759403792793, 127),
        "test overflow with big exponents"
    );
    assert_eq_add!(
        Dec64::from_parts(9999999999999999, 0),
        ONE,
        Dec64::from_parts(10000000000000000, 0),
        "9999999999999999 + 1"
    );
    assert_eq_add!(NEG_ONE, EPSILON, ALMOST_NEG_ONE, "-1 + EPSILON");
    assert_eq_add!(NEG_PI, PI, ZERO, "-pi + pi");
    assert_eq_add!(MAXINT, ONE, maxint_plus, "MAXINT + ONE");
    assert_eq_add!(MAXINT, HALF, maxint_plus, "MAXINT + half");
    assert_eq_add!(MAXINT, CENT, MAXINT, "MAXINT + cent");
    assert_eq_add!(
        MAXINT,
        Dec64::from_parts(4999999999, -11),
        MAXINT,
        "MAXINT + 0.4999999999"
    );
    assert_eq_add!(
        MAXINT,
        MAXINT,
        Dec64::from_parts(7205759403792793, 1),
        "MAXINT + MAXINT"
    );
    assert_eq_add!(
        MAXINT,
        Dec64::from_parts(111, -2),
        maxint_plus,
        "MAXINT + 1.11"
    );
    assert_eq_add!(
        MAXINT,
        Dec64::from_parts(36028797018963967, -20),
        MAXINT,
        "MAXINT + something too small"
    );
    assert_eq_add!(
        MAXINT,
        Dec64::from_parts(30000000000000000, -16),
        maxint_plus,
        "MAXINT + 3"
    );
    assert_eq_add!(
        MAXINT,
        Dec64::from_parts(20000000000000000, -16),
        maxint_plus,
        "MAXINT + something too small"
    );
    assert_eq_add!(
        MAXINT,
        MININT,
        Dec64::from_parts(-1, 0),
        "MAXINT + MININT"
    );
    assert_eq_add!(MAX, Dec64::from_parts(1, -127), MAX, "insignificance");
    assert_eq_add!(MAX, ONE, MAX, "insignificance");
    assert_eq_add!(MAX, MAXINT, MAX, "insignificance");
    assert_eq_add!(MAX, Dec64::from_parts(1, 127), NAN, "overflow the exponent");
    assert_eq_add!(
        MAX,
        Dec64::from_parts(10, 126),
        NAN,
        "overflow the exponent"
    );
    assert_eq_add!(
        MAX,
        Dec64::from_parts(100, 125),
        NAN,
        "overflow the exponent"
    );
    assert_eq_add!(
        MAX,
        Dec64::from_parts(1000, 124),
        NAN,
        "overflow the exponent"
    );
    assert_eq_add!(
        MAX,
        Dec64::from_parts(500, 124),
        NAN,
        "overflow the exponent"
    );
    assert_eq_add!(MAX, MAX, NAN, "overflow the exponent");
    assert_eq_add!(
        MAX,
        Dec64::from_parts(-36028797018963967, 127),
        ZERO,
        "extreme ZERO"
    );
    assert_eq_add!(ALMOST_NEG_ONE, ONE, EPSILON, "ALMOST_NEG_ONE + ONE");
    assert_eq_add!(
        ALMOST_NEG_ONE,
        ALMOST_ONE,
        ZERO,
        "ALMOST_NEG_ONE + ALMOST_ONE"
    );
    assert_eq_add!(
        Dec64::from_parts(1, -1),
        Dec64::from_parts(1, -3),
        Dec64::from_parts(101, -3),
        "0.1 + 0.001"
    );
    assert_eq_add!(
        Dec64::from_parts(1, -1),
        Dec64::from_parts(1, -17),
        Dec64::from_parts(10000000000000001, -17),
        "0.1 + 1e-16"
    );
    assert_eq_add!(
        Dec64::from_parts(7182818284590704, -16),
        ONE,
        Dec64::from_parts(17182818284590704, -16),
        "7182818284590704e-16 + 1"
    );
    assert_eq_add!(
        Dec64::from_parts(7182818284590704, -16),
        Dec64::from_parts(10, -1),
        Dec64::from_parts(17182818284590704, -16),
        "7182818284590704e-16 + 10e-1"
    );
    assert_eq_add!(
        Dec64::from_parts(4000000000000000, -16),
        Dec64::from_parts(10, -1),
        Dec64::from_parts(14000000000000000, -16),
        "4000000000000000e-16 + 10e-1"
    );
    assert_eq_add!(
        Dec64::from_parts(1, -1),
        Dec64::from_parts(2, -1),
        Dec64::from_parts(3, -1),
        "0.1 + 0.2"
    );
}
