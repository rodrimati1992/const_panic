#![allow(non_local_definitions)]

use const_panic::FmtArg;

use const_panic::test_utils::MyPhantomData;

#[test]
fn struct_formatting() {
    let array = [3, 5, 8, 13];

    let foo = Foo {
        x: &array,
        y: 21,
        z: Bar(false, true),
        w: Baz,
        a: Qux::Up,
        b: Qux::Down { x: 21, y: 34 },
        c: Qux::Left(55),
    };

    assert_eq!(trunc_fmt!(999;FmtArg::DEBUG; foo), *format!("{:?}", foo));

    assert_eq!(
        trunc_fmt!(999;FmtArg::ALT_DEBUG; foo),
        *format!("{:#?}", foo)
    );

    assert_eq!(trunc_fmt!(999;FmtArg::HEX; foo), *format!("{:X?}", foo));

    assert_eq!(
        trunc_fmt!(999;FmtArg::ALT_HEX; foo),
        *format!("{:#X?}", foo)
    );

    let _: [const_panic::PanicVal<'_>; 1] = foo.w.to_panicvals(FmtArg::DEBUG);

    let _: [const_panic::PanicVal<'_>; <Qux<u8> as const_panic::PanicFmt>::PV_COUNT] =
        foo.b.to_panicvals(FmtArg::DEBUG);
}

#[derive(Debug)]
struct Foo<'a> {
    x: &'a [u8],
    y: u8,
    z: Bar,
    w: Baz,
    a: Qux<u8>,
    b: Qux<u16>,
    c: Qux<u32>,
}

const_panic::impl_panicfmt! {
    struct Foo<'a> {
        x: &[u8],
        y: u8,
        z: Bar,
        w: Baz,
        a: Qux<u8>,
        b: Qux<u16>,
        c: Qux<u32>,
    }

    (impl Foo<'_>)
}

#[derive(Debug)]
struct Bar(bool, bool);

const_panic::impl_panicfmt! {
    struct Bar(bool, bool);
}

#[derive(Debug)]
struct Baz;

const_panic::impl_panicfmt! {
    struct Baz;
}

#[derive(Debug)]
enum Qux<T> {
    Up,
    Down { x: T, y: T },
    Left(u64),
}

const_panic::impl_panicfmt! {
    enum Qux<T> {
        Up,
        Down{
            x: T,
            y: T,
        },
        Left(u64),
    }

    (impl Qux<u8> )
    (impl Qux<u16>)
    (impl Qux<u32>)
}

#[test]
fn to_panicvals_lifetime_test() {
    let struct_tup = StaticTupleStruct(&5u32);
    let struct_brace = StaticBracedStruct { x: &8 };
    let enum_ = StaticEnum::Bar(&13u8);

    assert_eq!(
        trunc_fmt!(999;FmtArg::DEBUG; struct_tup),
        *format!("{:?}", struct_tup)
    );
    assert_eq!(
        trunc_fmt!(999;FmtArg::DEBUG; struct_brace),
        *format!("{:?}", struct_brace)
    );
    assert_eq!(
        trunc_fmt!(999;FmtArg::DEBUG; enum_),
        *format!("{:?}", enum_)
    );
}

#[derive(Debug)]
struct StaticTupleStruct<'a>(&'a u32)
where
    'a: 'static;

const_panic::impl_panicfmt! {
    struct StaticTupleStruct<'a> (&'a u32)
    where ['a: 'static];
}

#[derive(Debug)]
struct StaticBracedStruct<'a>
where
    'a: 'static,
{
    x: &'a u32,
}

const_panic::impl_panicfmt! {
    struct StaticBracedStruct<'a>
    where ['a: 'static]
    {
        x: &'a u32,
    }
}

