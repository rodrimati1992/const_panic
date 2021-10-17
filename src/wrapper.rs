use crate::{IntVal, PanicFmt, PanicVal};

#[repr(transparent)]
pub struct Wrapper<T>(pub T);

macro_rules! impl_panicfmt_array {
    (
        PV_LEN = $pv_len:expr;
        fn[$($impl:tt)*](&$self:ident: $ty:ty) -> $ret:ty {
            $($content:tt)*
        }
    ) => (
        impl<$($impl)*> PanicFmt for $ty {
            type This = Self;
            type Kind = crate::panic_fmt::StdType;
            const PV_LEN: usize = $pv_len;
        }

        impl<'s, $($impl)*> Wrapper<&'s $ty> {
            pub const fn to_panicvals($self: Self) -> $ret {
                $($content)*
            }
        }
    )
}

macro_rules! pick_first_ty {
    ($ty:ty $(, $__:ty)?) => {
        $ty
    };
}

macro_rules! impl_panicfmt_panicarg {
    (
        fn $panic_arg_ctor:ident[$($impl:tt)*](
            $this:ident:
            $ty:ty
        ) -> PanicVal<$pa_lt:lifetime>
        $panic_args:block


    )=>{
        impl PanicVal<'_> {
            pub const fn $panic_arg_ctor<$($impl)*>($this: $ty) -> PanicVal<$pa_lt>
            $panic_args
        }

        impl<$($impl)*> PanicFmt for $ty {
            type This = Self;
            type Kind = crate::panic_fmt::StdType;

            const PV_LEN: usize = 1;
        }

        impl<'s, $($impl)*> Wrapper<&'s $ty> {
            pub const fn to_panicvals(self: Self) -> [PanicVal<$pa_lt>;1] {
                [PanicVal::$panic_arg_ctor(*self.0)]
            }
            pub const fn to_panicval(self: Self) -> PanicVal<$pa_lt> {
                PanicVal::$panic_arg_ctor(*self.0)
            }
        }
    }
}

macro_rules! impl_panicfmt_int {
    ($panic_arg_ctor:ident, $intarg_contructor:ident, $ty:ty) => {
        impl_panicfmt_panicarg! {
            fn $panic_arg_ctor[](this: $ty) -> PanicVal<'static> {
                PanicVal::Int(IntVal::$intarg_contructor(this as _))
            }
        }
    };
}

impl_panicfmt_int! {from_u8, from_u128, u8}
impl_panicfmt_int! {from_u16, from_u128, u16}
impl_panicfmt_int! {from_u32, from_u128, u32}
impl_panicfmt_int! {from_u64, from_u128, u64}
impl_panicfmt_int! {from_u128, from_u128, u128}
impl_panicfmt_int! {from_usize, from_u128, usize}

impl_panicfmt_int! {from_i8, from_i128, i8}
impl_panicfmt_int! {from_i16, from_i128, i16}
impl_panicfmt_int! {from_i32, from_i128, i32}
impl_panicfmt_int! {from_i64, from_i128, i64}
impl_panicfmt_int! {from_i128, from_i128, i128}
impl_panicfmt_int! {from_isize, from_i128, isize}

impl_panicfmt_panicarg! {
    fn from_bool[](this: bool) -> PanicVal<'static> {
        PanicVal::Str(if this { "true" } else { "false" })
    }
}

macro_rules! impl_panicfmt_panicarg_unsized {
    (
        fn $panic_arg_ctor:ident[$($impl:tt)*](
            $this:ident:
            $ty:ty
        ) -> PanicVal<$pa_lt:lifetime>
        $panic_args:block


    )=>{
        impl PanicVal<'_> {
            pub const fn $panic_arg_ctor<$pa_lt, $($impl)*>($this: &$pa_lt $ty) -> PanicVal<$pa_lt>
            $panic_args
        }

        impl<$($impl)*> PanicFmt for $ty {
            type This = Self;
            type Kind = crate::panic_fmt::StdType;
            const PV_LEN: usize = 1;
        }

        impl<$pa_lt,     $($impl)*> Wrapper<&$pa_lt $ty> {
            pub const fn to_panicvals(self: Self) -> [PanicVal<$pa_lt>;1] {
                [PanicVal::$panic_arg_ctor(self.0)]
            }
            pub const fn to_panicval(self: Self) -> PanicVal<$pa_lt> {
                PanicVal::$panic_arg_ctor(self.0)
            }
        }
    }
}

impl_panicfmt_panicarg_unsized! {
    fn from_str[](this: str) -> PanicVal<'s> {
        PanicVal::Str(this)
    }
}

impl_panicfmt_array! {
    PV_LEN = N;
    fn['a, const N: usize](&self: [PanicVal<'a>; N]) -> &'s [PanicVal<'a>; N] {
        self.0
    }
}

impl_panicfmt_array! {
    PV_LEN = usize::MAX;
    fn['a](&self: [PanicVal<'a>]) -> &'s [PanicVal<'a>] {
        self.0
    }
}

impl<'a, 'b> Wrapper<&'a &'b [PanicVal<'b>]> {
    pub const fn deref_panic_vals(self) -> &'b [PanicVal<'b>] {
        *self.0
    }
}
impl<'a, 'b, const N: usize> Wrapper<&'a &'b [PanicVal<'b>; N]> {
    pub const fn deref_panic_vals(self) -> &'b [PanicVal<'b>] {
        *self.0
    }
}
impl<'b, const N: usize> Wrapper<&'b [PanicVal<'b>; N]> {
    pub const fn deref_panic_vals(self) -> &'b [PanicVal<'b>] {
        self.0
    }
}
