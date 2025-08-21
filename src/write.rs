//! To-String ([`Display`]) implementation for DEC64.

use crate::Dec64;
use std::fmt::{self, Display, Formatter, Write};
use std::ops::Range;

impl Dec64 {
    const PLACES: i16 = 0;

    pub fn write(self, wr: &mut Formatter) -> fmt::Result {
        let mut coefficient = self.coefficient();
        let exponent = self.exponent();

        if self.is_zero() {
            return wr.write_char('0');
        } else if self.is_nan() {
            return wr.write_str("nan");
        }

        if coefficient < 0 {
            wr.write_char('-')?;
            coefficient = -coefficient;
        }
        let mut coefficient = coefficient as usize;
        let exponent = exponent as i16;

        let mut digit_buffer = [b'0'; 32];
        let mut digit_count = 0;
        let mut zero_count = 0;
        for place_i in 0..=16 {
            let place = 16 - place_i;
            let digit = coefficient / 10usize.pow(place);
            digit_buffer[digit_count] = digit as u8 + b'0';
            if digit == 0 {
                // already have a real digit, so we’re not zero
                if digit_count != 0 {
                    digit_count += 1;
                }
                zero_count += 1;
            } else {
                digit_count += 1;
                zero_count = 0;
            }
            coefficient -= digit * 10usize.pow(place);
        }

        if exponent >= 0 {
            let to = digit_count as i16 + exponent;
            if to + Self::PLACES > 20 {
                Self::write_scientific(exponent, digit_count, zero_count, &digit_buffer, wr)?;
            } else {
                Self::write_digits(&digit_buffer, 0..(to as isize), wr)?;
                if Self::PLACES > 0 {
                    wr.write_char('.')?;
                    Self::write_digits(
                        &digit_buffer,
                        (to as isize)..((Self::PLACES + to) as isize),
                        wr,
                    )?;
                }
            }
        } else {
            let from = digit_count as i16 + exponent;
            let mut to = digit_count - zero_count;
            if from <= 0 {
                let places = to as i16 - from;
                if places > 18 {
                    Self::write_scientific(exponent, digit_count, zero_count, &digit_buffer, wr)?;
                } else {
                    wr.write_str("0.")?;
                    if places < Self::PLACES {
                        to = (Self::PLACES + from) as usize;
                    }
                    Self::write_digits(&digit_buffer, (from as isize)..(to as isize), wr)?;
                }
            } else {
                Self::write_digits(&digit_buffer, 0..(from as isize), wr)?;
                wr.write_char('.')?;
                if to - (from as usize) < Self::PLACES as usize {
                    to = (Self::PLACES + from) as usize;
                }
                Self::write_digits(&digit_buffer, (from as isize)..(to as isize), wr)?;
            }
        }
        Ok(())
    }

    fn write_scientific(
        exponent: i16,
        digit_count: usize,
        zero_count: usize,
        digit_buffer: &[u8],
        wr: &mut Formatter,
    ) -> fmt::Result {
        let adjusted_exponent = exponent + digit_count as i16;
        let digit_count = digit_count - zero_count;

        Self::write_digit_at_idx(digit_buffer, 0, wr)?;
        if 1 < digit_count {
            wr.write_char('.')?;
            Self::write_digits(digit_buffer, 1..(digit_count as isize), wr)?;
        }
        Self::write_exponent(adjusted_exponent - 1, wr)
    }

    #[inline]
    fn write_digit_at_idx(digit_buffer: &[u8], index: isize, wr: &mut Formatter) -> fmt::Result {
        wr.write_char(index.try_into().map_or(b'0', |index: usize| {
            *digit_buffer.get(index).unwrap_or(&b'0')
        }) as char)
    }

    #[inline]
    fn write_digits(digit_buffer: &[u8], range: Range<isize>, wr: &mut Formatter) -> fmt::Result {
        for index in range {
            Self::write_digit_at_idx(digit_buffer, index, wr)?;
        }
        Ok(())
    }

    #[inline]
    fn write_exponent(mut exponent: i16, wr: &mut Formatter) -> fmt::Result {
        if exponent == 0 {
            return Ok(());
        }
        wr.write_char('e')?;
        if exponent < 0 {
            exponent = -exponent;
            wr.write_char('-')?;
        }
        if exponent >= 100 {
            wr.write_char('1')?;
            exponent -= 100;
        }
        if exponent >= 10 || exponent >= 100 {
            wr.write_char((b'0' + (exponent / 10) as u8) as char)?;
        }
        wr.write_char((b'0' + (exponent % 10) as u8) as char)
    }
}

impl Display for Dec64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f)
    }
}