#[derive(Debug)]
enum StaticEnum<'a>
where
    'a: 'static,
{
    #[allow(dead_code)]
    Foo,
    Bar(&'a u8),
}

const_panic::impl_panicfmt! {
    enum StaticEnum<'a>
    where ['a: 'static]
    {Foo, Bar(&u8)}
}

#[test]
fn generic_parsing() {
    const_panic::inline_macro! {
        (GenericParsing0<'_>),
        (GenericParsing1<'_>),
        (GenericParsing2<'_, ()>),
        (GenericParsing3<'_, ()>),
        (GenericParsing4<'_, (), 0>),
        (GenericParsing5<'_, (), 0>),
        (GenericParsing6<'_, (), 0>),
        (GenericParsingB0<'_>),
        (GenericParsingB1<'_>),
        (GenericParsingB2<'_, str>),
        (GenericParsingB3<'_, [u8]>),
        (GenericParsingB4<'_, u32, 0>),
        (GenericParsingB5<'_, u32, 0>),
        (GenericParsingB6<'_, u32, 0>);
        ($type_name:path) =>
        {
            let foo = $type_name(MyPhantomData::NEW);
            assert_eq!(trunc_fmt!(999;FmtArg::DEBUG; foo), *format!("{:?}", foo));
        }
    }
}

#[derive(Debug)]
struct GenericParsing0<'a>(MyPhantomData<(&'a (), ())>);

#[derive(Debug)]
struct GenericParsing1<'a>(MyPhantomData<(&'a (), ())>);

#[derive(Debug)]
struct GenericParsing2<'a, T>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsing3<'a, T>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsing4<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsing5<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsing6<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

const_panic::impl_panicfmt! {
    struct GenericParsing0<'a,>(MyPhantomData<(&'a (), ())>);
}

const_panic::impl_panicfmt! {
    struct GenericParsing1<'a,>(MyPhantomData<(&'a (), ())>);
}

const_panic::impl_panicfmt! {
    struct GenericParsing2<'a, ignore T>(MyPhantomData<(&'a (), T)>);
}

const_panic::impl_panicfmt! {
    struct GenericParsing3<'a, ignore T,>(MyPhantomData<(&'a (), T)>);
}

const_panic::impl_panicfmt! {
    struct GenericParsing4<'a, ignore T, const U: u32>(MyPhantomData<(&'a (), T)>);
}

const_panic::impl_panicfmt! {
    struct GenericParsing5<'a, ignore(MyPhantomData<u8>) T, ignore const U: u32,>(
        MyPhantomData<(&'a (), T)>
    );
}

const_panic::impl_panicfmt! {
    struct GenericParsing6<'a, ignore T, ignore(2) const U: u32,>(
        MyPhantomData<(&'a (), T)>
    );
}

#[derive(Debug)]
struct GenericParsingB0<'a>(MyPhantomData<(&'a (), ())>);

#[derive(Debug)]
struct GenericParsingB1<'a>(MyPhantomData<(&'a (), ())>);

#[derive(Debug)]
struct GenericParsingB2<'a, T: ?Sized>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsingB3<'a, T: ?Sized>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsingB4<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsingB5<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

#[derive(Debug)]
struct GenericParsingB6<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

const_panic::impl_panicfmt! {
    struct GenericParsingB0<'a>(MyPhantomData<(&'a (), ())>);

    (impl['a] GenericParsingB0<'a>)
}

const_panic::impl_panicfmt! {
    struct GenericParsingB1<'a>(MyPhantomData<(&'a (), ())>);

    (impl['a] GenericParsingB1<'a,> where[])
}

const_panic::impl_panicfmt! {
    struct GenericParsingB2<'a, ignore T>(MyPhantomData<(&'a (), T)>)
    where[T: ?Sized];

    (impl['a, T] GenericParsingB2<'a, T> where[T: ?Sized])
}

const_panic::impl_panicfmt! {
    struct GenericParsingB3<'a, ignore T,>(MyPhantomData<(&'a (), T)>)
    where[T: ?Sized];

    (impl['a, T: ?Sized] GenericParsingB3<'a, T,>)
}

const_panic::impl_panicfmt! {
    struct GenericParsingB4<'a, T, const U: u32>(MyPhantomData<(&'a (), T)>);

    (impl['a, const U: u32] GenericParsingB4<'a, u32, U,>)
}

const_panic::impl_panicfmt! {
    struct GenericParsingB5<'a, ignore(MyPhantomData<u8>) T, ignore const U: u32,>(
        MyPhantomData<(&'a (), T)>
    );

    (impl['a, const U: u32] GenericParsingB5<'a, u32, U>)
}

const_panic::impl_panicfmt! {
    struct GenericParsingB6<'a, ignore T, ignore(2) const U: u32,>(
        MyPhantomData<(&'a (), T)>
    );

    (impl['a] GenericParsingB6<'a, u32, 0>)
}

#[test]
fn where_clause_emision() {
    assert_eq!(
        trunc_fmt!(999;FmtArg::DEBUG; Unit0),
        *format!("{:?}", Unit0)
    );

    assert_eq!(HasConsts::A, 3);
    assert_eq!(HasConsts::B, 5);
}

struct HasConsts;

#[derive(Debug)]
struct Unit0;

const_panic::impl_panicfmt! {
    struct Unit0
    where[[(); {
        impl HasConsts {
            const A: u8 = 3;
        }

        0
    }]:];

    (
        impl Unit0
        where[[(); {
            impl HasConsts {
                const B: u8 = 5;
            }

            0
        }]:]
    )
}
