use const_panic::{FmtArg, PanicFmt};

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

#[derive(Debug, PanicFmt)]
struct Foo<'a> {
    x: &'a [u8],
    y: u8,
    z: Bar,
    w: Baz,
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
    for val in [Qux::Up, Qux::Down { x: 21, y: 34 }, Qux::Left(55)] {
        assert_eq!(
            fmt_flatten!(FmtArg::DEBUG; Qux => val),
            *format!("{:?}", val)
        );

        assert_eq!(
            fmt_flatten!(FmtArg::ALT_DEBUG; Qux => val),
            *format!("{:#?}", val)
        );
    }
}

#[derive(Debug, PanicFmt)]
enum Qux {
    Up,
    Down { x: u32, y: u32 },
    Left(u64),
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
