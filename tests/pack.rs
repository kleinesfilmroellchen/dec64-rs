#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn pack_zero() {
    let expect = ZERO;
    for i in -256..256 {
        let result = Dec64::pack(0, i);

        assert!(result.is_zero());
        assert_eq!(result.coefficient(), 0);
        assert_eq!(result.exponent(), 0);
        assert_eq!(result, expect, "@ exponent: {}", i);
    }
}

#[test]
fn pack_one() {
    let one_normal = Dec64::pack(1, 0);
    let one_low = Dec64::pack(10000000000000000, -16);
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
fn pack_min_coefficient() {
    let coefficient = dec64::MIN_COEFFICIENT;
    let exponent = 0;
    let expect = Dec64::from_parts(coefficient, exponent);
    let result = Dec64::pack(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), exponent);
    assert_eq!(result, expect);
}

#[test]
fn pack_max_coefficient() {
    let coefficient = MAX_COEFFICIENT;
    let exponent = 0;
    let expect = Dec64::from_parts(coefficient, exponent);
    let result = Dec64::pack(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), exponent);
    assert_eq!(result, expect);
}

#[test]
fn pack_min() {
    let coefficient = MIN_COEFFICIENT;
    let exponent = MAX_EXP;
    let expect = MIN;
    let result = Dec64::pack(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), exponent as i8);
    assert_eq!(result, expect);
}

#[test]
fn pack_max() {
    let coefficient = MAX_COEFFICIENT;
    let exponent = MAX_EXP;
    let expect = MAX;
    let result = Dec64::pack(coefficient, exponent as i32);

    assert_eq!(result.coefficient(), coefficient);
    assert_eq!(result.exponent(), exponent as i8);
    assert_eq!(result, expect);
}

#[test]
fn pack_min_minus_one() {
    let coefficient = MIN_COEFFICIENT - 1;
    let exponent = MAX_EXP;
    let expect = NAN;
    let result = Dec64::pack(coefficient, exponent as i32);

    assert!(result.is_nan());
    assert_eq!(result, expect);
}

#[test]
fn pack_max_plus_one() {
    let coefficient = MAX_COEFFICIENT + 1;
    let exponent = MAX_EXP;
    let expect = NAN;
    let result = Dec64::pack(coefficient, exponent as i32);

    assert!(result.is_nan());
    assert_eq!(result, expect);
}

#[test]
fn pack_reduce_exp() {
    let coefficient = 36_028_797_018_963;
    let exponent = 130;
    let expect_coefficient = coefficient * 1000;
    let expect_exponent = MAX_EXP;
    let expect = Dec64::from_parts(expect_coefficient, expect_exponent as i8);
    let result = Dec64::pack(coefficient, exponent);

    assert_eq!(result.coefficient(), expect_coefficient);
    assert_eq!(result.exponent(), expect_exponent as i8);
    assert_eq!(result, expect);
}

#[test]
fn pack_reduce_exp_too_big() {
    let coefficient = 36_028_797_018_964;
    let exponent = 130;
    let expect = NAN;
    let result = Dec64::pack(coefficient, exponent);

    assert!(result.is_nan());
    assert_eq!(result, expect);
}

#[test]
fn pack_increase_exp() {
    let coefficient = 1_000;
    let exponent = -130;
    let expect_coefficient = coefficient / 1000;
    let expect_exponent = MIN_EXP;
    let expect = Dec64::from_parts(expect_coefficient, expect_exponent as i8);
    let result = Dec64::pack(coefficient, exponent);

    assert_eq!(result.coefficient(), expect_coefficient);
    assert_eq!(result.exponent(), expect_exponent as i8);
    assert_eq!(result, expect);
}

#[test]
fn pack_increase_exp_too_small() {
    let coefficient = 100;
    let exponent = -130;
    let expect = ZERO;
    let result = Dec64::pack(coefficient, exponent);

    assert!(result.is_zero());
    assert_eq!(result, expect);
}
