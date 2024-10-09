#![no_std]
#![doc = include_str!("../README.md")]

/// An error type used to differentiate between overflow and niche errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CheckedError {
    Overflow,
    Niche,
}

macro_rules! nonany {
    ($name:ident, $nonzero:ident, $int:ty, $signed:ident) => {
        /// An integer that is known not to equal `NICHE`.
        /// 
        /// This enables some memory layout optimization.
        #[doc = concat!("For example, `Option<", stringify!($name), "<0>>` is the same size as `", stringify!($int), "`:")]
        ///
        /// ```rust
        #[doc = concat!("assert_eq!(core::mem::size_of::<Option<nonany::", stringify!($name), "<0>>>(), core::mem::size_of::<", stringify!($int), ">());")]
        /// ```
        #[derive(Clone, Copy, Eq, Hash, PartialEq)]
        pub struct $name<const NICHE: $int>(core::num::$nonzero);

        impl<const NICHE: $int> $name<NICHE> {
            /// The niche value of this integer type.
            pub const NICHE: $int = NICHE;

            /// The size of this integer type in bits.
            pub const BITS: u32 = <$int>::BITS;

            /// Creates a non-any if the given value is not `NICHE`.
            pub const fn new(value: $int) -> Option<Self> {
                match core::num::$nonzero::new(value ^ NICHE) {
                    Some(value) => Some(Self(value)),
                    None => None
                }
            }

            /// Creates a non-any without checking whether the value is `NICHE`.
            /// This results in undefined behaviour if the value is `NICHE`.
            ///
            /// # Safety
            ///
            /// The value must not be `NICHE`.
            pub const unsafe fn new_unchecked(value: $int) -> Self {
                Self(core::num::$nonzero::new_unchecked(value ^ NICHE))
            }
    
            /// Returns the contained value as a primitive type.
            pub const fn get(self) -> $int {
                self.0.get() ^ NICHE
            }

            nonany!(@$signed, $name, $nonzero, $int);
        }

        impl<const NICHE: $int> core::cmp::PartialOrd for $name<NICHE> {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl<const NICHE: $int> core::cmp::Ord for $name<NICHE> {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.get().cmp(&other.get())
            }
        }

        impl<const NICHE: $int> core::fmt::Debug for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::Display for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::UpperHex for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::LowerHex for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::Octal for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Octal::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::Binary for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::LowerExp for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::LowerExp::fmt(&self.get(), f)
            }
        }

        impl<const NICHE: $int> core::fmt::UpperExp for $name<NICHE> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperExp::fmt(&self.get(), f)
            }
        }
    };

    (@signed, $name:ident, $nonzero:ident, $int:ty) => {
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
    };
    

    (@unsigned, $name:ident, $nonzero:ident, $int:ty) => {
        
    };
}

nonany!(NonAnyI8, NonZeroI8, i8, signed);
nonany!(NonAnyI16, NonZeroI16, i16, signed);
nonany!(NonAnyI32, NonZeroI32, i32, signed);
nonany!(NonAnyI64, NonZeroI64, i64, signed);
nonany!(NonAnyI128, NonZeroI128, i128, signed);
nonany!(NonAnyIsize, NonZeroIsize, isize, signed);

nonany!(NonAnyU8, NonZeroU8, u8, unsigned);
nonany!(NonAnyU16, NonZeroU16, u16, unsigned);
nonany!(NonAnyU32, NonZeroU32, u32, unsigned);
nonany!(NonAnyU64, NonZeroU64, u64, unsigned);
nonany!(NonAnyU128, NonZeroU128, u128, unsigned);
nonany!(NonAnyUsize, NonZeroUsize, usize, unsigned);

