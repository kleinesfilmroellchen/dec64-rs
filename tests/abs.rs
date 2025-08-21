#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn all_abs() {
    const NONNORMAL_NAN: Dec64 = Dec64::from_raw(128);
    const ZIP: Dec64 = Dec64::from_raw(250);

    assert_eq!(NAN.abs(), NAN, "nan");
    assert_eq!(NONNORMAL_NAN.abs(), NAN, "nonnan");
    assert_eq!(ZERO.abs(), ZERO, "zero");
    assert_eq!(ZIP.abs(), ZERO, "zip");
    assert_eq!(Dec64::from_raw(100).abs(), ZERO, "zero alias");
    assert_eq!(ONE.abs(), ONE, "one");
    assert_eq!(NEGATIVE_ONE.abs(), ONE, "-1");
    assert_eq!(ALMOST_NEGATIVE_ONE.abs(), ALMOST_ONE, "almost_negative_one");
    assert_eq!(MININT.abs(), MAXINT + ONE, "-maxint");
    assert_eq!(MIN.abs(), NAN, "-maxnum");
    assert_eq!(MAX.abs(), MAX, "maxnum");
}
