use crate::wrapper::Wrapper;

use core::marker::PhantomData;

/// Marker types for types that can be formatted by const panics.
pub trait PanicFmt {
    type This: ?Sized;
    type Kind;

    /// The length of the array returned in `Self::to_panicvals`.
    const PV_COUNT: usize;

    const PROOF: IsPanicFmt<Self, Self::This, Self::Kind> = IsPanicFmt::NEW;
}

impl<'a, T: PanicFmt + ?Sized> PanicFmt for &'a T {
    type This = T::This;
    type Kind = T::Kind;
    const PV_COUNT: usize = T::PV_COUNT;
}

pub struct IsStdType;

pub struct IsCustomType;

pub struct IsPanicFmt<S: ?Sized, T: ?Sized, K> {
    pub self_: PhantomData<fn() -> S>,
    pub this: PhantomData<fn() -> T>,
    pub kind: PhantomData<fn() -> K>,
    _priv: (),
}

impl<T: PanicFmt + ?Sized> IsPanicFmt<T, T::This, T::Kind> {
    pub const NEW: Self = Self {
        self_: PhantomData,
        this: PhantomData,
        kind: PhantomData,
        _priv: (),
    };
}

impl<S: ?Sized, T: ?Sized, K> IsPanicFmt<S, T, K> {
    pub const fn infer(self, _: &S) -> Self {
        self
    }
}

impl<S: ?Sized, T: ?Sized> IsPanicFmt<S, T, IsStdType> {
    pub const fn coerce(self, x: &T) -> Wrapper<&T> {
        Wrapper(x)
    }
}

impl<S: ?Sized, T: ?Sized> IsPanicFmt<S, T, IsCustomType> {
    pub const fn coerce(self, x: &T) -> &T {
        x
    }
}

impl<S: ?Sized, T: ?Sized, K> Copy for IsPanicFmt<S, T, K> {}
impl<S: ?Sized, T: ?Sized, K> Clone for IsPanicFmt<S, T, K> {
    fn clone(&self) -> Self {
        *self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone)]
#[non_exhaustive]
pub struct FmtArg {
    pub indentation: u8,
}

impl FmtArg {
    pub const DISPLAY: Self = Self { indentation: 0 };

    pub const DEBUG: Self = Self { indentation: 0 };

    pub const fn add_indentation(mut self, indentation: u8) -> Self {
        self.indentation += indentation;
        self
    }
}