macro_rules! nonany_tryfrom_int {
    ($nonany:ident, $to_int:ty) => {
        nonany_tryfrom_int!($nonany, $to_int, i8);
        nonany_tryfrom_int!($nonany, $to_int, i16);
        nonany_tryfrom_int!($nonany, $to_int, i32);
        nonany_tryfrom_int!($nonany, $to_int, i64);
        nonany_tryfrom_int!($nonany, $to_int, i128);
        nonany_tryfrom_int!($nonany, $to_int, isize);
        nonany_tryfrom_int!($nonany, $to_int, u8);
        nonany_tryfrom_int!($nonany, $to_int, u16);
        nonany_tryfrom_int!($nonany, $to_int, u32);
        nonany_tryfrom_int!($nonany, $to_int, u64);
        nonany_tryfrom_int!($nonany, $to_int, u128);
        nonany_tryfrom_int!($nonany, $to_int, usize);
    };

    ($nonany:ident, $to_int:ty, $from_int:ty) => {
        impl<const NICHE: $to_int> core::convert::TryFrom<$from_int> for $crate::$nonany<NICHE> {
            type Error = $crate::CheckedError;    
            fn try_from(value: $from_int) -> Result<Self, Self::Error> {
                match core::convert::TryInto::<$to_int>::try_into(value) {
                    Ok(value) => match Self::new(value) {
                        Some(value) => Ok(value),
                        None => Err(CheckedError::Niche)
                    },
                    Err(_) => Err(CheckedError::Overflow)
                }
            }
        }
    };
}

nonany_tryfrom_int!(NonAnyI8, i8);
nonany_tryfrom_int!(NonAnyI16, i16);
nonany_tryfrom_int!(NonAnyI32, i32);
nonany_tryfrom_int!(NonAnyI64, i64);
nonany_tryfrom_int!(NonAnyI128, i128);
nonany_tryfrom_int!(NonAnyIsize, isize);

nonany_tryfrom_int!(NonAnyU8, u8);
nonany_tryfrom_int!(NonAnyU16, u16);
nonany_tryfrom_int!(NonAnyU32, u32);
nonany_tryfrom_int!(NonAnyU64, u64);
nonany_tryfrom_int!(NonAnyU128, u128);
nonany_tryfrom_int!(NonAnyUsize, usize);

macro_rules! nonany_tryfrom_nonany {
    ($to_nonany:tt, $to_int:tt) => {
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyI8, i8);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyI16, i16);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyI32, i32);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyI64, i64);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyI128, i128);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyIsize, isize);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyU8, u8);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyU16, u16);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyU32, u32);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyU64, u64);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyU128, u128);
        nonany_tryfrom_nonany!($to_nonany, $to_int, NonAnyUsize, usize);
    };

    (NonAnyI8, i8, NonAnyI8, i8) => { };
    (NonAnyI16, i16, NonAnyI16, i16) => { };
    (NonAnyI32, i32, NonAnyI32, i32) => { };
    (NonAnyI64, i64, NonAnyI64, i64) => { };
    (NonAnyI128, i128, NonAnyI128, i128) => { };
    (NonAnyIsize, isize, NonAnyIsize, isize) => { };
    (NonAnyU8, u8, NonAnyU8, u8) => { };
    (NonAnyU16, u16, NonAnyU16, u16) => { };
    (NonAnyU32, u32, NonAnyU32, u32) => { };
    (NonAnyU64, u64, NonAnyU64, u64) => { };
    (NonAnyU128, u128, NonAnyU128, u128) => { };
    (NonAnyUsize, usize, NonAnyUsize, usize) => { };

    ($to_nonany:ident, $to_int:ty, $from_nonany:ident, $from_int:ty) => {
        impl<const TO_NICHE: $to_int, const FROM_NICHE: $from_int> core::convert::TryFrom<$crate::$from_nonany<FROM_NICHE>> for $crate::$to_nonany<TO_NICHE> {
            type Error = $crate::CheckedError;    
            fn try_from(value: $crate::$from_nonany<FROM_NICHE>) -> Result<Self, Self::Error> {
                match core::convert::TryInto::<$to_int>::try_into(value.get()) {
                    Ok(value) => match Self::new(value) {
                        Some(value) => Ok(value),
                        None => Err(CheckedError::Niche)
                    },
                    Err(_) => Err(CheckedError::Overflow)
                }
            }
        }
    };
}

nonany_tryfrom_nonany!(NonAnyI8, i8);
nonany_tryfrom_nonany!(NonAnyI16, i16);
nonany_tryfrom_nonany!(NonAnyI32, i32);
nonany_tryfrom_nonany!(NonAnyI64, i64);
nonany_tryfrom_nonany!(NonAnyI128, i128);
nonany_tryfrom_nonany!(NonAnyIsize, isize);

