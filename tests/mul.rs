#![allow(clippy::wildcard_imports)]

use dec64::Dec64;
use dec64::consts::*;

macro_rules! assert_eq_mult {
    ($lhs:expr, $rhs:expr, $result:expr, $msg:expr) => {
        assert_eq!($lhs * $rhs, $result, $msg)
    };
}

#[test]
fn all_c() {
    assert_eq_mult!(
        Dec64::new(4195835, 0),
        Dec64::new(3145727, 0),
        Dec64::new(13198951447045, 0),
        "4195835 * 3145727"
    );
    assert_eq_mult!(
        Dec64::new(4294967296, 0),
        Dec64::new(2147483648, 0),
        Dec64::new(922337203685477581, 1),
        "4294967296 * 2147483648"
    );
    assert_eq_mult!(
        Dec64::new(4294967296, 0),
        Dec64::new(4294967296, 0),
        Dec64::new(1844674407370955162, 1),
        "4294967296 * 4294967296"
    );
    assert_eq_mult!(NAN, NAN, NAN, "NAN * NAN");
    // Discrepancy from C implementation: Any operation with NAN must produce NAN, according to https://www.crockford.com/dec64.html
    assert_eq_mult!(NAN, ZERO, NAN, "NAN * ZERO");
    assert_eq_mult!(NAN_NAN, NAN_NAN, NAN, "NAN_NAN * NAN_NAN");
    assert_eq_mult!(NAN_NAN, ONE, NAN, "NAN_NAN * 1");
    // Discrepancy from C implementation: Any operation with NAN must produce NAN
    assert_eq_mult!(ZERO, NAN, NAN, "0 * NAN");
    // Discrepancy from C implementation: Any operation with NAN must produce NAN
    assert_eq_mult!(ZERO, NAN_NAN, NAN, "0 * NAN_NAN");
    assert_eq_mult!(ZERO, ZIP, ZERO, "ZERO * ZIP");
    assert_eq_mult!(ZERO, MAX, ZERO, "ZERO * MAX");
    assert_eq_mult!(ZIP, ZERO, ZERO, "ZIP * ZERO");
    assert_eq_mult!(ZIP, ZIP, ZERO, "ZIP * ZIP");
    assert_eq_mult!(
        MIN,
        HALF,
        Dec64::from_parts(-18014398509481984, 127),
        "MIN * HALF"
    );
    // Discrepancy from C implementation: Producing too large values (here: coefficients) results in NAN, not in zero.
    assert_eq_mult!(MIN, MIN, NAN, "MIN * MIN");
    assert_eq_mult!(EPSILON, EPSILON, Dec64::new(1, -32), "EPSILON * EPSILON");
    assert_eq_mult!(ONE, NAN_NAN, NAN, "1 * NAN_NAN");
    assert_eq_mult!(NEG_ONE, ONE, NEG_ONE, "-1 * 1");
    assert_eq_mult!(NEG_ONE, NEG_ONE, ONE, "-1 * -1");
    assert_eq_mult!(TWO, FIVE, TEN, "2 * 5");
    assert_eq_mult!(TWO, MAX, NAN, "2 * MAX");
    assert_eq_mult!(
        TWO,
        Dec64::new(36028797018963967, 126),
        Dec64::new(7205759403792793, 127),
        "2 * a big ONE"
    );
    assert_eq_mult!(THREE, TWO, SIX, "3 * 2");
    assert_eq_mult!(
        TEN,
        Dec64::new(36028797018963967, 126),
        MAX,
        "10 * a big ONE"
    );
    assert_eq_mult!(TEN, Dec64::new(1, 127), Dec64::new(10, 127), "10 * 1e127");
    assert_eq_mult!(
        Dec64::new(1, 2),
        Dec64::new(1, 127),
        Dec64::new(100, 127),
        "1e2 * 1e127"
    );
    assert_eq_mult!(
        Dec64::new(1, 12),
        Dec64::new(1, 127),
        Dec64::new(1000000000000, 127),
        "1e2 * 1e127"
    );
    assert_eq_mult!(
        Dec64::new(1, 12),
        Dec64::new(1, 127),
        Dec64::new(1000000000000, 127),
        "1e12 * 1e127"
    );
    assert_eq_mult!(
        Dec64::new(3, 16),
        Dec64::new(1, 127),
        Dec64::new(30000000000000000, 127),
        "3e16 * 1e127"
    );
    assert_eq_mult!(Dec64::new(3, 17), Dec64::new(1, 127), NAN, "3e16 * 1e127");
    assert_eq_mult!(
        Dec64::new(-3, 16),
        Dec64::new(1, 127),
        Dec64::new(-30000000000000000, 127),
        "3e16 * 1e127"
    );
    assert_eq_mult!(Dec64::new(-3, 17), Dec64::new(1, 127), NAN, "3e16 * 1e127");
    assert_eq_mult!(
        Dec64::new(9999999999999999, 0),
        TEN,
        Dec64::new(9999999999999999, 1),
        "9999999999999999 * 10"
    );
    assert_eq_mult!(MAXINT, ZERO, ZERO, "MAXINT * ZERO");
    assert_eq_mult!(
        MAXINT,
        EPSILON,
        Dec64::new(36028797018963967, -16),
        "MAXINT * EPSILON"
    );
    assert_eq_mult!(
        MAXINT,
        MAXINT,
        Dec64::new(12980742146337068, 17),
        "MAXINT * MAXINT"
    );
    assert_eq_mult!(
        MAXINT,
        Dec64::from_parts(27755575615628914, -33),
        ONE,
        "MAXINT * 1 / MAXINT"
    );
    assert_eq_mult!(MININT, NAN, NAN, "-MAXINT * NAN");
    assert_eq_mult!(
        MININT,
        MAXINT,
        Dec64::new(-12980742146337069, 17),
        "-MAXINT * MAXINT"
    );
    assert_eq_mult!(MAX, MAX, NAN, "MAX * MAX");
    // Discrepancy from C implementation: Producing too large values results in NAN, not in other values.
    assert_eq_mult!(MAX, MIN, NAN, "MAX * MIN");
    assert_eq_mult!(MIN, MIN, NAN, "MIN * MIN");
}
