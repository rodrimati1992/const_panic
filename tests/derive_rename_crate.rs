#![cfg(feature = "derive")]

extern crate const_panic as cpanic;
extern crate std as const_panic;

use cpanic::{FmtArg, PanicFmt};

macro_rules! fmt_flatten {
    ($fmt:expr, $val:expr) => {
        cpanic::ArrayString::<256>::from_panicvals(&$val.to_panicvals($fmt)).unwrap()
    };
}

#[test]
fn derive_struct_formatting() {
    let foo = Foo {
        x: &[3, 5, 8, 13],
        y: 21,
    };
    assert_eq!(fmt_flatten!(FmtArg::DEBUG, foo), *format!("{:?}", foo));
    assert_eq!(fmt_flatten!(FmtArg::ALT_DEBUG, foo), *format!("{:#?}", foo));
}

#[derive(Debug, PanicFmt)]
#[pfmt(crate = ::cpanic)]
struct Foo<'a> {
    x: &'a [u8],
    y: u8,
}

#[test]
fn derive_enum_formatting() {
    for val in [Qux::Up, Qux::Down { x: 21, y: 34 }, Qux::Left(55)] {
        assert_eq!(fmt_flatten!(FmtArg::DEBUG, val), *format!("{:?}", val));
        assert_eq!(fmt_flatten!(FmtArg::ALT_DEBUG, val), *format!("{:#?}", val));
    }
}

#[derive(Debug, PanicFmt)]
#[pfmt(crate = cpanic)]
enum Qux {
    Up,
    Down { x: u32, y: u32 },
    Left(u64),
}
