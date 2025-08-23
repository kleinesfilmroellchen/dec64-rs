![](dec64.png)

# [DEC64](http://dec64.com/) in Rust

This package provides a DEC64 decimal floating-point type. DEC64 is a simple floating-point type using powers of 10 (as opposed to powers of 2, like common IEEE 754 binary floating point) and a twos-complement based representation. This simplifies integer handling, gets rid of many special-cased numbers in favor of a simple “NaN” for edge cases, and allows monetary calculations without accumulating errors. See <https://dec64.com> for more details on what DEC64 is and why you should use it instead of traditional floating-point.

This Rust implementation of DEC64 provides a fast, natively-represented implementation in the form of the [`Dec64`] type. Notable features:

- Common arithmetic operators and traits implemented (`Eq`, `Ord`), so it behaves like any other numeric type
- Many edge cases handled better than C/Assembly implementations
- Conversion from any integer type (possibly with precision loss for very large magnitudes)
- Conversion from any binary floating-point type (again with possible precision loss), using the [Grisu2](https://www.cs.tufts.edu/~nr/cs257/archive/florian-loitsch/printf.pdf) algorithm to do so very quickly.
- [`Display`] implementation; FromString and other serialization WIP
- No unsafe code
- `no_std` support by disabling the default `std` feature (removes string conversion functionality)
- Lots of tests, mostly ported from the C implementation, ensuring matching behavior in most cases

The future plan (see below) is to implement at least some functionality directly in assembly (usually by porting the public-domain assembly implementations), but for now, the safe Rust code is fast enough for many applications.

### Planned/current feature list

A rough outline where this library is headed.

- [x] no_std support
- [ ] other operations
	- [x] multiply
	- [x] divide
	- [x] all kinds of rounding
	- [ ] sin/cos/tan
	- [ ] sqrt/invsqrt/pow
- [ ] port all C tests exactly
	- [ ] use C tests to check exact correctness of implementation
- [ ] port (and/or implement) assembly implementations and cross-check against Rust-only implementation:
	- [ ] RISC-V RV64IM
	- [ ] Aarch64
	- [ ] x86-64