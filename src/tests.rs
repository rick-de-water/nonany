use core::{fmt::Write, str};
use impls::impls;

struct FormatBuffer {
    buffer: [u8;4096],
    offset: usize
}

impl FormatBuffer {
    pub fn as_str(&self) -> &str {
        str::from_utf8(&self.buffer[0..self.offset]).unwrap()
    }
}

impl Default for FormatBuffer {
    fn default() -> Self {
        Self {
            buffer:[(); 4096].map(|_| 0u8),
            offset: Default::default()
        }
    }
}

impl Write for FormatBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let end = self.offset + bytes.len();
        if end > self.buffer.len() {
            return Err(core::fmt::Error);
        }

        self.buffer[self.offset..end].copy_from_slice(bytes);

        Ok(())
    }
}

macro_rules! test_size {
    (crate::$name:ident, $base:ident) => {
        const _:() = assert!(core::mem::size_of::<Option<crate::$name>>() == core::mem::size_of::<$base>());
    };
    ($name:ident, $base:ident) => {
        const _:() = assert!(core::mem::size_of::<Option<$name>>() == core::mem::size_of::<$base>());
    };
}

macro_rules! test_nonany {
    (i8, $niche:expr, $test:ident) => {
        test_nonany!(NonAnyI8, i8, $niche, $test, true);
    };

    (u8, $niche:expr, $test:ident) => {
        test_nonany!(NonAnyU8, u8, $niche, $test, false);
    };

    ($nonany:ident, $int:ident, $niche:expr, $test:ident, $signed:ident) => {
        #[test]
        fn $test() {
            const NICHE: $int = $niche;
            type NonAny = crate::$nonany::<{ NICHE }>;

            for i in $int::MIN..=$int::MAX {
                if i == NICHE {
                    continue
                }

                let non = NonAny::new(i).unwrap();

                assert!(PartialEq::eq(&non, &non));
                assert!(!PartialEq::ne(&non, &non));

                assert_eq!(PartialOrd::partial_cmp(&non, &non), Some(core::cmp::Ordering::Equal));
                assert!(!PartialOrd::lt(&non, &non));
                assert!(PartialOrd::le(&non, &non));
                assert!(!PartialOrd::gt(&non, &non));
                assert!(PartialOrd::ge(&non, &non));
                
                assert_eq!(Ord::cmp(&non, &non), core::cmp::Ordering::Equal);
                assert_eq!(Ord::min(non, non), non);
                assert_eq!(Ord::max(non, non), non);
                assert_eq!(Ord::clamp(non, non, non), non);

                if i != $int::MAX && i + 1 != NICHE {
                    let non_plus_1 = NonAny::new(i + 1).unwrap();

                    assert!(!PartialEq::eq(&non, &non_plus_1));
                    assert!(!PartialEq::eq(&non_plus_1, &non));
                    assert!(PartialEq::ne(&non, &non_plus_1));
                    assert!(PartialEq::ne(&non_plus_1, &non));

                    assert_eq!(PartialOrd::partial_cmp(&non, &non_plus_1), Some(core::cmp::Ordering::Less));
                    assert_eq!(PartialOrd::partial_cmp(&non_plus_1, &non), Some(core::cmp::Ordering::Greater));

                    assert!(PartialOrd::lt(&non, &non_plus_1));
                    assert!(!PartialOrd::lt(&non_plus_1, &non));
                    assert!(PartialOrd::le(&non, &non_plus_1));
                    assert!(!PartialOrd::le(&non_plus_1, &non));
                    assert!(!PartialOrd::gt(&non, &non_plus_1));
                    assert!(PartialOrd::gt(&non_plus_1, &non));
                    assert!(!PartialOrd::ge(&non, &non_plus_1));
                    assert!(PartialOrd::ge(&non_plus_1, &non));
                
                    assert_eq!(Ord::cmp(&non, &non_plus_1), core::cmp::Ordering::Less);
                    assert_eq!(Ord::cmp(&non_plus_1, &non), core::cmp::Ordering::Greater);
                    assert_eq!(Ord::min(non, non_plus_1), non);
                    assert_eq!(Ord::min(non_plus_1, non), non);
                    assert_eq!(Ord::max(non, non_plus_1), non_plus_1);
                    assert_eq!(Ord::max(non_plus_1, non), non_plus_1);
                    assert_eq!(Ord::clamp(non, non, non_plus_1), non);
                    assert_eq!(Ord::clamp(non_plus_1, non, non_plus_1), non_plus_1);
                    assert_eq!(Ord::clamp(non, non_plus_1, non_plus_1), non_plus_1);
                    assert_eq!(Ord::clamp(non_plus_1, non, non), non);
                }

                let mut non_buffer = FormatBuffer::default();
                let mut i_buffer = FormatBuffer::default();

                macro_rules! format_str { () => { "{0}, {0:?}, {0:x}, {0:X}, {0:#x}, {0:#X}, {0:o}, {0:b}, {0:e}, {0:E}" } }

                core::write!(non_buffer, format_str!(), non).unwrap();
                core::write!(i_buffer, format_str!(), i).unwrap();

                assert_eq!(non_buffer.as_str(), i_buffer.as_str());
            }
        }
        
        const _: () = {
            const NICHE: $int = $niche;
            type NonAny = crate::$nonany::<{ NICHE }>;
            test_size!(NonAny, $int);

            let mut i = $int::MIN;
            let mut first = true;
            loop {
                if first {
                    first = false;
                } else {
                    if i == $int::MAX {
                        break;
                    }
                    i += 1;
                }

                let non = NonAny::new(i);

                let non = match non {
                    Some(non) => non,
                    None => {
                        assert!(i == NICHE);
                        continue;
                    }
                };

                assert!(non.get() == i);
                assert!(non.0.get() == i ^ NICHE);
                
                assert!(unsafe { NonAny::new_unchecked(i).get() } == non.get());

                test_nonany!(@signed, $signed, $int, NICHE, non, i);
            };
        };
    };

    (@signed, true, $int:ident, $niche:ident, $non:ident, $i:ident) => {
        assert!($non.is_positive() == $i.is_positive());
        assert!($non.is_negative() == $i.is_negative());

        if $i == $int::MIN {
             match $non.checked_abs() {
                Ok(_) => panic!(),
                Err(err) => match err {
                    crate::CheckedError::Overflow => (),
                    crate::CheckedError::Niche => panic!()
                }
            };
        } else if $i < 0 && $i.abs() == $niche {
            match $non.checked_abs() {
                Ok(_) => panic!(),
                Err(err) => match err {
                    crate::CheckedError::Overflow => panic!(),
                    crate::CheckedError::Niche => ()
                }
            };
        } else {
            let abs = match $non.checked_abs() {
                Ok(abs) => abs,
                Err(err) => match err {
                    crate::CheckedError::Overflow => panic!(),
                    crate::CheckedError::Niche => panic!()
                }
            };
            assert!(abs.get() == $i.abs())
        };
    };

    (@signed, false, $non:ident, $int:ident, $i:ident, $niche:ident) => {};
}

