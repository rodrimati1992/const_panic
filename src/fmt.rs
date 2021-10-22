//! Formatting-related items
//!
//! For examples, [you can look at the ones for PanicFmt](PanicFmt#examples)
//!

#[cfg(feature = "non_basic")]
mod non_basic_fmt;

#[cfg(feature = "non_basic")]
pub use self::non_basic_fmt::*;

use crate::wrapper::Wrapper;

use core::marker::PhantomData;

/// Marker types for types that can be formatted by const panics.
///
/// # Implementor
///
/// Implementors are expected to also define this inherent method to format the type:
/// ```rust
/// # use const_panic::fmt::{FmtArg, IsCustomType, PanicFmt};
/// # use const_panic::PanicVal;
/// # struct Foo;
/// # impl Foo {
/// const fn to_panicvals<'a>(&'a self, f: FmtArg) -> [PanicVal<'a>; <Self as PanicFmt>::PV_COUNT]
/// # { loop{} }
/// # }
/// # impl PanicFmt for Foo {
/// #   type This = Self;
/// #   type Kind = IsCustomType;
/// #   const PV_COUNT: usize = 1;
/// # }
/// ```
/// The returned [`PanicVal`](crate::PanicVal) can also be `PanicVal<'static>`.
///
///
///
pub trait PanicFmt {
    /// The type after dereferencing all references.
    ///
    /// User-defined should generally set this to `Self`.
    type This: ?Sized;
    /// Whether this is a user-defined type or standard library type.
    ///
    /// User-defined should generally set this to [`IsCustomType`].
    type Kind;

    /// The length of the array returned in `Self::to_panicvals`
    /// (an inherent method that formats the type for panic messages).
    const PV_COUNT: usize;

    /// A marker type that proves that `Self` implements `PanicFmt`.
    ///
    /// Used by const_panic macros to coerce both standard library and
    /// user-defined types into some type that has a `to_panicvals` method.
    const PROOF: IsPanicFmt<Self, Self::This, Self::Kind> = IsPanicFmt::NEW;
}

impl<'a, T: PanicFmt + ?Sized> PanicFmt for &'a T {
    type This = T::This;
    type Kind = T::Kind;
    const PV_COUNT: usize = T::PV_COUNT;
}

/// Marker type used as the [`PanicFmt::Kind`] associated type for std types.
pub struct IsStdType;

/// Marker type used as the [`PanicFmt::Kind`] for user-defined types.
pub struct IsCustomType;

/// A marker type that proves that `S` implements
/// [`PanicFmt<This = T, Kind = K>`](PanicFmt).
///
/// Used by const_panic macros to coerce both standard library and
/// user-defined types into some type that has a `to_panicvals` method.
///
pub struct IsPanicFmt<S: ?Sized, T: ?Sized, K> {
    self_: PhantomData<fn() -> S>,
    this: PhantomData<fn() -> T>,
    kind: PhantomData<fn() -> K>,
    _priv: (),
}

impl<T: PanicFmt + ?Sized> IsPanicFmt<T, T::This, T::Kind> {
    /// Constucts an `IsPanicFmt`
    pub const NEW: Self = Self {
        self_: PhantomData,
        this: PhantomData,
        kind: PhantomData,
        _priv: (),
    };
}

impl<S: ?Sized, T: ?Sized, K> IsPanicFmt<S, T, K> {
    /// Infers the `S` type parameter with the argument.
    ///
    /// Because the only ways to construct `IsPanicFmt`
    /// use `IsPanicFmt<S, S::This, S::Kind>`,
    /// the other type parameters are inferred along with `S`.
    pub const fn infer(self, _: &S) -> Self {
        self
    }
}

impl<S: ?Sized, T: ?Sized> IsPanicFmt<S, T, IsStdType> {
    /// For coercing `&T` to `Wrapper<&T>`.
    pub const fn coerce(self, x: &T) -> Wrapper<&T> {
        Wrapper(x)
    }
}

