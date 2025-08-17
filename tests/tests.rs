#![allow(clippy::wildcard_imports)]

use dec64::consts::*;
use dec64::*;

#[test]
fn roundtrip_usize() {
    let dec = Dec64::from(255_usize);

    let num: usize = dec.into();

    assert_eq!(num, 255_usize);
}

#[test]
fn roundtrip_u8() {
    let dec = Dec64::from(255_u8);

    let num: u8 = dec.into();

    assert_eq!(num, 255_u8);
}

#[test]
fn roundtrip_u16() {
    let dec = Dec64::from(255_u16);

    let num: u16 = dec.into();

    assert_eq!(num, 255_u16);
}

#[test]
fn roundtrip_u32() {
    let dec = Dec64::from(255_u32);

    let num: u32 = dec.into();

    assert_eq!(num, 255_u32);
}

#[test]
fn roundtrip_u64() {
    let dec = Dec64::from(255_u64);

    let num: u64 = dec.into();

    assert_eq!(num, 255_u64);
}

#[test]
fn roundtrip_isize() {
    let dec = Dec64::from(-128_isize);

    let num: isize = dec.into();

    assert_eq!(num, -128_isize);
}

#[test]
fn roundtrip_i8() {
    let dec = Dec64::from(-128_i8);

    let num: i8 = dec.into();

    assert_eq!(num, -128_i8);
}

#[test]
fn roundtrip_i16() {
    let dec = Dec64::from(-128_i16);

    let num: i16 = dec.into();

    assert_eq!(num, -128_i16);
}

#[test]
fn roundtrip_i32() {
    let dec = Dec64::from(-128_i32);

    let num: i32 = dec.into();

    assert_eq!(num, -128_i32);
}

#[test]
fn roundtrip_i64() {
    let dec = Dec64::from(-128_i64);

    let num: i64 = dec.into();

    assert_eq!(num, -128_i64);
}

#[test]
fn roundrtrip_f32() {
    let dec = Dec64::from(3.141592653589793_f32);

    let num: f32 = dec.into();

    assert_eq!(num, 3.141592653589793_f32);
}

#[test]
fn roundrtrip_f64() {
    let dec = Dec64::from(3.141592653589793_f64);

    let num: f64 = dec.into();

    assert_eq!(num, 3.141592653589793_f64);
}

#[test]
fn roundtrip_big_f64() {
    let dec = Dec64::from(1234567890f64);

    let num: f64 = dec.into();

    assert_eq!(num, 1234567890f64);
}

#[test]
fn roundtrip_huge_f64() {
    let dec = Dec64::from(1.23456e100);

    let num: f64 = dec.into();

    assert_eq!(num, 1.23456e100);
}

#[test]
fn roundtrip_tiny_f64() {
    let dec = Dec64::from(1.23456e-100);

    let num: f64 = dec.into();

    assert_eq!(num, 1.23456e-100);
}

#[test]
fn roundtrip_huge_neg_f64() {
    let dec = Dec64::from(-1.23456e100);

    let num: f64 = dec.into();

    assert_eq!(num, -1.23456e100);
}

#[test]
fn roundtrip_tiny_neg_f64() {
    let dec = Dec64::from(-1.23456e-100);

    let num: f64 = dec.into();

    assert_eq!(num, -1.23456e-100);
}

#[test]
fn compose_f64_pi() {
    let dec = Dec64::from_parts(3141592653589793, -15);

    let num: f64 = dec.into();

    assert_eq!(num, 3.141592653589793);
}

#[test]
fn compose_f32_pi() {
    let dec = Dec64::from_parts(3141592653589793, -15);

    let num: f32 = dec.into();

    assert_eq!(num, 3.141592653589793);
}

#[test]
fn compose_max() {
    let dec = Dec64::from_parts(MAX_COEFFICIENT, 127);

    assert_eq!(dec, MAX);
}

#[test]
fn compose_min() {
    let dec = Dec64::from_parts(MIN_COEFFICIENT, 127);

    assert_eq!(dec, MIN);
}

#[test]
fn compose_min_positive() {
    let dec = Dec64::from_parts(1, -127);

    assert_eq!(dec, MIN_POSITIVE);
}

#[test]
fn compose_nan() {
    let nan_normal = Dec64::from_parts(0, -128);
    let nan_subnormal = Dec64::from_parts(42, -128);

    assert!(nan_normal.is_nan());
    assert!(nan_subnormal.is_nan());
    assert_eq!(nan_normal, NAN);
    assert!(nan_subnormal != nan_normal);
}

#[test]
fn compose_zero() {
    let zero_normal = Dec64::from_parts(0, 0);
    let zero_high = Dec64::from_parts(0, 127);
    let zero_low = Dec64::from_parts(0, -127);

    assert!(zero_normal.is_zero());
    assert!(zero_high.is_zero());
    assert!(zero_low.is_zero());
    assert_eq!(zero_normal, ZERO);
    assert_eq!(zero_high, ZERO);
    assert_eq!(zero_low, ZERO);
}

#[test]
fn write_42() {
    let dec = Dec64::from_parts(42, 0);
    let string = dec.to_string();
    assert_eq!(string, "42");
}