test_size!(crate::NonZeroI8, i8);
test_size!(crate::NonZeroI16, i16);
test_size!(crate::NonZeroI32, i32);
test_size!(crate::NonZeroI64, i64);
test_size!(crate::NonZeroI128, i128);
test_size!(crate::NonZeroIsize, isize);

test_size!(crate::NonMinI8, i8);
test_size!(crate::NonMinI16, i16);
test_size!(crate::NonMinI32, i32);
test_size!(crate::NonMinI64, i64);
test_size!(crate::NonMinI128, i128);
test_size!(crate::NonMinIsize, isize);

test_size!(crate::NonMaxI8, i8);
test_size!(crate::NonMaxI16, i16);
test_size!(crate::NonMaxI32, i32);
test_size!(crate::NonMaxI64, i64);
test_size!(crate::NonMaxI128, i128);
test_size!(crate::NonMaxIsize, isize);

test_size!(crate::NonZeroU8, u8);
test_size!(crate::NonZeroU16, u16);
test_size!(crate::NonZeroU32, u32);
test_size!(crate::NonZeroU64, u64);
test_size!(crate::NonZeroU128, u128);
test_size!(crate::NonZeroUsize, usize);

test_size!(crate::NonMinU8, u8);
test_size!(crate::NonMinU16, u16);
test_size!(crate::NonMinU32, u32);
test_size!(crate::NonMinU64, u64);
test_size!(crate::NonMinU128, u128);
test_size!(crate::NonMinUsize, usize);

test_size!(crate::NonMaxU8, u8);
test_size!(crate::NonMaxU16, u16);
test_size!(crate::NonMaxU32, u32);
test_size!(crate::NonMaxU64, u64);
test_size!(crate::NonMaxU128, u128);
test_size!(crate::NonMaxUsize, usize);

macro_rules! test_from_int {
    ($nonany:ident) => {
        const _: () = assert!(impls!($crate::$nonany: !From<i8> & !From<i16> & !From<i32> & !From<i64> & !From<i128> & !From<isize>));
        const _: () = assert!(impls!($crate::$nonany: !From<u8> & !From<u16> & !From<u32> & !From<u64> & !From<u128> & !From<usize>));
        const _: () = assert!(impls!($crate::$nonany: TryFrom<i8> & TryFrom<i16> & TryFrom<i32> & TryFrom<i64> & TryFrom<i128> & TryFrom<isize>));
        const _: () = assert!(impls!($crate::$nonany: TryFrom<u8> & TryFrom<u16> & TryFrom<u32> & TryFrom<u64> & TryFrom<u128> & TryFrom<usize>));
        const _: () = assert!(impls!($crate::$nonany: TryFrom<$crate::NonZeroI8> & TryFrom<$crate::NonZeroI16> & TryFrom<$crate::NonZeroI32> & TryFrom<$crate::NonZeroI64> & TryFrom<$crate::NonZeroI128> & TryFrom<$crate::NonZeroIsize>));
        const _: () = assert!(impls!($crate::$nonany: TryFrom<$crate::NonZeroU8> & TryFrom<$crate::NonZeroU16> & TryFrom<$crate::NonZeroU32> & TryFrom<$crate::NonZeroU64> & TryFrom<$crate::NonZeroU128> & TryFrom<$crate::NonZeroUsize>));
    };
}

test_from_int!(NonZeroI8);
test_from_int!(NonZeroI16);
test_from_int!(NonZeroI32);
test_from_int!(NonZeroI64);
test_from_int!(NonZeroI128);
test_from_int!(NonZeroIsize);

test_from_int!(NonZeroU8);
test_from_int!(NonZeroU16);
test_from_int!(NonZeroU32);
test_from_int!(NonZeroU64);
test_from_int!(NonZeroU128);
test_from_int!(NonZeroUsize);

const _: () = assert!(impls!(crate::NonZeroI8: From<crate::NonZeroI8> & !From<crate::NonMinI8> & !From<crate::NonZeroI16> & !From<crate::NonZeroI32> & !From<crate::NonZeroI64> & !From<crate::NonZeroI128> & !From<crate::NonZeroIsize>));
const _: () = assert!(impls!(crate::NonZeroI16: From<crate::NonZeroI8> & !From<crate::NonMinI8> & From<crate::NonZeroI16> & !From<crate::NonMinI16> & !From<crate::NonZeroI32> & !From<crate::NonZeroI64> & !From<crate::NonZeroI128> & !From<crate::NonZeroIsize>));



