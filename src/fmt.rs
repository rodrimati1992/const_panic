//! Formatting-related items
//!
//! For examples, [you can look at the ones for PanicFmt](PanicFmt#examples)
//!

use crate::{wrapper::Wrapper, ArrayString, PanicVal};

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
pub struct IsPanicFmt<S: ?Sized, T: ?Sized, K> {
    self_: PhantomData<fn() -> S>,
    this: PhantomData<fn() -> T>,
    kind: PhantomData<fn() -> K>,
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

/// Carries all of the configuration for formatting functions.
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub struct FmtArg {
    /// How much indentation is needed for a field/array element.
    ///
    /// This is only used by types that call  
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

    /// A `FmtArg` with no indentation and `Debug` formatting.
    pub const DEBUG: Self = Self {
        indentation: 0,
        is_alternate: false,
        fmt_kind: FmtKind::Debug,
    };

    /// A `FmtArg` with no indentation and alternate `Display` formatting.
    pub const ALT_DISPLAY: Self = Self::DISPLAY.set_alternate(true);

    /// A `FmtArg` with no indentation and alternate `Debug` formatting.
    pub const ALT_DEBUG: Self = Self::DEBUG.set_alternate(true);

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

    /// Sets wether alternate formatting is enabled
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

////////////////////////////////////////////////////////////////////////////////

/// What kind of formatting to do, either `Display` or `Debug`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FmtKind {
    Debug,
    Display,
}

////////////////////////////////////////////////////////////////////////////////

/// Whether a delimiter is the opening or closing one.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DelimSide {
    // `(`/`[`/`{`
    Open,
    // `)`/`]`/`}`
    Close,
}

/// What delimiter this is `()`/`[]`/`{}`.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}
/// What delimiter this is.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Delimiter {
    pub kind: DelimKind,
    pub side: DelimSide,
}

/// `(`
pub const OPEN_PAREN: Delimiter = Delimiter {
    kind: DelimKind::Paren,
    side: DelimSide::Open,
};
/// `)`
pub const CLOSE_PAREN: Delimiter = Delimiter {
    kind: DelimKind::Paren,
    side: DelimSide::Close,
};
/// `[`
pub const OPEN_BRACKET: Delimiter = Delimiter {
    kind: DelimKind::Bracket,
    side: DelimSide::Open,
};
/// `]`
pub const CLOSE_BRACKET: Delimiter = Delimiter {
    kind: DelimKind::Bracket,
    side: DelimSide::Close,
};
/// `{`
pub const OPEN_BRACE: Delimiter = Delimiter {
    kind: DelimKind::Brace,
    side: DelimSide::Open,
};
/// `}`
pub const CLOSE_BRACE: Delimiter = Delimiter {
    kind: DelimKind::Brace,
    side: DelimSide::Close,
};

impl Delimiter {
    pub const fn to_panicvals(self, f: FmtArg) -> [PanicVal<'static>; 1] {
        [self.to_panicval(f)]
    }
    pub const fn to_panicval(self, f: FmtArg) -> PanicVal<'static> {
        match (self, f.is_alternate) {
            (self::OPEN_PAREN, false) => PanicVal::write_str("("),
            (self::CLOSE_PAREN, false) => PanicVal::write_str(")"),
            (self::OPEN_BRACKET, false) => PanicVal::write_str("["),
            (self::CLOSE_BRACKET, false) => PanicVal::write_str("]"),
            (self::OPEN_BRACE, false) => PanicVal::write_str(" { "),
            (self::CLOSE_BRACE, false) => PanicVal::write_str(" }"),
            (self::OPEN_PAREN, true) => PanicVal::write_str("(\n").with_rightpad(f),
            (self::CLOSE_PAREN, true) => PanicVal::write_str(")").with_leftpad(f),
            (self::OPEN_BRACKET, true) => PanicVal::write_str("[\n").with_rightpad(f),
            (self::CLOSE_BRACKET, true) => PanicVal::write_str("]").with_leftpad(f),
            (self::OPEN_BRACE, true) => PanicVal::write_str(" {\n").with_rightpad(f),
            (self::CLOSE_BRACE, true) => PanicVal::write_str("}").with_leftpad(f),
        }
    }
}

