#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn neg_zero() {
    assert_eq!(-ZERO, ZERO);
}

#[test]
fn neg_zip() {
    assert_eq!(-ZIP, ZERO);
}

#[test]
fn neg_trivial() {
    assert_eq!(-ONE, NEG_ONE);
    assert_eq!(-TWO, NEG_TWO);
    assert_eq!(-TEN, NEG_TEN);
    assert_eq!(-PI, NEG_PI);
}

#[test]
fn neg_trivial_neg() {
    assert_eq!(-NEG_ONE, ONE);
    assert_eq!(-NEG_TWO, TWO);
    assert_eq!(-NEG_TEN, TEN);
    assert_eq!(-NEG_PI, PI);
}

#[test]
fn neg_nan() {
    assert_eq!(-NAN, NAN);
    assert_eq!(-NAN_NAN, NAN);
}

#[test]
fn neg_tenth() {
    assert_eq!(-TENTH, NEG_TENTH);
}

#[test]
fn neg_neg_tenth() {
    assert_eq!(-NEG_TENTH, TENTH);
}

#[test]
fn neg_max() {
    let expect = Dec64::from_parts(-MAX_COEFFICIENT, MAX_EXP as i8);
    assert_eq!(-MAX, expect);
}

#[test]
fn neg_min() {
    assert_eq!(-MIN, NAN);
}

#[test]
fn neg_maxint() {
    let expect = Dec64::from_parts(-MAX_COEFFICIENT, 0);
    assert_eq!(-MAXINT, expect);
}

#[test]
fn neg_minint() {
    // this will round up since last digit of MIN_COEFFICIENT is 6
    let expect = Dec64::from_parts(-MIN_COEFFICIENT / 10 + 1, 1);
    assert_eq!(-MININT, expect);
}