nonany_tryfrom_nonany!(NonAnyU8, u8);
nonany_tryfrom_nonany!(NonAnyU16, u16);
nonany_tryfrom_nonany!(NonAnyU32, u32);
nonany_tryfrom_nonany!(NonAnyU64, u64);
nonany_tryfrom_nonany!(NonAnyU128, u128);
nonany_tryfrom_nonany!(NonAnyUsize, usize);

macro_rules! nonany_tryfrom_nonzero {
    ($to_nonany:tt, $to_int:tt) => {
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroI8, i8);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroI16, i16);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroI32, i32);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroI64, i64);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroI128, i128);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroIsize, isize);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroU8, u8);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroU16, u16);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroU32, u32);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroU64, u64);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroU128, u128);
        nonany_tryfrom_nonzero!($to_nonany, $to_int, NonZeroUsize, usize);
    };

    ($to_nonany:ident, $to_int:ty, $from_nonzero:ident, $from_int:ty) => {
        impl<const NICHE: $to_int> core::convert::TryFrom<core::num::$from_nonzero> for $crate::$to_nonany<NICHE> {
            type Error = $crate::CheckedError;    
            fn try_from(value: core::num::$from_nonzero) -> Result<Self, Self::Error> {
                match core::convert::TryInto::<$to_int>::try_into(value.get()) {
                    Ok(value) => match Self::new(value) {
                        Some(value) => Ok(value),
                        None => Err(CheckedError::Niche)
                    },
                    Err(_) => Err(CheckedError::Overflow)
                }
            }
        }
    };
}

nonany_tryfrom_nonzero!(NonAnyI8, i8);
nonany_tryfrom_nonzero!(NonAnyI16, i16);
nonany_tryfrom_nonzero!(NonAnyI32, i32);
nonany_tryfrom_nonzero!(NonAnyI64, i64);
nonany_tryfrom_nonzero!(NonAnyI128, i128);
nonany_tryfrom_nonzero!(NonAnyIsize, isize);

nonany_tryfrom_nonzero!(NonAnyU8, u8);
nonany_tryfrom_nonzero!(NonAnyU16, u16);
nonany_tryfrom_nonzero!(NonAnyU32, u32);
nonany_tryfrom_nonzero!(NonAnyU64, u64);
nonany_tryfrom_nonzero!(NonAnyU128, u128);
nonany_tryfrom_nonzero!(NonAnyUsize, usize);

macro_rules! nonany_typedef {
    ($nonzero:ident, $nonmin:ident, $nonmax:ident, $nonany:ident, $int:ident) => {
        /// An integer that is known not to equal zero.
        pub type $nonzero = $nonany<0>;

        #[doc = concat!("An integer that is known not to equal `", stringify!($int), "::MIN`.")]
        pub type $nonmin = $nonany<{ $int::MIN }>;

        #[doc = concat!("An integer that is known not to equal `", stringify!($int), "::MAX`.")]
        pub type $nonmax = $nonany<{ $int::MAX }>;
    }
}

nonany_typedef!(NonZeroI8, NonMinI8, NonMaxI8, NonAnyI8, i8);
nonany_typedef!(NonZeroI16, NonMinI16, NonMaxI16, NonAnyI16, i16);
nonany_typedef!(NonZeroI32, NonMinI32, NonMaxI32, NonAnyI32, i32);
nonany_typedef!(NonZeroI64, NonMinI64, NonMaxI64, NonAnyI64, i64);
nonany_typedef!(NonZeroI128, NonMinI128, NonMaxI128, NonAnyI128, i128);
nonany_typedef!(NonZeroIsize, NonMinIsize, NonMaxIsize, NonAnyIsize, isize);

nonany_typedef!(NonZeroU8, NonMinU8, NonMaxU8, NonAnyU8, u8);
nonany_typedef!(NonZeroU16, NonMinU16, NonMaxU16, NonAnyU16, u16);
nonany_typedef!(NonZeroU32, NonMinU32, NonMaxU32, NonAnyU32, u32);
nonany_typedef!(NonZeroU64, NonMinU64, NonMaxU64, NonAnyU64, u64);
nonany_typedef!(NonZeroU128, NonMinU128, NonMaxU128, NonAnyU128, u128);
nonany_typedef!(NonZeroUsize, NonMinUsize, NonMaxUsize, NonAnyUsize, usize);

#[cfg(test)]
mod tests;