#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn zero() {
    let expect = ZERO;
    for i in -256..256 {
        let result = Dec64::new(0, i);

        assert!(result.is_zero());
        assert_eq!(result.coefficient(), 0);
        assert_eq!(result.exponent(), 0);
        assert_eq!(result, expect, "@ exponent: {}", i);
    }
}

#[test]
fn one() {
    let one_normal = Dec64::new(1, 0);
    let one_low = Dec64::new(10000000000000000, -16);
    let expect = Dec64::from_parts(1, 0);
    assert_eq!(one_normal, expect, "@ normal");
    println!(
        "{:?} {} {}",
        (one_low - expect),
        (one_low - expect).is_zero(),
        one_low == expect
    );
    assert_eq!(one_low, expect, "@ low");
}

#[test]
fn min_coefficient() {
    let coefficient = dec64::MIN_COEFFICIENT;
    let exponent = 0;
    let expect = Dec64::from_parts(coefficient, exponent);
    let result = Dec64::new(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), exponent);
    assert_eq!(result, expect);
}

#[test]
fn max_coefficient() {
    let coefficient = MAX_COEFFICIENT;
    let exponent = 0;
    let expect = Dec64::from_parts(coefficient, exponent);
    let result = Dec64::new(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), exponent);
    assert_eq!(result, expect);
}

#[test]
fn min() {
    let coefficient = MIN_COEFFICIENT;
    let exponent = MAX_EXP;
    let expect = MIN;
    let result = Dec64::new(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), { exponent });
    assert_eq!(result, expect);
}

#[test]
fn max() {
    let coefficient = MAX_COEFFICIENT;
    let exponent = MAX_EXP;
    let expect = MAX;
    let result = Dec64::new(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), { exponent });
    assert_eq!(result, expect);
}

#[test]
fn min_minus_one() {
    let coefficient = MIN_COEFFICIENT - 1;
    let exponent = MAX_EXP;
    let expect = NAN;
    let result = Dec64::new(coefficient, exponent as i32);

    assert!(result.is_nan());
    assert_eq!(result, expect);
}

#[test]
fn max_plus_one() {
    let coefficient = MAX_COEFFICIENT + 1;
    let exponent = MAX_EXP;
    let expect = NAN;
    let result = Dec64::new(coefficient, exponent as i32);

    assert!(result.is_nan());
    assert_eq!(result, expect);
}

#[test]
fn reduce_exp() {
    let coefficient = 36_028_797_018_963;
    let exponent = 130;
    let expect_coefficient = coefficient * 1000;
    let expect_exponent = MAX_EXP;
    let expect = Dec64::from_parts(expect_coefficient, expect_exponent);
    let result = Dec64::new(coefficient, exponent);

    assert_eq!(result.coefficient(), expect_coefficient);
    assert_eq!(result.exponent(), { expect_exponent });
    assert_eq!(result, expect);
}

#[test]
fn reduce_exp_too_big() {
    let coefficient = 36_028_797_018_964;
    let exponent = 130;
    let expect = NAN;
    let result = Dec64::new(coefficient, exponent);

    assert!(result.is_nan());
    assert_eq!(result, expect);
}

#[test]
fn increase_exp() {
    let coefficient = 1_000;
    let exponent = -130;
    let expect_coefficient = coefficient / 1000;
    let expect_exponent = MIN_EXP;
    let expect = Dec64::from_parts(expect_coefficient, expect_exponent);
    let result = Dec64::new(coefficient, exponent);

    assert_eq!(result.coefficient(), expect_coefficient);
    assert_eq!(result.exponent(), { expect_exponent });
    assert_eq!(result, expect);
}

#[test]
fn increase_exp_too_small() {
    let coefficient = 100;
    let exponent = -130;
    let expect = ZERO;
    let result = Dec64::new(coefficient, exponent);

    assert!(result.is_zero());
    assert_eq!(result, expect);
}
