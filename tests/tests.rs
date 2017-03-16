extern crate dec64;

use dec64::Dec64;

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
    let dec = Dec64::from_parts(dec64::MAX_COEFFICIENT, 127);

    assert_eq!(dec, dec64::MAX);
}

#[test]
fn compose_min() {
    let dec = Dec64::from_parts(dec64::MIN_COEFFICIENT, 127);

    assert_eq!(dec, dec64::MIN);
}

#[test]
fn compose_min_positive() {
    let dec = Dec64::from_parts(1, -127);

    assert_eq!(dec, dec64::MIN_POSITIVE);
}

#[test]
fn compose_nan() {
    let nan_normal = Dec64::from_parts(0, -128);
    let nan_subnormal = Dec64::from_parts(42, -128);

    assert!(nan_normal.is_nan());
    assert!(nan_subnormal.is_nan());
    assert_eq!(nan_normal, dec64::NAN);
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
    assert_eq!(zero_normal, dec64::ZERO);
    assert_eq!(zero_high, dec64::ZERO);
    assert_eq!(zero_low, dec64::ZERO);
}


#[test]
fn write_42() {
    let dec = Dec64::from_parts(42, 0);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "42");
}

#[test]
fn write_pi() {
    let dec = Dec64::from_parts(3141592653589793, -15);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "3.141592653589793");
}


#[test]
fn write_pi_from_float() {
    let dec = Dec64::from(3.141592653589793);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "3.141592653589793");
}

#[test]
fn write_midperiod() {
    let dec = Dec64::from_parts(123456, -3);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "123.456");
}

#[test]
fn write_small() {
    let dec = Dec64::from_parts(1, -5);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "0.00001");
}

#[test]
fn write_tiny() {
    let dec = Dec64::from_parts(123456, -35);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "1.23456e-30");
}


#[test]
fn write_huge() {
    let dec = Dec64::from_parts(123456, 30);

    let mut buf: Vec<u8> = Vec::new();

    dec.write(&mut buf).unwrap();

    let string = String::from_utf8(buf).unwrap();

    assert_eq!(string, "123456e30");
}
