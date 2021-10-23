use crate::{
    fmt::{FmtArg, PanicFmt},
    panic_val::{PanicVal, PanicVariant},
    StdWrapper,
};

macro_rules! impl_panicfmt_array {
    ($(($variant:ident, $panicval_ctor:ident, $ty:ty)),* $(,)*) => {

        #[derive(Copy, Clone)]
        #[non_exhaustive]
        pub(crate) struct Slice<'s> {
            pub(crate) fmtarg: FmtArg,
            pub(crate) vari: SliceV<'s>,
        }

        #[derive(Copy, Clone)]
        #[non_exhaustive]
        pub(crate) enum SliceV<'s> {
            $(
                $variant(&'s [$ty]),
            )*
        }


        impl<'s> Slice<'s> {
            // length in elements
            pub(crate) const fn arr_len(self) -> usize {
                match self.vari {
                    $(
                        SliceV::$variant(arr) => arr.len(),
                    )*
                }
            }
            pub(crate) const fn get(self, index: usize) -> PanicVal<'s> {
                match self.vari {
                    $(
                        SliceV::$variant(arr) => {
                            let elem: &'s <$ty as PanicFmt>::This = &arr[index];
                            StdWrapper(elem).to_panicval(self.fmtarg)
                        },
                    )*
                }
            }
        }


        #[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
        impl<'s> PanicVal<'s> {
            $(
                /// Constructs a `PanicVal` from a slice.
                pub const fn $panicval_ctor(this: &'s [$ty], mut fmtarg: FmtArg) -> PanicVal<'s> {
                    fmtarg = fmtarg.indent();
                    if this.is_empty() {
                        fmtarg = fmtarg.set_alternate(false);
                    }
                    PanicVal::__new(
                        PanicVariant::Slice(Slice{
                            fmtarg,
                            vari: SliceV::$variant(this),
                        }),
                        fmtarg
                    )
                }
            )*
        }

        $(
            impl<'s> PanicFmt for [$ty] {
                type This = Self;
                type Kind = crate::fmt::IsStdType;
                const PV_COUNT: usize = 1;
            }
            impl<'s, const LEN: usize> PanicFmt for [$ty; LEN] {
                type This = Self;
                type Kind = crate::fmt::IsStdType;
                const PV_COUNT: usize = 1;
            }

            #[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
            impl<'s> StdWrapper<&'s [$ty]> {
                /// Converts the slice to a single-element `PanicVal` array.
                pub const fn to_panicvals(self: Self, f:FmtArg) -> [PanicVal<'s>;1] {
                    [PanicVal::$panicval_ctor(self.0, f)]
                }
                /// Converts the slice to a `PanicVal`.
                pub const fn to_panicval(self: Self, f:FmtArg) -> PanicVal<'s> {
                    PanicVal::$panicval_ctor(self.0, f)
                }
            }

            #[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
            impl<'s, const LEN: usize> StdWrapper<&'s [$ty; LEN]> {
                /// Converts the array to a single-element `PanicVal` array.
                pub const fn to_panicvals(self: Self, f:FmtArg) -> [PanicVal<'s>;1] {
                    [PanicVal::$panicval_ctor(self.0, f)]
                }
                /// Converts the array to a `PanicVal`.
                pub const fn to_panicval(self: Self, f:FmtArg) -> PanicVal<'s> {
                    PanicVal::$panicval_ctor(self.0, f)
                }
            }
        )*

    };
}

impl_panicfmt_array! {
    (U8, from_slice_u8, u8),
    (U16, from_slice_u16, u16),
    (U32, from_slice_u32, u32),
    (U64, from_slice_u64, u64),
    (U128, from_slice_u128, u128),
    (Usize, from_slice_usize, usize),
    (I8, from_slice_i8, i8),
    (I16, from_slice_i16, i16),
    (I32, from_slice_i32, i32),
    (I64, from_slice_i64, i64),
    (I128, from_slice_i128, i128),
    (Isize, from_slice_isize, isize),
    (Bool, from_slice_bool, bool),
    (Str, from_slice_str, &'s str),
}

#[derive(Copy, Clone)]
pub(crate) struct SliceIter<'b, 's> {
    slice: &'b Slice<'s>,
    state: IterState,
    arr_len: usize,
}

#[derive(Copy, Clone)]
enum IterState {
    Start,
    Index(usize),
    End,
}

impl<'s> Slice<'s> {
    pub(crate) const fn iter<'b>(&'b self) -> SliceIter<'b, 's> {
        SliceIter {
            slice: self,
            state: IterState::Start,
            arr_len: self.arr_len(),
        }
    }
}

impl<'b, 's> SliceIter<'b, 's> {
    pub(crate) const fn next(mut self) -> ([PanicVal<'s>; 2], Option<Self>) {
        let slice = self.slice;
        let ret = match self.state {
            IterState::Start => {
                self.state = if self.arr_len == 0 {
                    IterState::End
                } else {
                    IterState::Index(0)
                };

                [
                    crate::fmt::OpenBracket.to_panicval(slice.fmtarg),
                    PanicVal::EMPTY,
                ]
            }
            IterState::Index(x) => {
                let comma = if x + 1 == self.arr_len {
                    self.state = IterState::End;
                    crate::fmt::COMMA_TERM
                } else {
                    self.state = IterState::Index(x + 1);
                    crate::fmt::COMMA_SEP
                }
                .to_panicval(slice.fmtarg);

                [slice.get(x), comma]
            }
            IterState::End => {
                let close_brace = crate::fmt::CloseBracket.to_panicval(slice.fmtarg.unindent());
                return ([close_brace, PanicVal::EMPTY], None);
            }
        };

        (ret, Some(self))
    }
}
