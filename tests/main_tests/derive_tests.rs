use const_panic::{FmtArg, PanicFmt};

use const_panic::test_utils::MyPhantomData;

macro_rules! fmt_flatten {
    ($($args:tt)*) => (
        const_panic::ArrayString::<256>::from_panicvals(
            &const_panic::flatten_panicvals!($($args)*)
        ).unwrap()
    )
}

#[test]
fn struct_formatting() {
    let foo = Foo {
        x: &[3, 5, 8, 13],
        y: 21,
        z: Bar(false, true),
        w: Baz {
            h: &["hi", "hel\nlo"],
        },
    };

    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; Foo => foo),
        *format!("{:?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::ALT_DEBUG; Foo => foo),
        *format!("{:#?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::HEX; Foo => foo),
        *format!("{:X?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::ALT_HEX; Foo => foo),
        *format!("{:#X?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::DISPLAY; Foo => foo),
        "Foo { x: [3, 5, 8, 13], y: 21, z: Bar(false, true), w: Baz { h: [hi, hel\nlo] } }"
    );

    assert_eq!(
        fmt_flatten!(FmtArg::ALT_DISPLAY; Foo => foo),
        concat!(
            "Foo {\n",
            "    x: [\n",
            "        3,\n",
            "        5,\n",
            "        8,\n",
            "        13,\n",
            "    ],\n",
            "    y: 21,\n",
            "    z: Bar(\n",
            "        false,\n",
            "        true,\n",
            "    ),\n",
            "    w: Baz {\n",
            "        h: [\n",
            "            hi,\n",
            "            hel\n",
            "lo,\n",
            "        ],\n",
            "    },\n",
            "}",
        )
    );
}

#[test]
fn nondebug_formatting() {
    let foo = DispStruct {
        x: &[3, 5, 8, 13],
        y: 21,
    };

    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; DispStruct => foo),
        *format!("{:?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::ALT_DEBUG; DispStruct => foo),
        *format!("{:#?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::HEX; DispStruct => foo),
        *format!("{:X?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::ALT_HEX; DispStruct => foo),
        *format!("{:#X?}", foo)
    );

    assert_eq!(
        fmt_flatten!(FmtArg::DISPLAY; DispStruct => foo),
        "hello: 21"
    );

    assert_eq!(
        fmt_flatten!(FmtArg::ALT_DISPLAY; DispStruct => foo),
        "hello: 21"
    );

}

#[derive(Debug, PanicFmt)]
struct Foo<'a> {
    x: &'a [u8],
    y: u8,
    z: Bar,
    w: Baz,
}



#[derive(Debug, PanicFmt)]
#[pfmt(display_fmt = Self::fmt_display)]
struct DispStruct<'a> {
    x: &'a [u8],
    y: u8,
}

impl DispStruct<'_> {
    const fn fmt_display(
        &self, 
        fmtarg: FmtArg,
    ) -> [const_panic::PanicVal<'_>; DispStruct::PV_COUNT] {
        const_panic::flatten_panicvals!(fmtarg, DispStruct::PV_COUNT;
            "hello: ", self.y
        )
    }
}

#[derive(Debug, PanicFmt)]
struct Bar(bool, bool);

#[derive(Debug, PanicFmt)]
struct Baz {
    h: &'static [&'static str],
}

#[test]
fn const_gen_struct_formatting() {
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenS<0> => ConstGenS([])),
        "ConstGenS([])"
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenS<1> => ConstGenS([3])),
        "ConstGenS([3])"
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenS<2> => ConstGenS([3, 5])),
        "ConstGenS([3, 5])"
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenS<3> => ConstGenS([3, 5, 8])),
        "ConstGenS([3, 5, 8])"
    );
}

#[derive(Debug, PanicFmt)]
struct ConstGenS<const N: usize>([u32; N]);

#[test]
fn enum_formatting() {
    const_panic::inline_macro! {
        (u8 = Qux::Up),
        (u16 = Qux::Down { x: 21, y: 34 }),
        (u32 = Qux::Left(55));
        ($T:ty = $val:expr) =>

        let val: Qux<$T> = $val;

        assert_eq!(
            fmt_flatten!(FmtArg::DEBUG; Qux<$T> => val),
            *format!("{:?}", val)
        );

        assert_eq!(
            fmt_flatten!(FmtArg::ALT_DEBUG; Qux<$T> => val),
            *format!("{:#?}", val)
        );
    }
}

#[derive(Debug, PanicFmt)]
#[pfmt(impl Qux<u8>)]
#[pfmt(impl Qux<u16>)]
#[pfmt(impl Qux<u32>)]
enum Qux<T> {
    Up,
    Down { x: T, y: T },
    Left(u64),
}

#[test]
fn display_enum_formatting() {
    for val in [DispEnum::Up, DispEnum::Down] {
        assert_eq!(
            fmt_flatten!(FmtArg::DEBUG; DispEnum => val),
            *format!("{:?}", val)
        );

        assert_eq!(
            fmt_flatten!(FmtArg::ALT_DEBUG; DispEnum => val),
            *format!("{:#?}", val)
        );
    }

    assert_eq!(
        fmt_flatten!(FmtArg::DISPLAY; DispEnum => DispEnum::Up),
        "up",
    );
    assert_eq!(
        fmt_flatten!(FmtArg::ALT_DISPLAY; DispEnum => DispEnum::Up),
        "UP",
    );

    assert_eq!(
        fmt_flatten!(FmtArg::DISPLAY; DispEnum => DispEnum::Down),
        "down",
    );
    assert_eq!(
        fmt_flatten!(FmtArg::ALT_DISPLAY; DispEnum => DispEnum::Down),
        "DOWN",
    );
}

