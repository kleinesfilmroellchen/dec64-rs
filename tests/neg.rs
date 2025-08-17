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
    assert_eq!(-ONE, NEG_ONE);
    assert_eq!(-TWO, NEG_TWO);
    assert_eq!(-TEN, NEG_TEN);
    assert_eq!(-PI, NEG_PI);
}

#[test]
fn trivial_neg() {
    assert_eq!(-NEG_ONE, ONE);
    assert_eq!(-NEG_TWO, TWO);
    assert_eq!(-NEG_TEN, TEN);
    assert_eq!(-NEG_PI, PI);
}

#[test]
fn nan() {
    assert_eq!(-NAN, NAN);
    assert_eq!(-NAN_NAN, NAN);
}

#[test]
fn tenth() {
    assert_eq!(-TENTH, NEG_TENTH);
}

#[test]
fn neg_tenth() {
    assert_eq!(-NEG_TENTH, TENTH);
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
