use crate::{FmtArg, PanicFmt, PanicVal, StdWrapper};

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! nonzero_impls {
    ($(($int_ctor:ident, $ty:ty))*) => (
        $(
            impl PanicFmt for $ty {
                type This = Self;
                type Kind = crate::fmt::IsStdType;
                const PV_COUNT: usize = 1;
            }

            impl StdWrapper<&$ty> {
                #[doc = concat!(
                    "Converts this `", stringify!($ty), "` to a `PanicVal` array."
                )]
                pub const fn to_panicvals(self, fmtarg: FmtArg) -> [PanicVal<'static>; 1] {
                    [PanicVal::$int_ctor(self.0.get(), fmtarg)]
                }

                #[doc = concat!(
                    "Converts this `", stringify!($ty), "` to a `PanicVal`."
                )]
                pub const fn to_panicval(self, fmtarg: FmtArg) -> PanicVal<'static> {
                    PanicVal::$int_ctor(self.0.get(), fmtarg)
                }
            }
        )*

        impl_for_option!{
            $((for[], 'static, $ty, $ty))*
        }
    )
}

nonzero_impls! {
    (from_u8, NonZeroU8)
    (from_i8, NonZeroI8)
    (from_u16, NonZeroU16)
    (from_i16, NonZeroI16)
    (from_u32, NonZeroU32)
    (from_i32, NonZeroI32)
    (from_u64, NonZeroU64)
    (from_i64, NonZeroI64)
    (from_u128, NonZeroU128)
    (from_i128, NonZeroI128)
    (from_usize, NonZeroUsize)
    (from_isize, NonZeroIsize)
}
