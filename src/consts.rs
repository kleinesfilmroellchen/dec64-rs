//! DEC64 constants.

use super::*;

/// Difference between `1.0` and the next largest representable number = `1.0_e-16`.
pub const EPSILON: Dec64 = Dec64::from_parts(1, -16_i8);
/// Standard Not a Number (NaN) value.
pub const NAN: Dec64 = Dec64::from_raw(0x80);
/// Standard ZERO value.
pub const ZERO: Dec64 = Dec64::from_raw(0);
/// Largest DEC64 value = `36_028_797_018_963_967_e127`.
pub const MAX: Dec64 = Dec64::from_parts(MAX_COEFFICIENT, MAX_EXP);
/// Smallest DEC64 value = `-36_028_797_018_963_968_e127`.
pub const MIN: Dec64 = Dec64::from_parts(MIN_COEFFICIENT, MAX_EXP);
/// Smallest positive DEC64 = `1.0_e-127`.
pub const TINIEST: Dec64 = Dec64::from_parts(1, MIN_EXP);
/// Largest negative DEC64 = `-1.0_e-127`
pub const NEGATIVE_TINIEST: Dec64 = Dec64::from_parts(-1, MIN_EXP);

/// Archimedes constant: π = `3.1415926535897932`.
pub const PI: Dec64 = Dec64::from_parts(31_415_926_535_897_932, -16_i8);

/// π/2.0 = `1.5707963267948966`.
pub const FRAC_PI_2: Dec64 = Dec64::from_parts(15_707_963_267_948_966, -16_i8);

/// π/3.0 = `1.0471975511965977`.
pub const FRAC_PI_3: Dec64 = Dec64::from_parts(10_471_975_511_965_977, -16_i8);

/// π/4.0 = `0.7853981633974483`.
pub const FRAC_PI_4: Dec64 = Dec64::from_parts(7_853_981_633_974_483, -16_i8);

/// π/6.0 = `0.5235987755982989`.
pub const FRAC_PI_6: Dec64 = Dec64::from_parts(5_235_987_755_982_989, -16_i8);

/// π/8.0 = `0.3926990816987242`.
pub const FRAC_PI_8: Dec64 = Dec64::from_parts(3_926_990_816_987_242, -16_i8);

/// 1.0/π = `0.31830988618379067`.
pub const FRAC_1_PI: Dec64 = Dec64::from_parts(31_830_988_618_379_067, -17_i8);

/// 2.0/π = `0.6366197723675813`.
pub const FRAC_2_PI: Dec64 = Dec64::from_parts(6_366_197_723_675_813, -16_i8);

/// 2.0/sqrt(π) = `1.1283791670955125`.
pub const FRAC_2_SQRT_PI: Dec64 = Dec64::from_parts(11_283_791_670_955_125, -16_i8);

/// sqrt(2.0) = `1.4142135623730950`.
pub const SQRT_2: Dec64 = Dec64::from_parts(14_142_135_623_730_950, -16_i8);

/// 1.0/sqrt(2.0) = `0.7071067811865475`.
pub const FRAC_1_SQRT_2: Dec64 = Dec64::from_parts(7_071_067_811_865_475, -16_i8);

/// sqrt(3.0) = `1.7320508075688773`.
pub const SQRT_3: Dec64 = Dec64::from_parts(17_320_508_075_688_773, -16_i8);

/// 1.0/sqrt(3.0) = `0.5773502691896258`.
pub const FRAC_1_SQRT_3: Dec64 = Dec64::from_parts(5_773_502_691_896_258, -16_i8);

/// Euler's number: e = `2.7182818284590452`.
pub const E: Dec64 = Dec64::from_parts(27_182_818_284_590_452, -16_i8);

/// log2(e) = `1.4426950408889634`.
pub const LOG2_E: Dec64 = Dec64::from_parts(14_426_950_408_889_634, -16_i8);

/// log10(e) = `0.4342944819032518`.
pub const LOG10_E: Dec64 = Dec64::from_parts(4_342_944_819_032_518, -16_i8);

/// ln(2.0) = `0.6931471805599453`.
pub const LN_2: Dec64 = Dec64::from_parts(6_931_471_805_599_453, -16_i8);

