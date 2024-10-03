# nonany
nonany provides integer types with customizable niche values in stable rust. The main benefit of integer types with niches is that it enables the compiler to do memory layout optimization, such that an `Option` of the integer is the same size as the integer itself:

```rust
assert_eq!(
    core::mem::size_of::<Option<nonany::NonAnyU32<0xDEADBEEF>>>(),
    core::mem::size_of::<nonany::NonAnyU32<0xDEADBEEF>>());
```

## Example
```rust
use nonany::NonAnyI8;

assert!(NonAnyI8::<20>::new(100).is_some(), "Values that aren't the niche can be stored");
assert!(NonAnyI8::<20>::new(20).is_none(), "The niche itself cannot be stored");

let foo = NonAnyI8::<20>::new(25).unwrap();
assert_eq!(foo.get(), 25, "The value can be loaded");
```

## Provided types
nonmax defines generic types with user defined niches for all integer types, as well as type aliases common use cases:

|   | `NonAny` | `NonMin` | `NonMax` | `NonZero` |
|---|---|---|---|---|
| `i8` | [`NonAnyI8`](https://docs.rs/nonany/latest/nonany/struct.NonAnyI8.html) | [`NonMinI8`](https://docs.rs/nonany/latest/nonany/type.NonMinI8.html) | [`NonMaxI8`](https://docs.rs/nonany/latest/nonany/type.NonMaxI8.html) | [`NonZeroI8`](https://docs.rs/nonany/latest/nonany/type.NonZeroI8.html) |
| `i16` | [`NonAnyI16`](https://docs.rs/nonany/latest/nonany/struct.NonAnyI16.html) | [`NonMinI16`](https://docs.rs/nonany/latest/nonany/type.NonMinI16.html) | [`NonMaxI16`](https://docs.rs/nonany/latest/nonany/type.NonMaxI16.html) | [`NonZeroI16`](https://docs.rs/nonany/latest/nonany/type.NonZeroI16.html) |
| `i32` | [`NonAnyI32`](https://docs.rs/nonany/latest/nonany/struct.NonAnyI32.html) | [`NonMinI32`](https://docs.rs/nonany/latest/nonany/type.NonMinI32.html) | [`NonMaxI32`](https://docs.rs/nonany/latest/nonany/type.NonMaxI32.html) | [`NonZeroI32`](https://docs.rs/nonany/latest/nonany/type.NonZeroI32.html) |
| `i64` | [`NonAnyI64`](https://docs.rs/nonany/latest/nonany/struct.NonAnyI64.html) | [`NonMinI64`](https://docs.rs/nonany/latest/nonany/type.NonMinI64.html) | [`NonMaxI64`](https://docs.rs/nonany/latest/nonany/type.NonMaxI64.html) | [`NonZeroI64`](https://docs.rs/nonany/latest/nonany/type.NonZeroI64.html) |
| `i128` | [`NonAnyI128`](https://docs.rs/nonany/latest/nonany/struct.NonAnyI128.html) | [`NonMinI128`](https://docs.rs/nonany/latest/nonany/type.NonMinI128.html) | [`NonMaxI128`](https://docs.rs/nonany/latest/nonany/type.NonMaxI128.html) | [`NonZeroI128`](https://docs.rs/nonany/latest/nonany/type.NonZeroI128.html) |
| `isize` | [`NonAnyIsize`](https://docs.rs/nonany/latest/nonany/struct.NonAnyIsize.html) | [`NonMinIsize`](https://docs.rs/nonany/latest/nonany/type.NonMinIsize.html) | [`NonMaxIsize`](https://docs.rs/nonany/latest/nonany/type.NonMaxIsize.html) | [`NonZeroIsize`](https://docs.rs/nonany/latest/nonany/type.NonZeroIsize.html) |
| `u8` | [`NonAnyU8`](https://docs.rs/nonany/latest/nonany/struct.NonAnyU8.html) | [`NonMinU8`](https://docs.rs/nonany/latest/nonany/type.NonMinU8.html) | [`NonMaxU8`](https://docs.rs/nonany/latest/nonany/type.NonMaxU8.html) | [`NonZeroU8`](https://docs.rs/nonany/latest/nonany/type.NonZeroU8.html) |
| `u16` | [`NonAnyU16`](https://docs.rs/nonany/latest/nonany/struct.NonAnyU16.html) | [`NonMinU16`](https://docs.rs/nonany/latest/nonany/type.NonMinU16.html) | [`NonMaxU16`](https://docs.rs/nonany/latest/nonany/type.NonMaxU16.html) | [`NonZeroU16`](https://docs.rs/nonany/latest/nonany/type.NonZeroU16.html) |
| `u32` | [`NonAnyU32`](https://docs.rs/nonany/latest/nonany/struct.NonAnyU32.html) | [`NonMinU32`](https://docs.rs/nonany/latest/nonany/type.NonMinU32.html) | [`NonMaxU32`](https://docs.rs/nonany/latest/nonany/type.NonMaxU32.html) | [`NonZeroU32`](https://docs.rs/nonany/latest/nonany/type.NonZeroU32.html) |
| `u64` | [`NonAnyU64`](https://docs.rs/nonany/latest/nonany/struct.NonAnyU64.html) | [`NonMinU64`](https://docs.rs/nonany/latest/nonany/type.NonMinU64.html) | [`NonMaxU64`](https://docs.rs/nonany/latest/nonany/type.NonMaxU64.html) | [`NonZeroU64`](https://docs.rs/nonany/latest/nonany/type.NonZeroU64.html) |
| `u128` | [`NonAnyU128`](https://docs.rs/nonany/latest/nonany/struct.NonAnyU128.html) | [`NonMinU128`](https://docs.rs/nonany/latest/nonany/type.NonMinU128.html) | [`NonMaxU128`](https://docs.rs/nonany/latest/nonany/type.NonMaxU128.html) | [`NonZeroU128`](https://docs.rs/nonany/latest/nonany/type.NonZeroU128.html) |
| `usize` | [`NonAnyUsize`](https://docs.rs/nonany/latest/nonany/struct.NonAnyUsize.html) | [`NonMinUsize`](https://docs.rs/nonany/latest/nonany/type.NonMinUsize.html) | [`NonMaxUsize`](https://docs.rs/nonany/latest/nonany/type.NonMaxUsize.html) | [`NonZeroUsize`](https://docs.rs/nonany/latest/nonany/type.NonZeroUsize.html) |


## How does it work?
Internally all `NonAny` types use the `NonZero` types from the standard library. When a value is stored in `NonAny`, the value is stored in the internal `NonZero` as an XOR of the value and the niche. Any value XORed with the niche that isn't the niche itself can never be zero, so this works out perfectly.

The upside of this technique is that it works on stable rust. The downside is that it requires an, albeit cheap, XOR operation to load and store the value. Additionally, unlike the `NonZero` types, transmuting `NonAny` types to their underlying integer types results in a value that was XORed with the niche, instead of the value itself.

## MSRV
The MSRV is fixed at currently 1.56.0, and the intention is to keep it there at least until version 1.0 is released. A bump in the MSRV

## Similar libraries
 - [nonmax](https://github.com/LPGhatguy/nonmax) - Uses the same XOR technique to create types with an `<int>::MAX` niche. The equivalent in nonany would be to either use a niche of `<int>::MAX`, or the `NonMax*` type aliases.
 - [nook](https://github.com/tialaramex/nook/) - Uses unstable `rustc_` attributes to define balanced integers. The equivalent in nonany would be to either use a niche of `<int>::MIN`, or the `NonMin*` type aliases.
## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.