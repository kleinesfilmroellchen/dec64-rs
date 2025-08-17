#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn add_zero() {
    assert_eq!(ZERO + ZERO, ZERO);

    assert_eq!(ZERO + ONE, ONE);
    assert_eq!(ONE + ZERO, ONE);

    assert_eq!(ZERO + MAX, MAX);
    assert_eq!(MAX + ZERO, MAX);

    assert_eq!(ZERO + MIN, MIN);
    assert_eq!(MIN + ZERO, MIN);
}

#[test]
fn add_zip() {
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
fn add_trivial() {
    assert_eq!(ONE + ONE, TWO);
    assert_eq!(ONE + NEG_ONE, ZERO);
    assert_eq!(NEG_ONE + ONE, ZERO);
}

#[test]
fn add_same_neg() {
    assert_eq!(NEG_ONE + NEG_ONE, NEG_TWO);
    assert_eq!(NEG_TWO + NEG_TWO, NEG_FOUR);
    assert_eq!(NEG_FOUR + NEG_FOUR, NEG_EIGHT);
}

#[test]
fn add_nan() {
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
fn add_neg_tenth() {
    assert_eq!(NEG_TENTH + NEG_TENTH, NEG_FIFTH);
}

#[test]
fn add_range_overflow() {
    assert_eq!(MAX + MAX, NAN);
    assert_eq!(MIN + MIN, NAN);
}

#[test]
fn add_minmax() {
    let expect = Dec64::from_parts(-1, 127);

    assert_eq!(MAX + MIN, expect);
    assert_eq!(MIN + MAX, expect);
}

#[test]
fn add_positive_integer_overflow() {
    let ten = TEN;
    let value = Dec64::from_parts(36028797018963960, 0);
    let expect = Dec64::from_parts(3602879701896397, 1);
    let result = value + ten;

    assert_eq!(result, expect);
}

#[test]
fn add_negative_integer_overflow() {
    let ten = NEG_TEN;
    let value = Dec64::from_parts(-36028797018963960, 0);
    let expect = Dec64::from_parts(-3602879701896397, 1);
    let result = value + ten;

    assert_eq!(result, expect);
}

#[test]
fn add_with_zero_exponent() {
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
fn add_with_same_non_zero_exponent() {
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
fn add_with_reducable_exponent() {
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
fn add_with_significance_loss() {
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
fn add_double_max_coefficient() {
    let value = Dec64::from_parts(MAX_COEFFICIENT, 64);
    let expect = Dec64::from_parts(MAX_COEFFICIENT * 2 / 10, 65);

    assert_eq!(value + value, expect);
}

#[test]
fn add_double_min_coefficient() {
    let value = Dec64::from_parts(MIN_COEFFICIENT, 64);
    // min coefficient is rounded down when scaling exponent since -1.
    let expect = Dec64::from_parts(MIN_COEFFICIENT * 2 / 10 - 1, 65);

    assert_eq!(value + value, expect);
}