impl<S: ?Sized, T: ?Sized> IsPanicFmt<S, T, IsCustomType> {
    /// For coercing `&T` (with any amount of stacked references) to `&T`.
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

/// Carries all of the configuration for formatting functions.
///
/// # Example
///
#[cfg_attr(feature = "non_basic", doc = "```rust")]
#[cfg_attr(not(feature = "non_basic"), doc = "```ignore")]
/// use const_panic::{ArrayString, FmtArg, Wrapper};
///
/// // `Wrapper` wraps references to std types to provide their `to_panicvals` methods
/// let array = Wrapper(&["3", "foo\nbar", "\0qux"]);
///
/// // Debug formatting
/// assert_eq!(
///     ArrayString::<99>::from_panicvals(&array.to_panicvals(FmtArg::DEBUG)).unwrap(),
///     r#"["3", "foo\nbar", "\x00qux"]"#
/// );
///
/// // Alternate-Debug formatting
/// assert_eq!(
///     ArrayString::<99>::from_panicvals(&array.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///     concat!(
///         "[\n",
///         "    \"3\",\n",
///         "    \"foo\\nbar\",\n",
///         "    \"\\x00qux\",\n",
///         "]",
///     )
/// );
///
/// // Display formatting
/// assert_eq!(
///     ArrayString::<99>::from_panicvals(&array.to_panicvals(FmtArg::DISPLAY)).unwrap(),
///     "[3, foo\nbar, \x00qux]"
/// );
///
/// // Alternate-Display formatting
/// assert_eq!(
///     ArrayString::<99>::from_panicvals(&array.to_panicvals(FmtArg::ALT_DISPLAY)).unwrap(),
///     concat!(
///         "[\n",
///         "    3,\n",
///         "    foo\n",
///         "bar,\n",
///         "    \x00qux,\n",
///         "]",
///     )
/// );
///
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub struct FmtArg {
    /// How much indentation is needed for a field/array element.
    ///
    /// Indentation is used by [`fmt::Delimiter`](crate::fmt::Delimiter)
    /// and by [`fmt::Separator`](crate::fmt::Separator),
    /// when the [`is_alternate` field](#structfield.is_alternate) flag is enabled.
    pub indentation: u8,
    /// Whether alternate formatting is being used.
    pub is_alternate: bool,
    /// Whether this is intended to be `Display` or `Debug` formatted.
    pub fmt_kind: FmtKind,
}

impl FmtArg {
    /// A `FmtArg` with no indentation and `Display` formatting.
    pub const DISPLAY: Self = Self {
        indentation: 0,
        fmt_kind: FmtKind::Display,
        is_alternate: false,
    };

    /// A `FmtArg` with `Debug` formatting and no indentation.
    pub const DEBUG: Self = Self {
        indentation: 0,
        is_alternate: false,
        fmt_kind: FmtKind::Debug,
    };

    /// A `FmtArg` with alternate `Display` formatting and no indentation.
    pub const ALT_DISPLAY: Self = Self::DISPLAY.set_alternate(true);

    /// A `FmtArg` with alternate `Debug` formatting and no indentation.
    pub const ALT_DEBUG: Self = Self::DEBUG.set_alternate(true);

    /// Sets whether alternate formatting is enabled
    pub const fn set_alternate(mut self, is_alternate: bool) -> Self {
        self.is_alternate = is_alternate;
        self
    }

    /// Changes the formatting to `Display`.
    pub const fn set_display(mut self) -> Self {
        self.fmt_kind = FmtKind::Display;
        self
    }

    /// Changes the formatting to `Debug`.
    pub const fn set_debug(mut self) -> Self {
        self.fmt_kind = FmtKind::Debug;
        self
    }
}

#[cfg(feature = "non_basic")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
impl FmtArg {
    /// Increments the indentation by [`INDENTATION_STEP`] spaces.
    pub const fn indent(mut self) -> Self {
        self.indentation += INDENTATION_STEP;
        self
    }

    /// Decrement the indentation by [`INDENTATION_STEP`] spaces.
    pub const fn unindent(mut self) -> Self {
        self.indentation = self.indentation.saturating_sub(INDENTATION_STEP);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

/// What kind of formatting to do, either `Display` or `Debug`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FmtKind {
    /// `Debug` formatting
    Debug,
    /// `Display` formatting
    Display,
}
