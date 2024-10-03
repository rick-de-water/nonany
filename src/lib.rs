#![no_std]

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedError {
    Overflow,
    Niche,
}

macro_rules! nonany {
    ($name:ident, $nonzero:ident, $int:ty) => {
        /// An integer that is known not to equal zero.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        pub struct $name<const NICHE: $int>(core::num::$nonzero);

        impl<const NICHE: $int> $name<NICHE> {
            pub const NICHE: $int = NICHE;
            pub const BITS: u32 = <$int>::BITS;

            pub const fn new(value: $int) -> Option<Self> {
                match core::num::$nonzero::new(value ^ NICHE) {
                    Some(value) => Some(Self(value)),
                    None => None
                }
            }

            pub const unsafe fn new_unchecked(value: $int) -> Self {
                Self(core::num::$nonzero::new_unchecked(value ^ NICHE))
            }
    
            pub const fn get(self) -> $int {
                self.0.get() ^ NICHE
            }
        }
    };
}

macro_rules! nonany_signed {
    ($name:ident, $nonzero:ident, $int:ty) => {
        nonany!($name, $nonzero, $int);

        impl<const NICHE: $int> $name<NICHE> {
            pub const fn abs(self) -> Option<Self> {
                let value = self.get().abs();
                Self::new(value)
            }

            pub const fn checked_abs(self) -> Result<Self, CheckedError> {
                match self.get().checked_abs() {
                    Some(value) => match Self::new(value) {
                        Some(value) => Ok(value),
                        None => Err(CheckedError::Niche)
                    },
                    None => Err(CheckedError::Overflow)
                }
            }

            pub const fn is_positive(self) -> bool {
                self.get().is_positive()
            }

            pub const fn is_negative(self) -> bool {
                self.get().is_negative()
            }
        }
    };
}
nonany!(NonAnyU8, NonZeroU8, u8);
nonany!(NonAnyU16, NonZeroU16, u16);
nonany!(NonAnyU32, NonZeroU32, u32);
nonany!(NonAnyU64, NonZeroU64,u64);
nonany!(NonAnyU128, NonZeroU128,u128);
nonany!(NonAnyUsize, NonZeroUsize,usize);

nonany_signed!(NonAnyI8, NonZeroI8, i8);
nonany_signed!(NonAnyI16, NonZeroI16, i16);
nonany_signed!(NonAnyI32, NonZeroI32, i32);
nonany_signed!(NonAnyI64, NonZeroI64, i64);
nonany_signed!(NonAnyI128, NonZeroI128, i128);
nonany_signed!(NonAnyIsize, NonZeroIsize, isize);

pub type NonZeroI8 = NonAnyI8<0>;
pub type NonZeroI16 = NonAnyI16<0>;
pub type NonZeroI32 = NonAnyI32<0>;
pub type NonZeroI64 = NonAnyI64<0>;
pub type NonZeroI128 = NonAnyI128<0>;
pub type NonZeroIsize = NonAnyIsize<0>;

pub type NonMinI8 = NonAnyI8<{ i8::MIN }>;
pub type NonMinI16 = NonAnyI16<{ i16::MIN }>;
pub type NonMinI32 = NonAnyI32<{ i32::MIN }>;
pub type NonMinI64 = NonAnyI64<{ i64::MIN }>;
pub type NonMinI128 = NonAnyI128<{ i128::MIN }>;
pub type NonMinIsize = NonAnyIsize<{ isize::MIN }>;

pub type NonMaxI8 = NonAnyI8<{ i8::MAX }>;
pub type NonMaxI16 = NonAnyI16<{ i16::MAX }>;
pub type NonMaxI32 = NonAnyI32<{ i32::MAX }>;
pub type NonMaxI64 = NonAnyI64<{ i64::MAX }>;
pub type NonMaxI128 = NonAnyI128<{ i128::MAX }>;
pub type NonMaxIsize = NonAnyIsize<{ isize::MAX }>;

pub type NonZeroU8 = NonAnyI8<0>;
pub type NonZeroU16 = NonAnyI16<0>;
pub type NonZeroU32 = NonAnyI32<0>;
pub type NonZeroU64 = NonAnyI64<0>;
pub type NonZeroU128 = NonAnyI128<0>;
pub type NonZeroUsize = NonAnyIsize<0>;

pub type NonMinU8 = NonAnyU8<{ u8::MIN }>;
pub type NonMinU16 = NonAnyU16<{ u16::MIN }>;
pub type NonMinU32 = NonAnyU32<{ u32::MIN }>;
pub type NonMinU64 = NonAnyU64<{ u64::MIN }>;
pub type NonMinU128 = NonAnyU128<{ u128::MIN }>;
pub type NonMinUsize = NonAnyUsize<{ usize::MIN }>;

pub type NonMaxU8 = NonAnyU8<{ u8::MAX }>;
pub type NonMaxU16 = NonAnyU16<{ u16::MAX }>;
pub type NonMaxU32 = NonAnyU32<{ u32::MAX }>;
pub type NonMaxU64 = NonAnyU64<{ u64::MAX }>;
pub type NonMaxU128 = NonAnyU128<{ u128::MAX }>;
pub type NonMaxUsize = NonAnyUsize<{ usize::MAX }>;

#[cfg(test)]
mod tests;