//const _: () = assert!(impls!(crate::$nonany: !From<crate::NonZeroU8> & !From<crate::NonZeroU16> & !From<crate::NonZeroU32> & !From<crate::NonZeroU64> & !From<crate::NonZeroU128> & !From<crate::NonZeroUsize>));
//const _: () = assert!(impls!(crate::$nonany: TryFrom<crate::NonZeroI8> & TryFrom<crate::NonZeroI16> & TryFrom<crate::NonZeroI32> & TryFrom<crate::NonZeroI64> & TryFrom<crate::NonZeroI128> & TryFrom<crate::NonZeroIsize>));
//const _: () = assert!(impls!(crate::$nonany: TryFrom<crate::NonZeroU8> & TryFrom<crate::NonZeroU16> & TryFrom<crate::NonZeroU32> & TryFrom<crate::NonZeroU64> & TryFrom<crate::NonZeroU128> & TryFrom<crate::NonZeroUsize>));


test_nonany!(i8, -128i8, test_nonanyi_0);
test_nonany!(i8, -127i8, test_nonanyi_1);
test_nonany!(i8, -126i8, test_nonanyi_2);
test_nonany!(i8, -125i8, test_nonanyi_3);
test_nonany!(i8, -124i8, test_nonanyi_4);
test_nonany!(i8, -123i8, test_nonanyi_5);
test_nonany!(i8, -122i8, test_nonanyi_6);
test_nonany!(i8, -121i8, test_nonanyi_7);
test_nonany!(i8, -120i8, test_nonanyi_8);
test_nonany!(i8, -119i8, test_nonanyi_9);
test_nonany!(i8, -118i8, test_nonanyi_10);
test_nonany!(i8, -117i8, test_nonanyi_11);
test_nonany!(i8, -116i8, test_nonanyi_12);
test_nonany!(i8, -115i8, test_nonanyi_13);
test_nonany!(i8, -114i8, test_nonanyi_14);
test_nonany!(i8, -113i8, test_nonanyi_15);
test_nonany!(i8, -112i8, test_nonanyi_16);
test_nonany!(i8, -111i8, test_nonanyi_17);
test_nonany!(i8, -110i8, test_nonanyi_18);
test_nonany!(i8, -109i8, test_nonanyi_19);
test_nonany!(i8, -108i8, test_nonanyi_20);
test_nonany!(i8, -107i8, test_nonanyi_21);
test_nonany!(i8, -106i8, test_nonanyi_22);
test_nonany!(i8, -105i8, test_nonanyi_23);
test_nonany!(i8, -104i8, test_nonanyi_24);
test_nonany!(i8, -103i8, test_nonanyi_25);
test_nonany!(i8, -102i8, test_nonanyi_26);
test_nonany!(i8, -101i8, test_nonanyi_27);
test_nonany!(i8, -100i8, test_nonanyi_28);
test_nonany!(i8, -99i8, test_nonanyi_29);
test_nonany!(i8, -98i8, test_nonanyi_30);
test_nonany!(i8, -97i8, test_nonanyi_31);
test_nonany!(i8, -96i8, test_nonanyi_32);
test_nonany!(i8, -95i8, test_nonanyi_33);
test_nonany!(i8, -94i8, test_nonanyi_34);
test_nonany!(i8, -93i8, test_nonanyi_35);
test_nonany!(i8, -92i8, test_nonanyi_36);
test_nonany!(i8, -91i8, test_nonanyi_37);
test_nonany!(i8, -90i8, test_nonanyi_38);
test_nonany!(i8, -89i8, test_nonanyi_39);
test_nonany!(i8, -88i8, test_nonanyi_40);
test_nonany!(i8, -87i8, test_nonanyi_41);
test_nonany!(i8, -86i8, test_nonanyi_42);
test_nonany!(i8, -85i8, test_nonanyi_43);
test_nonany!(i8, -84i8, test_nonanyi_44);
test_nonany!(i8, -83i8, test_nonanyi_45);
test_nonany!(i8, -82i8, test_nonanyi_46);
test_nonany!(i8, -81i8, test_nonanyi_47);
test_nonany!(i8, -80i8, test_nonanyi_48);
test_nonany!(i8, -79i8, test_nonanyi_49);
test_nonany!(i8, -78i8, test_nonanyi_50);
test_nonany!(i8, -77i8, test_nonanyi_51);
test_nonany!(i8, -76i8, test_nonanyi_52);
test_nonany!(i8, -75i8, test_nonanyi_53);
test_nonany!(i8, -74i8, test_nonanyi_54);
test_nonany!(i8, -73i8, test_nonanyi_55);
test_nonany!(i8, -72i8, test_nonanyi_56);
test_nonany!(i8, -71i8, test_nonanyi_57);
test_nonany!(i8, -70i8, test_nonanyi_58);
test_nonany!(i8, -69i8, test_nonanyi_59);
test_nonany!(i8, -68i8, test_nonanyi_60);
test_nonany!(i8, -67i8, test_nonanyi_61);
test_nonany!(i8, -66i8, test_nonanyi_62);
test_nonany!(i8, -65i8, test_nonanyi_63);
test_nonany!(i8, -64i8, test_nonanyi_64);
test_nonany!(i8, -63i8, test_nonanyi_65);
test_nonany!(i8, -62i8, test_nonanyi_66);
test_nonany!(i8, -61i8, test_nonanyi_67);
test_nonany!(i8, -60i8, test_nonanyi_68);
test_nonany!(i8, -59i8, test_nonanyi_69);
test_nonany!(i8, -58i8, test_nonanyi_70);
test_nonany!(i8, -57i8, test_nonanyi_71);
test_nonany!(i8, -56i8, test_nonanyi_72);
test_nonany!(i8, -55i8, test_nonanyi_73);
test_nonany!(i8, -54i8, test_nonanyi_74);
test_nonany!(i8, -53i8, test_nonanyi_75);
test_nonany!(i8, -52i8, test_nonanyi_76);
test_nonany!(i8, -51i8, test_nonanyi_77);
test_nonany!(i8, -50i8, test_nonanyi_78);
test_nonany!(i8, -49i8, test_nonanyi_79);
test_nonany!(i8, -48i8, test_nonanyi_80);
test_nonany!(i8, -47i8, test_nonanyi_81);
test_nonany!(i8, -46i8, test_nonanyi_82);
test_nonany!(i8, -45i8, test_nonanyi_83);
test_nonany!(i8, -44i8, test_nonanyi_84);
test_nonany!(i8, -43i8, test_nonanyi_85);
test_nonany!(i8, -42i8, test_nonanyi_86);
test_nonany!(i8, -41i8, test_nonanyi_87);
test_nonany!(i8, -40i8, test_nonanyi_88);
test_nonany!(i8, -39i8, test_nonanyi_89);
test_nonany!(i8, -38i8, test_nonanyi_90);
test_nonany!(i8, -37i8, test_nonanyi_91);
test_nonany!(i8, -36i8, test_nonanyi_92);
test_nonany!(i8, -35i8, test_nonanyi_93);
test_nonany!(i8, -34i8, test_nonanyi_94);
test_nonany!(i8, -33i8, test_nonanyi_95);
test_nonany!(i8, -32i8, test_nonanyi_96);
test_nonany!(i8, -31i8, test_nonanyi_97);
test_nonany!(i8, -30i8, test_nonanyi_98);
test_nonany!(i8, -29i8, test_nonanyi_99);
test_nonany!(i8, -28i8, test_nonanyi_100);
test_nonany!(i8, -27i8, test_nonanyi_101);
test_nonany!(i8, -26i8, test_nonanyi_102);
test_nonany!(i8, -25i8, test_nonanyi_103);
test_nonany!(i8, -24i8, test_nonanyi_104);
test_nonany!(i8, -23i8, test_nonanyi_105);
test_nonany!(i8, -22i8, test_nonanyi_106);
test_nonany!(i8, -21i8, test_nonanyi_107);
test_nonany!(i8, -20i8, test_nonanyi_108);
test_nonany!(i8, -19i8, test_nonanyi_109);
test_nonany!(i8, -18i8, test_nonanyi_110);
test_nonany!(i8, -17i8, test_nonanyi_111);
test_nonany!(i8, -16i8, test_nonanyi_112);
test_nonany!(i8, -15i8, test_nonanyi_113);
test_nonany!(i8, -14i8, test_nonanyi_114);
test_nonany!(i8, -13i8, test_nonanyi_115);
test_nonany!(i8, -12i8, test_nonanyi_116);
test_nonany!(i8, -11i8, test_nonanyi_117);
test_nonany!(i8, -10i8, test_nonanyi_118);
test_nonany!(i8, -9i8, test_nonanyi_119);
test_nonany!(i8, -8i8, test_nonanyi_120);
test_nonany!(i8, -7i8, test_nonanyi_121);
test_nonany!(i8, -6i8, test_nonanyi_122);
test_nonany!(i8, -5i8, test_nonanyi_123);
test_nonany!(i8, -4i8, test_nonanyi_124);
test_nonany!(i8, -3i8, test_nonanyi_125);
test_nonany!(i8, -2i8, test_nonanyi_126);
test_nonany!(i8, -1i8, test_nonanyi_127);
test_nonany!(i8, 0i8, test_nonanyi_128);
test_nonany!(i8, 1i8, test_nonanyi_129);
test_nonany!(i8, 2i8, test_nonanyi_130);
test_nonany!(i8, 3i8, test_nonanyi_131);
test_nonany!(i8, 4i8, test_nonanyi_132);
test_nonany!(i8, 5i8, test_nonanyi_133);
test_nonany!(i8, 6i8, test_nonanyi_134);
test_nonany!(i8, 7i8, test_nonanyi_135);
test_nonany!(i8, 8i8, test_nonanyi_136);
test_nonany!(i8, 9i8, test_nonanyi_137);
test_nonany!(i8, 10i8, test_nonanyi_138);
test_nonany!(i8, 11i8, test_nonanyi_139);
test_nonany!(i8, 12i8, test_nonanyi_140);
test_nonany!(i8, 13i8, test_nonanyi_141);
test_nonany!(i8, 14i8, test_nonanyi_142);
test_nonany!(i8, 15i8, test_nonanyi_143);
test_nonany!(i8, 16i8, test_nonanyi_144);
test_nonany!(i8, 17i8, test_nonanyi_145);
test_nonany!(i8, 18i8, test_nonanyi_146);
test_nonany!(i8, 19i8, test_nonanyi_147);
test_nonany!(i8, 20i8, test_nonanyi_148);
test_nonany!(i8, 21i8, test_nonanyi_149);
test_nonany!(i8, 22i8, test_nonanyi_150);
test_nonany!(i8, 23i8, test_nonanyi_151);
test_nonany!(i8, 24i8, test_nonanyi_152);
test_nonany!(i8, 25i8, test_nonanyi_153);
test_nonany!(i8, 26i8, test_nonanyi_154);
test_nonany!(i8, 27i8, test_nonanyi_155);
test_nonany!(i8, 28i8, test_nonanyi_156);
test_nonany!(i8, 29i8, test_nonanyi_157);
test_nonany!(i8, 30i8, test_nonanyi_158);
test_nonany!(i8, 31i8, test_nonanyi_159);
test_nonany!(i8, 32i8, test_nonanyi_160);
test_nonany!(i8, 33i8, test_nonanyi_161);
test_nonany!(i8, 34i8, test_nonanyi_162);
test_nonany!(i8, 35i8, test_nonanyi_163);
test_nonany!(i8, 36i8, test_nonanyi_164);
test_nonany!(i8, 37i8, test_nonanyi_165);
test_nonany!(i8, 38i8, test_nonanyi_166);
test_nonany!(i8, 39i8, test_nonanyi_167);
test_nonany!(i8, 40i8, test_nonanyi_168);
test_nonany!(i8, 41i8, test_nonanyi_169);
test_nonany!(i8, 42i8, test_nonanyi_170);
test_nonany!(i8, 43i8, test_nonanyi_171);
test_nonany!(i8, 44i8, test_nonanyi_172);
test_nonany!(i8, 45i8, test_nonanyi_173);
test_nonany!(i8, 46i8, test_nonanyi_174);
test_nonany!(i8, 47i8, test_nonanyi_175);
test_nonany!(i8, 48i8, test_nonanyi_176);
test_nonany!(i8, 49i8, test_nonanyi_177);
test_nonany!(i8, 50i8, test_nonanyi_178);
test_nonany!(i8, 51i8, test_nonanyi_179);
test_nonany!(i8, 52i8, test_nonanyi_180);
test_nonany!(i8, 53i8, test_nonanyi_181);
test_nonany!(i8, 54i8, test_nonanyi_182);
test_nonany!(i8, 55i8, test_nonanyi_183);
test_nonany!(i8, 56i8, test_nonanyi_184);
test_nonany!(i8, 57i8, test_nonanyi_185);
test_nonany!(i8, 58i8, test_nonanyi_186);
test_nonany!(i8, 59i8, test_nonanyi_187);
test_nonany!(i8, 60i8, test_nonanyi_188);
test_nonany!(i8, 61i8, test_nonanyi_189);
test_nonany!(i8, 62i8, test_nonanyi_190);
test_nonany!(i8, 63i8, test_nonanyi_191);
test_nonany!(i8, 64i8, test_nonanyi_192);
test_nonany!(i8, 65i8, test_nonanyi_193);
test_nonany!(i8, 66i8, test_nonanyi_194);
test_nonany!(i8, 67i8, test_nonanyi_195);
test_nonany!(i8, 68i8, test_nonanyi_196);
test_nonany!(i8, 69i8, test_nonanyi_197);
test_nonany!(i8, 70i8, test_nonanyi_198);
test_nonany!(i8, 71i8, test_nonanyi_199);
test_nonany!(i8, 72i8, test_nonanyi_200);
test_nonany!(i8, 73i8, test_nonanyi_201);
test_nonany!(i8, 74i8, test_nonanyi_202);
test_nonany!(i8, 75i8, test_nonanyi_203);
test_nonany!(i8, 76i8, test_nonanyi_204);
test_nonany!(i8, 77i8, test_nonanyi_205);
test_nonany!(i8, 78i8, test_nonanyi_206);
test_nonany!(i8, 79i8, test_nonanyi_207);
test_nonany!(i8, 80i8, test_nonanyi_208);
test_nonany!(i8, 81i8, test_nonanyi_209);
test_nonany!(i8, 82i8, test_nonanyi_210);
test_nonany!(i8, 83i8, test_nonanyi_211);
test_nonany!(i8, 84i8, test_nonanyi_212);
test_nonany!(i8, 85i8, test_nonanyi_213);
test_nonany!(i8, 86i8, test_nonanyi_214);
test_nonany!(i8, 87i8, test_nonanyi_215);
test_nonany!(i8, 88i8, test_nonanyi_216);
test_nonany!(i8, 89i8, test_nonanyi_217);
test_nonany!(i8, 90i8, test_nonanyi_218);
test_nonany!(i8, 91i8, test_nonanyi_219);
test_nonany!(i8, 92i8, test_nonanyi_220);
test_nonany!(i8, 93i8, test_nonanyi_221);
test_nonany!(i8, 94i8, test_nonanyi_222);
test_nonany!(i8, 95i8, test_nonanyi_223);
test_nonany!(i8, 96i8, test_nonanyi_224);
test_nonany!(i8, 97i8, test_nonanyi_225);
test_nonany!(i8, 98i8, test_nonanyi_226);
test_nonany!(i8, 99i8, test_nonanyi_227);
test_nonany!(i8, 100i8, test_nonanyi_228);
test_nonany!(i8, 101i8, test_nonanyi_229);
test_nonany!(i8, 102i8, test_nonanyi_230);
test_nonany!(i8, 103i8, test_nonanyi_231);
test_nonany!(i8, 104i8, test_nonanyi_232);
test_nonany!(i8, 105i8, test_nonanyi_233);
test_nonany!(i8, 106i8, test_nonanyi_234);
test_nonany!(i8, 107i8, test_nonanyi_235);
test_nonany!(i8, 108i8, test_nonanyi_236);
test_nonany!(i8, 109i8, test_nonanyi_237);
test_nonany!(i8, 110i8, test_nonanyi_238);
test_nonany!(i8, 111i8, test_nonanyi_239);
test_nonany!(i8, 112i8, test_nonanyi_240);
test_nonany!(i8, 113i8, test_nonanyi_241);
test_nonany!(i8, 114i8, test_nonanyi_242);
test_nonany!(i8, 115i8, test_nonanyi_243);
test_nonany!(i8, 116i8, test_nonanyi_244);
test_nonany!(i8, 117i8, test_nonanyi_245);
test_nonany!(i8, 118i8, test_nonanyi_246);
test_nonany!(i8, 119i8, test_nonanyi_247);
test_nonany!(i8, 120i8, test_nonanyi_248);
test_nonany!(i8, 121i8, test_nonanyi_249);
test_nonany!(i8, 122i8, test_nonanyi_250);
test_nonany!(i8, 123i8, test_nonanyi_251);
test_nonany!(i8, 124i8, test_nonanyi_252);
test_nonany!(i8, 125i8, test_nonanyi_253);
test_nonany!(i8, 126i8, test_nonanyi_254);
test_nonany!(i8, 127i8, test_nonanyi_255);
test_nonany!(u8, 0u8, test_nonanyu_0);
test_nonany!(u8, 1u8, test_nonanyu_1);
test_nonany!(u8, 2u8, test_nonanyu_2);
test_nonany!(u8, 3u8, test_nonanyu_3);
test_nonany!(u8, 4u8, test_nonanyu_4);
test_nonany!(u8, 5u8, test_nonanyu_5);
test_nonany!(u8, 6u8, test_nonanyu_6);
test_nonany!(u8, 7u8, test_nonanyu_7);
test_nonany!(u8, 8u8, test_nonanyu_8);
test_nonany!(u8, 9u8, test_nonanyu_9);
test_nonany!(u8, 10u8, test_nonanyu_10);
test_nonany!(u8, 11u8, test_nonanyu_11);
test_nonany!(u8, 12u8, test_nonanyu_12);
test_nonany!(u8, 13u8, test_nonanyu_13);
test_nonany!(u8, 14u8, test_nonanyu_14);
test_nonany!(u8, 15u8, test_nonanyu_15);
test_nonany!(u8, 16u8, test_nonanyu_16);
test_nonany!(u8, 17u8, test_nonanyu_17);
test_nonany!(u8, 18u8, test_nonanyu_18);
test_nonany!(u8, 19u8, test_nonanyu_19);
test_nonany!(u8, 20u8, test_nonanyu_20);
test_nonany!(u8, 21u8, test_nonanyu_21);
test_nonany!(u8, 22u8, test_nonanyu_22);
test_nonany!(u8, 23u8, test_nonanyu_23);
test_nonany!(u8, 24u8, test_nonanyu_24);
test_nonany!(u8, 25u8, test_nonanyu_25);
test_nonany!(u8, 26u8, test_nonanyu_26);
test_nonany!(u8, 27u8, test_nonanyu_27);
test_nonany!(u8, 28u8, test_nonanyu_28);
test_nonany!(u8, 29u8, test_nonanyu_29);
test_nonany!(u8, 30u8, test_nonanyu_30);
test_nonany!(u8, 31u8, test_nonanyu_31);
test_nonany!(u8, 32u8, test_nonanyu_32);
test_nonany!(u8, 33u8, test_nonanyu_33);
test_nonany!(u8, 34u8, test_nonanyu_34);
test_nonany!(u8, 35u8, test_nonanyu_35);
test_nonany!(u8, 36u8, test_nonanyu_36);
test_nonany!(u8, 37u8, test_nonanyu_37);
test_nonany!(u8, 38u8, test_nonanyu_38);
test_nonany!(u8, 39u8, test_nonanyu_39);
test_nonany!(u8, 40u8, test_nonanyu_40);
test_nonany!(u8, 41u8, test_nonanyu_41);
test_nonany!(u8, 42u8, test_nonanyu_42);
test_nonany!(u8, 43u8, test_nonanyu_43);
test_nonany!(u8, 44u8, test_nonanyu_44);
test_nonany!(u8, 45u8, test_nonanyu_45);
test_nonany!(u8, 46u8, test_nonanyu_46);
test_nonany!(u8, 47u8, test_nonanyu_47);
test_nonany!(u8, 48u8, test_nonanyu_48);
test_nonany!(u8, 49u8, test_nonanyu_49);
test_nonany!(u8, 50u8, test_nonanyu_50);
test_nonany!(u8, 51u8, test_nonanyu_51);
test_nonany!(u8, 52u8, test_nonanyu_52);
test_nonany!(u8, 53u8, test_nonanyu_53);
test_nonany!(u8, 54u8, test_nonanyu_54);
test_nonany!(u8, 55u8, test_nonanyu_55);
test_nonany!(u8, 56u8, test_nonanyu_56);
test_nonany!(u8, 57u8, test_nonanyu_57);
test_nonany!(u8, 58u8, test_nonanyu_58);
test_nonany!(u8, 59u8, test_nonanyu_59);
test_nonany!(u8, 60u8, test_nonanyu_60);
test_nonany!(u8, 61u8, test_nonanyu_61);
test_nonany!(u8, 62u8, test_nonanyu_62);
test_nonany!(u8, 63u8, test_nonanyu_63);
test_nonany!(u8, 64u8, test_nonanyu_64);
test_nonany!(u8, 65u8, test_nonanyu_65);
test_nonany!(u8, 66u8, test_nonanyu_66);
test_nonany!(u8, 67u8, test_nonanyu_67);
test_nonany!(u8, 68u8, test_nonanyu_68);
test_nonany!(u8, 69u8, test_nonanyu_69);
test_nonany!(u8, 70u8, test_nonanyu_70);
test_nonany!(u8, 71u8, test_nonanyu_71);
test_nonany!(u8, 72u8, test_nonanyu_72);
test_nonany!(u8, 73u8, test_nonanyu_73);
test_nonany!(u8, 74u8, test_nonanyu_74);
test_nonany!(u8, 75u8, test_nonanyu_75);
test_nonany!(u8, 76u8, test_nonanyu_76);
test_nonany!(u8, 77u8, test_nonanyu_77);
test_nonany!(u8, 78u8, test_nonanyu_78);
test_nonany!(u8, 79u8, test_nonanyu_79);
test_nonany!(u8, 80u8, test_nonanyu_80);
test_nonany!(u8, 81u8, test_nonanyu_81);
test_nonany!(u8, 82u8, test_nonanyu_82);
test_nonany!(u8, 83u8, test_nonanyu_83);
test_nonany!(u8, 84u8, test_nonanyu_84);
test_nonany!(u8, 85u8, test_nonanyu_85);
test_nonany!(u8, 86u8, test_nonanyu_86);
test_nonany!(u8, 87u8, test_nonanyu_87);
test_nonany!(u8, 88u8, test_nonanyu_88);
test_nonany!(u8, 89u8, test_nonanyu_89);
test_nonany!(u8, 90u8, test_nonanyu_90);
test_nonany!(u8, 91u8, test_nonanyu_91);
test_nonany!(u8, 92u8, test_nonanyu_92);
test_nonany!(u8, 93u8, test_nonanyu_93);
test_nonany!(u8, 94u8, test_nonanyu_94);
test_nonany!(u8, 95u8, test_nonanyu_95);
test_nonany!(u8, 96u8, test_nonanyu_96);
test_nonany!(u8, 97u8, test_nonanyu_97);
test_nonany!(u8, 98u8, test_nonanyu_98);
test_nonany!(u8, 99u8, test_nonanyu_99);
test_nonany!(u8, 100u8, test_nonanyu_100);
test_nonany!(u8, 101u8, test_nonanyu_101);
test_nonany!(u8, 102u8, test_nonanyu_102);
test_nonany!(u8, 103u8, test_nonanyu_103);
test_nonany!(u8, 104u8, test_nonanyu_104);
test_nonany!(u8, 105u8, test_nonanyu_105);
test_nonany!(u8, 106u8, test_nonanyu_106);
test_nonany!(u8, 107u8, test_nonanyu_107);
test_nonany!(u8, 108u8, test_nonanyu_108);
test_nonany!(u8, 109u8, test_nonanyu_109);
test_nonany!(u8, 110u8, test_nonanyu_110);
test_nonany!(u8, 111u8, test_nonanyu_111);
test_nonany!(u8, 112u8, test_nonanyu_112);
test_nonany!(u8, 113u8, test_nonanyu_113);
test_nonany!(u8, 114u8, test_nonanyu_114);
test_nonany!(u8, 115u8, test_nonanyu_115);
test_nonany!(u8, 116u8, test_nonanyu_116);
test_nonany!(u8, 117u8, test_nonanyu_117);
test_nonany!(u8, 118u8, test_nonanyu_118);
test_nonany!(u8, 119u8, test_nonanyu_119);
test_nonany!(u8, 120u8, test_nonanyu_120);
test_nonany!(u8, 121u8, test_nonanyu_121);
test_nonany!(u8, 122u8, test_nonanyu_122);
test_nonany!(u8, 123u8, test_nonanyu_123);
test_nonany!(u8, 124u8, test_nonanyu_124);
test_nonany!(u8, 125u8, test_nonanyu_125);
test_nonany!(u8, 126u8, test_nonanyu_126);
test_nonany!(u8, 127u8, test_nonanyu_127);
test_nonany!(u8, 128u8, test_nonanyu_128);
test_nonany!(u8, 129u8, test_nonanyu_129);
test_nonany!(u8, 130u8, test_nonanyu_130);
test_nonany!(u8, 131u8, test_nonanyu_131);
test_nonany!(u8, 132u8, test_nonanyu_132);
test_nonany!(u8, 133u8, test_nonanyu_133);
test_nonany!(u8, 134u8, test_nonanyu_134);
test_nonany!(u8, 135u8, test_nonanyu_135);
test_nonany!(u8, 136u8, test_nonanyu_136);
test_nonany!(u8, 137u8, test_nonanyu_137);
test_nonany!(u8, 138u8, test_nonanyu_138);
test_nonany!(u8, 139u8, test_nonanyu_139);
test_nonany!(u8, 140u8, test_nonanyu_140);
test_nonany!(u8, 141u8, test_nonanyu_141);
test_nonany!(u8, 142u8, test_nonanyu_142);
test_nonany!(u8, 143u8, test_nonanyu_143);
test_nonany!(u8, 144u8, test_nonanyu_144);
test_nonany!(u8, 145u8, test_nonanyu_145);
test_nonany!(u8, 146u8, test_nonanyu_146);
test_nonany!(u8, 147u8, test_nonanyu_147);
test_nonany!(u8, 148u8, test_nonanyu_148);
test_nonany!(u8, 149u8, test_nonanyu_149);
test_nonany!(u8, 150u8, test_nonanyu_150);
test_nonany!(u8, 151u8, test_nonanyu_151);
test_nonany!(u8, 152u8, test_nonanyu_152);
test_nonany!(u8, 153u8, test_nonanyu_153);
test_nonany!(u8, 154u8, test_nonanyu_154);
test_nonany!(u8, 155u8, test_nonanyu_155);
test_nonany!(u8, 156u8, test_nonanyu_156);
test_nonany!(u8, 157u8, test_nonanyu_157);
test_nonany!(u8, 158u8, test_nonanyu_158);
test_nonany!(u8, 159u8, test_nonanyu_159);
test_nonany!(u8, 160u8, test_nonanyu_160);
test_nonany!(u8, 161u8, test_nonanyu_161);
test_nonany!(u8, 162u8, test_nonanyu_162);
test_nonany!(u8, 163u8, test_nonanyu_163);
test_nonany!(u8, 164u8, test_nonanyu_164);
test_nonany!(u8, 165u8, test_nonanyu_165);
test_nonany!(u8, 166u8, test_nonanyu_166);
test_nonany!(u8, 167u8, test_nonanyu_167);
test_nonany!(u8, 168u8, test_nonanyu_168);
test_nonany!(u8, 169u8, test_nonanyu_169);
test_nonany!(u8, 170u8, test_nonanyu_170);
test_nonany!(u8, 171u8, test_nonanyu_171);
test_nonany!(u8, 172u8, test_nonanyu_172);
test_nonany!(u8, 173u8, test_nonanyu_173);
test_nonany!(u8, 174u8, test_nonanyu_174);
test_nonany!(u8, 175u8, test_nonanyu_175);
test_nonany!(u8, 176u8, test_nonanyu_176);
test_nonany!(u8, 177u8, test_nonanyu_177);
test_nonany!(u8, 178u8, test_nonanyu_178);
test_nonany!(u8, 179u8, test_nonanyu_179);
test_nonany!(u8, 180u8, test_nonanyu_180);
test_nonany!(u8, 181u8, test_nonanyu_181);
test_nonany!(u8, 182u8, test_nonanyu_182);
test_nonany!(u8, 183u8, test_nonanyu_183);
test_nonany!(u8, 184u8, test_nonanyu_184);
test_nonany!(u8, 185u8, test_nonanyu_185);
test_nonany!(u8, 186u8, test_nonanyu_186);
test_nonany!(u8, 187u8, test_nonanyu_187);
test_nonany!(u8, 188u8, test_nonanyu_188);
test_nonany!(u8, 189u8, test_nonanyu_189);
test_nonany!(u8, 190u8, test_nonanyu_190);
test_nonany!(u8, 191u8, test_nonanyu_191);
test_nonany!(u8, 192u8, test_nonanyu_192);
test_nonany!(u8, 193u8, test_nonanyu_193);
test_nonany!(u8, 194u8, test_nonanyu_194);
test_nonany!(u8, 195u8, test_nonanyu_195);
test_nonany!(u8, 196u8, test_nonanyu_196);
test_nonany!(u8, 197u8, test_nonanyu_197);
test_nonany!(u8, 198u8, test_nonanyu_198);
test_nonany!(u8, 199u8, test_nonanyu_199);
test_nonany!(u8, 200u8, test_nonanyu_200);
test_nonany!(u8, 201u8, test_nonanyu_201);
test_nonany!(u8, 202u8, test_nonanyu_202);
test_nonany!(u8, 203u8, test_nonanyu_203);
test_nonany!(u8, 204u8, test_nonanyu_204);
test_nonany!(u8, 205u8, test_nonanyu_205);
test_nonany!(u8, 206u8, test_nonanyu_206);
test_nonany!(u8, 207u8, test_nonanyu_207);
test_nonany!(u8, 208u8, test_nonanyu_208);
test_nonany!(u8, 209u8, test_nonanyu_209);
test_nonany!(u8, 210u8, test_nonanyu_210);
test_nonany!(u8, 211u8, test_nonanyu_211);
test_nonany!(u8, 212u8, test_nonanyu_212);
test_nonany!(u8, 213u8, test_nonanyu_213);
test_nonany!(u8, 214u8, test_nonanyu_214);
test_nonany!(u8, 215u8, test_nonanyu_215);
test_nonany!(u8, 216u8, test_nonanyu_216);
test_nonany!(u8, 217u8, test_nonanyu_217);
test_nonany!(u8, 218u8, test_nonanyu_218);
test_nonany!(u8, 219u8, test_nonanyu_219);
test_nonany!(u8, 220u8, test_nonanyu_220);
test_nonany!(u8, 221u8, test_nonanyu_221);
test_nonany!(u8, 222u8, test_nonanyu_222);
test_nonany!(u8, 223u8, test_nonanyu_223);
test_nonany!(u8, 224u8, test_nonanyu_224);
test_nonany!(u8, 225u8, test_nonanyu_225);
test_nonany!(u8, 226u8, test_nonanyu_226);
test_nonany!(u8, 227u8, test_nonanyu_227);
test_nonany!(u8, 228u8, test_nonanyu_228);
test_nonany!(u8, 229u8, test_nonanyu_229);
test_nonany!(u8, 230u8, test_nonanyu_230);
test_nonany!(u8, 231u8, test_nonanyu_231);
test_nonany!(u8, 232u8, test_nonanyu_232);
test_nonany!(u8, 233u8, test_nonanyu_233);
test_nonany!(u8, 234u8, test_nonanyu_234);
test_nonany!(u8, 235u8, test_nonanyu_235);
test_nonany!(u8, 236u8, test_nonanyu_236);
test_nonany!(u8, 237u8, test_nonanyu_237);
test_nonany!(u8, 238u8, test_nonanyu_238);
test_nonany!(u8, 239u8, test_nonanyu_239);
test_nonany!(u8, 240u8, test_nonanyu_240);
test_nonany!(u8, 241u8, test_nonanyu_241);
test_nonany!(u8, 242u8, test_nonanyu_242);
test_nonany!(u8, 243u8, test_nonanyu_243);
test_nonany!(u8, 244u8, test_nonanyu_244);
test_nonany!(u8, 245u8, test_nonanyu_245);
test_nonany!(u8, 246u8, test_nonanyu_246);
test_nonany!(u8, 247u8, test_nonanyu_247);
test_nonany!(u8, 248u8, test_nonanyu_248);
test_nonany!(u8, 249u8, test_nonanyu_249);
test_nonany!(u8, 250u8, test_nonanyu_250);
test_nonany!(u8, 251u8, test_nonanyu_251);
test_nonany!(u8, 252u8, test_nonanyu_252);
test_nonany!(u8, 253u8, test_nonanyu_253);
test_nonany!(u8, 254u8, test_nonanyu_254);
test_nonany!(u8, 255u8, test_nonanyu_255);