/*
Foo { x: [3, 5, 8, 13], y: 21, z: (34, 55) }
Foo {
    x: [
        3,
        5,
        8,
        13,
    ],
    y: 21,
    z: (
        34,
        55,
    ),
}
*/

impl PanicFmt for Delimiter {
    type This = Self;
    type Kind = IsCustomType;
    const PV_COUNT: usize = 1;
}

////////////////////////////////////////////////////////////////////////////////

/// How much indentation (in spaces) is added with `increment_indentation`.
pub const INDENTATION_STEP: u8 = 4;

////////////////////////////////////////////////////////////////////////////////

/// A stack allocated string type that's convetible to `PanicVal`.
pub type ShortString = ArrayString<16>;

////////////////////////////////////////////////////////////////////////////////

/// For computing the `PV_COUNT` of a struct or enum variant,
/// with the [`call`](PvCountForStruct::call) method.
pub struct PvCountForStruct {
    /// The amount of fields in the struct
    pub field_amount: usize,
    /// The summed up amount of `PanicVal`s that all the fields produce.
    ///
    /// Eg: for a struct with `Bar` and `Qux` fields, this would be
    /// `<Bar as PanicFmt>::PV_COUNT + <Qux as PanicFmt>::PV_COUNT`,
    ///
    pub summed_pv_count: usize,
    ///
    pub delimiter: StructDelim,
}

impl PvCountForStruct {
    pub const fn call(&self) -> usize {
        const DELIMITER_PVCOUNT: usize = 2;

        // field-less structs and variants don't output the empty delimiter
        if self.field_amount == 0 {
            1
        } else {
            1 + 2 + 2 * self.field_amount + self.summed_pv_count
        }
    }
}

/// The delimiter for structs and variants.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum StructDelim {
    Tupled,
    Braced,
}

impl StructDelim {
    pub const fn get_open_and_close(self) -> (Delimiter, Delimiter) {
        let open = Delimiter {
            kind: match self {
                Self::Tupled => DelimKind::Paren,
                Self::Braced => DelimKind::Brace,
            },
            side: DelimSide::Open,
        };

        (
            open,
            Delimiter {
                side: DelimSide::Close,
                ..open
            },
        )
    }
}

////////////////////////////////////////////////////////////////////////////////

/// A comma separator for use between fields in a struct.
pub const COMMA_SEP: FieldSeparator<'_> = FieldSeparator::new(",", IsLastField::No);

/// A comma for use after the last field in a struct.
pub const COMMA_TERM: FieldSeparator<'_> = FieldSeparator::new(",", IsLastField::Yes);

/// For telling `FieldSeparator` whether it comes after the last field or not.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IsLastField {
    Yes,
    No,
}

#[derive(Copy, Clone)]
pub struct FieldSeparator<'a>(&'a str, IsLastField);

impl<'a> FieldSeparator<'a> {
    ///
    ///
    /// # Panics
    ///
    /// Panics if `string` is longer than 12 bytes.
    ///
    pub const fn new(string: &'a str, is_last_field: IsLastField) -> Self {
        if string.len() > 12 {
            crate::concat_panic(&[&[
                PanicVal::write_str("expected a string shorter than 12 bytes,"),
                PanicVal::write_str("actual length: "),
                PanicVal::from_usize(string.len(), FmtArg::DISPLAY),
                PanicVal::write_str(", string: "),
                PanicVal::from_str(string, FmtArg::DEBUG),
            ]])
        }

        FieldSeparator(string, is_last_field)
    }

    pub const fn to_panicvals(self, f: FmtArg) -> [PanicVal<'static>; 1] {
        [PanicVal::from_element_separator(self.0, self.1, f)]
    }

    pub const fn to_panicval(self, f: FmtArg) -> PanicVal<'static> {
        PanicVal::from_element_separator(self.0, self.1, f)
    }
}

impl PanicFmt for FieldSeparator<'_> {
    type This = Self;
    type Kind = IsCustomType;
    const PV_COUNT: usize = 1;
}
