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
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

            nonany!(@signed, $signed, $name, $nonzero, $int);
        }
    };

    (@signed, true, $name:ident, $nonzero:ident, $int:ty) => {
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
    

    (@signed, false, $name:ident, $nonzero:ident, $int:ty) => {
        
    };
}

nonany!(NonAnyU8, NonZeroU8, u8, false);
nonany!(NonAnyU16, NonZeroU16, u16, false);
nonany!(NonAnyU32, NonZeroU32, u32, false);
nonany!(NonAnyU64, NonZeroU64, u64, false);
nonany!(NonAnyU128, NonZeroU128, u128, false);
nonany!(NonAnyUsize, NonZeroUsize, usize, false);

nonany!(NonAnyI8, NonZeroI8, i8, true);
nonany!(NonAnyI16, NonZeroI16, i16, true);
nonany!(NonAnyI32, NonZeroI32, i32, true);
nonany!(NonAnyI64, NonZeroI64, i64, true);
nonany!(NonAnyI128, NonZeroI128, i128, true);
nonany!(NonAnyIsize, NonZeroIsize, isize, true);

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