#[derive(Debug, PanicFmt)]
#[pfmt(display_fmt = Self::fmt_display)]
enum DispEnum {
    Up,
    Down,
}

impl DispEnum {
    const fn fmt_display(
        &self, 
        fmtarg: FmtArg,
    ) -> [const_panic::PanicVal<'_>; DispEnum::PV_COUNT] {
        const_panic::flatten_panicvals!(fmtarg, DispEnum::PV_COUNT;
            display: match (self, fmtarg.is_alternate) {
                (Self::Up, false) => "up",
                (Self::Up, true) => "UP",
                (Self::Down, false) => "down",
                (Self::Down, true) => "DOWN",
            }
        )
    }
}


#[test]
fn const_gen_enum_formatting() {
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenE<0> => ConstGenE::X(&[])),
        "X([])"
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenE<1> => ConstGenE::X(&[3])),
        "X([3])"
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenE<2> => ConstGenE::X(&[3, 5])),
        "X([3, 5])"
    );
    assert_eq!(
        fmt_flatten!(FmtArg::DEBUG; ConstGenE<3> => ConstGenE::X(&[3, 5, 8])),
        "X([3, 5, 8])"
    );
}

#[derive(Debug, PanicFmt)]
enum ConstGenE<'a, const N: usize> {
    X(&'a [u32; N]),
}

#[test]
fn ignored_generic_params_formatting() {
    #[derive(Debug)]
    struct NoFmt;

    const_panic::inline_macro! {
        (implicit_gpi),
        (explicit_gpi);
        ($module:ident) =>
        {
            use $module::IgnoredGenericParams as IGP;

            let foo = IGP(&3, MyPhantomData::NEW, MyPhantomData::NEW);

            assert_eq!(
                fmt_flatten!(FmtArg::DEBUG; IGP<NoFmt, NoFmt, 10, 'c'> => foo),
                *format!("{:?}", foo)
            );

            assert_eq!(
                fmt_flatten!(FmtArg::ALT_DEBUG; IGP<NoFmt, NoFmt, 10, 'c'> => foo),
                *format!("{:#?}", foo)
            );
        }
    }
}



#[test]
fn struct_panicvals_lower_bound() {
    assert_eq!(LbStruct::PV_COUNT, 100);
}

#[test]
fn enum_panicvals_lower_bound() {
    assert_eq!(LbEnum::PV_COUNT, 101);
}

#[derive(Debug, PanicFmt)]
#[pfmt(panicvals_lower_bound = 100)]
struct LbStruct<'a> {
    x: &'a [u8],
    y: u8,
}

#[derive(Debug, PanicFmt)]
#[pfmt(panicvals_lower_bound = 101)]
enum LbEnum {
    Up,
    Down,
}


mod implicit_gpi {
    use super::*;

    #[derive(Debug, PanicFmt)]
    #[pfmt(ignore(A, B))]
    pub struct IgnoredGenericParams<'a, A, B, const X: u32, const Y: char>(
        pub &'a u32,
        pub MyPhantomData<A>,
        pub MyPhantomData<B>,
    );
}

mod explicit_gpi {
    use super::*;

    #[derive(Debug, PanicFmt)]
    #[pfmt(ignore(A = u32, B = u64, X = 100, Y = '_'))]
    pub struct IgnoredGenericParams<'a, A, B, const X: u32, const Y: char>(
        pub &'a u32,
        pub MyPhantomData<A>,
        pub MyPhantomData<B>,
    );
}

#[test]
fn ignored_generic_params_and_impl_formatting() {
    #[derive(Debug)]
    struct NoFmt;

    const_panic::inline_macro! {
        (IgnoredAndImpl<i8, u8, 8, 'X'>),
        (IgnoredAndImpl<i16, u16, 13, 'Y'>),
        (IgnoredAndImpl<i16, u32, 13, 'Y'>);
        ($ty:path) =>
        {
            let foo = $ty(&3, MyPhantomData::NEW, MyPhantomData::NEW);

            assert_eq!(
                fmt_flatten!(FmtArg::DEBUG; $ty => foo),
                *format!("{:?}", foo)
            );

            assert_eq!(
                fmt_flatten!(FmtArg::ALT_DEBUG; $ty => foo),
                *format!("{:#?}", foo)
            );
        }
    }
}

#[derive(Debug, PanicFmt)]
#[pfmt(ignore(A))]
#[pfmt(impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u8, H, I>)]
#[pfmt(impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u16, H, I>)]
#[pfmt(impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u32, H, I>)]
pub struct IgnoredAndImpl<'a, A, B, const X: u32, const Y: char>(
    pub &'a u32,
    pub MyPhantomData<A>,
    pub MyPhantomData<B>,
);