/// ln(10.0) = `2.3025850929940457`.
pub const LN_10: Dec64 = Dec64::from_parts(23_025_850_929_940_457, -16_i8);

/// A non-normal `NaN`.
pub const NONNORMAL_NAN: Dec64 = Dec64::from_raw(0x8080);
/// Difference between `1.0` and the previous largest representable number = `1.0_e-16`.
pub const NEGATIVE_EPSILON: Dec64 = Dec64::from_parts(-1, -16_i8);
/// A non-normal `0`.
pub const ZIP: Dec64 = Dec64::from_raw(90);
/// `0.01` (one hundredth)
pub const CENT: Dec64 = Dec64::from_parts(1, -2_i8);
/// `0.1`
pub const TENTH: Dec64 = Dec64::from_parts(1, -1_i8);
/// `0.5`
pub const HALF: Dec64 = Dec64::from_parts(5, -1_i8);
/// `0.9999999999999999`
pub const ALMOST_ONE: Dec64 = Dec64::from_parts(9999999999999999, -16_i8);
/// `-0.9999999999999999`
pub const ALMOST_NEGATIVE_ONE: Dec64 = Dec64::from_parts(-9999999999999999, -16_i8);
/// `1.0 / normal::MAXINT`
pub const FRAC_1_MAXINT: Dec64 = Dec64::from_parts(27755575615628914, -33_i8);
/// Googol (`10^100`)
pub const GOOGOL: Dec64 = Dec64::from_parts(1, 100);
/// -PI
pub const NEGATIVE_PI: Dec64 = Dec64::from_parts(-31415926535897932, -16_i8);
/// `-0.1`
pub const NEGATIVE_TENTH: Dec64 = Dec64::from_parts(-1, -1_i8);
/// `-0.2`
pub const NEGATIVE_FIFTH: Dec64 = Dec64::from_parts(-2, -1_i8);

/// `1`
pub const ONE: Dec64 = Dec64::from_parts(1, 0);
/// `2`
pub const TWO: Dec64 = Dec64::from_parts(2, 0);
/// `3`
pub const THREE: Dec64 = Dec64::from_parts(3, 0);
/// `4`
pub const FOUR: Dec64 = Dec64::from_parts(4, 0);
/// `5`
pub const FIVE: Dec64 = Dec64::from_parts(5, 0);
/// `6`
pub const SIX: Dec64 = Dec64::from_parts(6, 0);
/// `7`
pub const SEVEN: Dec64 = Dec64::from_parts(7, 0);
/// `8`
pub const EIGHT: Dec64 = Dec64::from_parts(8, 0);
/// `9`
pub const NINE: Dec64 = Dec64::from_parts(9, 0);
/// `10`
pub const TEN: Dec64 = Dec64::from_parts(1, 1);

/// `-1`
pub const NEGATIVE_ONE: Dec64 = Dec64::from_parts(-1, 0);
/// `-2`
pub const NEGATIVE_TWO: Dec64 = Dec64::from_parts(-2, 0);
/// `-3`
pub const NEGATIVE_THREE: Dec64 = Dec64::from_parts(-3, 0);
/// `-4`
pub const NEGATIVE_FOUR: Dec64 = Dec64::from_parts(-4, 0);
/// `-5`
pub const NEGATIVE_FIVE: Dec64 = Dec64::from_parts(-5, 0);
/// `-6`
pub const NEGATIVE_SIX: Dec64 = Dec64::from_parts(-6, 0);
/// `-7`
pub const NEGATIVE_SEVEN: Dec64 = Dec64::from_parts(-7, 0);
/// `-8`
pub const NEGATIVE_EIGHT: Dec64 = Dec64::from_parts(-8, 0);
/// `-9`
pub const NEGATIVE_NINE: Dec64 = Dec64::from_parts(-9, 0);
/// `-10`
pub const NEGATIVE_TEN: Dec64 = Dec64::from_parts(-1, 1);

/// Maximal normal integer: maximum coefficient and exponent 0
pub const MAXINT: Dec64 = Dec64::from_parts(super::MAX_COEFFICIENT, 0);
/// Minimal normal integer: minimum coefficient and exponent 0
pub const MININT: Dec64 = Dec64::from_parts(super::MIN_COEFFICIENT, 0);
