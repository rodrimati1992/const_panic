use crate::{ArrayString, PanicVal};

use super::{FmtArg, IsCustomType, PanicFmt};

/// Whether a delimiter is the opening or closing one.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub enum DelimSide {
    // `(`/`[`/`{`
    Open,
    // `)`/`]`/`}`
    Close,
}

/// What delimiter this is `()`/`[]`/`{}`.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub enum DelimKind {
    Paren,
    Bracket,
    Brace,
}
/// What delimiter this is.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub struct Delimiter {
    pub kind: DelimKind,
    pub side: DelimSide,
}

/// `(`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const OPEN_PAREN: Delimiter = Delimiter {
    kind: DelimKind::Paren,
    side: DelimSide::Open,
};
/// `)`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const CLOSE_PAREN: Delimiter = Delimiter {
    kind: DelimKind::Paren,
    side: DelimSide::Close,
};
/// `[`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const OPEN_BRACKET: Delimiter = Delimiter {
    kind: DelimKind::Bracket,
    side: DelimSide::Open,
};
/// `]`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const CLOSE_BRACKET: Delimiter = Delimiter {
    kind: DelimKind::Bracket,
    side: DelimSide::Close,
};
/// `{`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const OPEN_BRACE: Delimiter = Delimiter {
    kind: DelimKind::Brace,
    side: DelimSide::Open,
};
/// `}`
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const INDENTATION_STEP: u8 = 4;

////////////////////////////////////////////////////////////////////////////////

/// A stack allocated string type that's convetible to `PanicVal`.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub type ShortString = ArrayString<16>;

////////////////////////////////////////////////////////////////////////////////

/// For computing the `PV_COUNT` of a struct or enum variant,
/// with the [`call`](PvCountForStruct::call) method.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
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
        // field-less structs and variants don't output the empty delimiter
        if self.field_amount == 0 {
            return 1;
        }

        const TYPE_NAME: usize = 1;
        const DELIM_TOKENS: usize = 2;
        let field_tokens = match self.delimiter {
            StructDelim::Tupled => self.field_amount,
            StructDelim::Braced => 2 * self.field_amount,
        };

        TYPE_NAME + DELIM_TOKENS + field_tokens + self.summed_pv_count
    }
}

/// The delimiter for structs and variants.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
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
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const COMMA_SEP: FieldSeparator<'_> = FieldSeparator::new(",", IsLastField::No);

/// A comma for use after the last field in a struct.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub const COMMA_TERM: FieldSeparator<'_> = FieldSeparator::new(",", IsLastField::Yes);

/// For telling `FieldSeparator` whether it comes after the last field or not.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
pub enum IsLastField {
    Yes,
    No,
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
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
