use crate::wrapper::Wrapper;

use core::marker::PhantomData;

/// Marker types for types that can be formatted by const panics.
pub trait PanicFmt {
    type This: ?Sized;
    type Kind;

    /// The length of the array returned in `Self::to_panicvals`.
    const PV_LEN: usize;

    const PROOF: IsPanicFmt<Self, Self::This, Self::Kind> = IsPanicFmt::NEW;
}

impl<'a, T: PanicFmt + ?Sized> PanicFmt for &'a T {
    type This = T::This;
    type Kind = T::Kind;
    const PV_LEN: usize = T::PV_LEN;
}

pub struct StdType;

pub struct CustomType;

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

impl<S: ?Sized, T: ?Sized> IsPanicFmt<S, T, StdType> {
    pub const fn coerce(self, x: &T) -> Wrapper<&T> {
        Wrapper(x)
    }
}

impl<S: ?Sized, T: ?Sized> IsPanicFmt<S, T, CustomType> {
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
