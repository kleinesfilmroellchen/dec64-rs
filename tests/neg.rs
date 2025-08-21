#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn zero() {
    assert_eq!(-ZERO, ZERO);
}

#[test]
fn zip() {
    assert_eq!(-ZIP, ZERO);
}

#[test]
fn trivial() {
    assert_eq!(-ONE, NEGATIVE_ONE);
    assert_eq!(-TWO, NEGATIVE_TWO);
    assert_eq!(-TEN, NEGATIVE_TEN);
    assert_eq!(-PI, NEGATIVE_PI);
}

#[test]
fn trivial_neg() {
    assert_eq!(-NEGATIVE_ONE, ONE);
    assert_eq!(-NEGATIVE_TWO, TWO);
    assert_eq!(-NEGATIVE_TEN, TEN);
    assert_eq!(-NEGATIVE_PI, PI);
}

#[test]
fn nan() {
    assert_eq!(-NAN, NAN);
    assert_eq!(-NONNORMAL_NAN, NAN);
}

#[test]
fn tenth() {
    assert_eq!(-TENTH, NEGATIVE_TENTH);
}

#[test]
fn neg_tenth() {
    assert_eq!(-NEGATIVE_TENTH, TENTH);
}

#[test]
fn max() {
    let expect = Dec64::from_parts(-MAX_COEFFICIENT, MAX_EXP);
    assert_eq!(-MAX, expect);
}

#[test]
fn min() {
    assert_eq!(-MIN, NAN);
}

#[test]
fn maxint() {
    let expect = Dec64::from_parts(-MAX_COEFFICIENT, 0);
    assert_eq!(-MAXINT, expect);
}

#[test]
fn minint() {
    // this will round up since last digit of MIN_COEFFICIENT is 6
    let expect = Dec64::from_parts(-MIN_COEFFICIENT / 10 + 1, 1);
    assert_eq!(-MININT, expect);
}