#[test]
fn write_pi() {
    let dec = Dec64::from_parts(3141592653589793, -15);
    let string = dec.to_string();
    assert_eq!(string, "3.141592653589793");
}

#[test]
fn write_pi_from_float() {
    let dec = Dec64::from(3.141592653589793);
    let string = dec.to_string();
    assert_eq!(string, "3.141592653589793");
}

#[test]
fn write_midperiod() {
    let dec = Dec64::from_parts(123456, -3);
    let string = dec.to_string();
    assert_eq!(string, "123.456");
}

#[test]
fn write_small() {
    let dec = Dec64::from_parts(1, -5);
    let string = dec.to_string();
    assert_eq!(string, "0.00001");
}

#[test]
fn write_tiny() {
    let dec = Dec64::from_parts(123456, -35);
    let string = dec.to_string();
    assert_eq!(string, "1.23456e-30");
}

#[test]
fn write_huge() {
    let dec = Dec64::from_parts(123456, 30);
    let string = dec.to_string();
    assert_eq!(string, "1.23456e35");
}

/// Same tests as C implementation
#[test]
fn write_c() {
    let nan = NAN; /* not a number */
    let nannan = Dec64::from_raw(32896); /* a non-normal nan */
    let zero = ZERO; /* 0 */
    let zip = Dec64::from_raw(1); /* a non normal 0 */
    let one = ONE; /* 1 */
    let two = Dec64::from_parts(2, 0); /* 2 */
    let three = Dec64::from_parts(3, 0); /* 3 */
    let four = Dec64::from_parts(4, 0); /* 4 */
    let five = Dec64::from_parts(5, 0); /* 5 */
    let six = Dec64::from_parts(6, 0); /* 6 */
    let seven = Dec64::from_parts(7, 0); /* 7 */
    let eight = Dec64::from_parts(8, 0); /* 8 */
    let nine = Dec64::from_parts(9, 0); /* 9 */
    let ten = Dec64::from_parts(10, 0); /* 10 */
    let minnum = Dec64::from_parts(1, -127); /* the smallest possible number */
    let epsilon = Dec64::from_parts(1, -16); /* the smallest number addable to 1 */
    let cent = Dec64::from_parts(1, -2); /* 0.01 */
    let half = Dec64::from_parts(5, -1); /* 0.5 */
    let almost_one = Dec64::from_parts(9999999999999999, -16); /* 0.9999999999999999 */
    let pi = Dec64::from_parts(31415926535897932, -16); /* pi */
    let maxint = Dec64::from_parts(36028797018963967, 0); /* the largest normal integer */
    let maxint_plus = Dec64::from_parts(3602879701896397, 1); /* the smallest number larger than maxint */
    let maxnum = Dec64::from_parts(36028797018963967, 127); /* the largest possible number */
    let negative_minnum = Dec64::from_parts(-1, -127); /* the smallest possible negative number */
    let negative_one = Dec64::from_parts(-1, 0); /* -1 */
    let negative_nine = Dec64::from_parts(-9, 0); /* -9 */
    let negative_pi = Dec64::from_parts(-31415926535897932, -16); /* -pi */
    let negative_maxint = Dec64::from_parts(-36028797018963968, 0); /* the largest negative normal integer */
    let negative_maxnum = Dec64::from_parts(-36028797018963968, 127); /* the largest possible negative number */
    let almost_negative_one = Dec64::from_parts(-9999999999999999, -16); /* -0.9999999999999999 */

    assert_eq!(nan.to_string(), "nan");
    assert_eq!(nannan.to_string(), "nan");
    assert_eq!(zero.to_string(), "0");
    assert_eq!(zip.to_string(), "0");
    assert_eq!(one.to_string(), "1");
    assert_eq!(two.to_string(), "2");
    assert_eq!(three.to_string(), "3");
    assert_eq!(four.to_string(), "4");
    assert_eq!(five.to_string(), "5");
    assert_eq!(six.to_string(), "6");
    assert_eq!(seven.to_string(), "7");
    assert_eq!(eight.to_string(), "8");
    assert_eq!(nine.to_string(), "9");
    assert_eq!(ten.to_string(), "10");
    assert_eq!(maxint.to_string(), "36028797018963967");
    assert_eq!(maxint_plus.to_string(), "36028797018963970");
    assert_eq!(maxnum.to_string(), "3.6028797018963967e143");
    assert_eq!(minnum.to_string(), "1e-127");
    assert_eq!(epsilon.to_string(), "0.0000000000000001");
    assert_eq!(almost_one.to_string(), "0.9999999999999999");
    assert_eq!(almost_negative_one.to_string(), "-0.9999999999999999");
    assert_eq!(pi.to_string(), "3.1415926535897932");
    assert_eq!(half.to_string(), "0.5");
    assert_eq!(cent.to_string(), "0.01");
    assert_eq!(negative_one.to_string(), "-1");
    assert_eq!(negative_nine.to_string(), "-9");
    assert_eq!(negative_minnum.to_string(), "-1e-127");
    assert_eq!(negative_maxint.to_string(), "-36028797018963968");
    assert_eq!(negative_maxnum.to_string(), "-3.6028797018963968e143");
    assert_eq!(negative_pi.to_string(), "-3.1415926535897932");
}
