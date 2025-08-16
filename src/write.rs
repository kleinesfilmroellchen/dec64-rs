use crate::Dec64;
use std::io;

/// look-up-table for every decimal number below 100
const DEC_DIGITS_LUT: [&[u8; 2]; 100] = [
    b"00", b"01", b"02", b"03", b"04", b"05", b"06", b"07", b"08", b"09", b"10", b"11", b"12",
    b"13", b"14", b"15", b"16", b"17", b"18", b"19", b"20", b"21", b"22", b"23", b"24", b"25",
    b"26", b"27", b"28", b"29", b"30", b"31", b"32", b"33", b"34", b"35", b"36", b"37", b"38",
    b"39", b"40", b"41", b"42", b"43", b"44", b"45", b"46", b"47", b"48", b"49", b"50", b"51",
    b"52", b"53", b"54", b"55", b"56", b"57", b"58", b"59", b"60", b"61", b"62", b"63", b"64",
    b"65", b"66", b"67", b"68", b"69", b"70", b"71", b"72", b"73", b"74", b"75", b"76", b"77",
    b"78", b"79", b"80", b"81", b"82", b"83", b"84", b"85", b"86", b"87", b"88", b"89", b"90",
    b"91", b"92", b"93", b"94", b"95", b"96", b"97", b"98", b"99",
];

#[inline(always)]
fn write_num(n: &mut usize, curr: &mut usize, buffer: &mut [u8]) {
    println!("writing {n} at offset {curr} into {buffer:?}");
    // eagerly decode 4 digits at a time
    while *n >= 10000 {
        let rem = *n % 10000;
        *n /= 10000;

        let d1 = rem / 100;
        let d2 = rem % 100;
        *curr -= 4;

        buffer[*curr..*curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
        buffer[*curr + 2..*curr + 4].copy_from_slice(DEC_DIGITS_LUT[d2 as usize]);
    }

    // decode 2 more digits
    if *n >= 100 {
        let d1 = *n % 100;
        *n /= 100;
        *curr -= 2;
        buffer[*curr..*curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
    }

    // decode last 1 or 2 digits
    if *n < 10 {
        *curr -= 1;
        buffer[*curr] = (*n as u8) + b'0';
    } else {
        let d1 = *n;
        *curr -= 2;
        buffer[*curr..*curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
        println!("buf: {buffer:?}");
    }
}

impl Dec64 {
    pub fn write<W: io::Write>(self, wr: &mut W) -> io::Result<()> {
        let mut n = self.coefficient();
        let e = self.exponent() as i16;

        if n == 0 {
            return wr.write_all(b"0");
        } else if e == -128 {
            return wr.write_all(b"nan");
        }

        if n < 0 {
            wr.write_all(b"-")?;
            // convert the negative num to positive by summing 1 to it's 2 complement
            n = -n;
        }
        let mut n = n as usize;

        let mut buf = [0; 24];
        let mut curr = buf.len();

        if e < 0 {
            let mut e = -e as u16;

            // Decimal number with a fraction that's fully printable
            if e < 18 {
                // eagerly decode 4 digits at a time
                for _ in 0..e >> 2 {
                    let rem = n % 10000;
                    n /= 10000;

                    let d1 = rem / 100;
                    let d2 = rem % 100;
                    curr -= 4;
                    buf[curr..curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
                    buf[curr + 2..curr + 4].copy_from_slice(DEC_DIGITS_LUT[d2 as usize]);
                }

                e &= 3;

                // write the remaining 3, 2 or 1 digits
                if e & 2 == 2 {
                    let d1 = n % 100;
                    n /= 100;
                    curr -= 2;
                    buf[curr..curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
                }

                if e & 1 == 1 {
                    curr -= 1;
                    buf[curr] = ((n % 10) as u8) + b'0';
                    n /= 10;
                }

                curr -= 1;
                buf[curr] = b'.';

                write_num(&mut n, &mut curr, &mut buf);

                return wr.write_all(&buf[curr..]);

            // Not easily printable, write down fraction, then full number, then exponent
            } else {
                // Single digit, no fraction
                if n < 10 {
                    curr -= 1;
                    buf[curr] = ((n % 10) as u8) + b'0';
                } else {
                    // eagerly decode 4 digits at a time
                    while n >= 100000 {
                        let rem = n % 10000;
                        n /= 10000;

                        let d1 = rem / 100;
                        let d2 = rem % 100;
                        curr -= 4;
                        buf[curr..curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
                        buf[curr + 2..curr + 4].copy_from_slice(DEC_DIGITS_LUT[d2 as usize]);
                    }

                    // decode 2 more digits
                    if n >= 1000 {
                        let d1 = n % 100;
                        n /= 100;
                        curr -= 2;
                        buf[curr..curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
                    }

                    // decode last 1 or 2 digits
                    if n < 100 {
                        curr -= 1;
                        buf[curr] = ((n % 10) as u8) + b'0';
                        n /= 10;
                    } else {
                        let d1 = n % 100;
                        n /= 100;
                        curr -= 2;
                        buf[curr..curr + 2].copy_from_slice(DEC_DIGITS_LUT[d1 as usize]);
                    }

                    curr -= 1;
                    buf[curr] = b'.';
                }
            }
        }

        write_num(&mut n, &mut curr, &mut buf);

        wr.write_all(&buf[curr..])
    }
}
// }
//
