use const_panic::FmtArg;

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

    // making sure that the `lifetime = 'static;` argument to `impl_panicfmt` has an effect.
    let _: [const_panic::PanicVal<'static>; 1] = foo.w.to_panicvals(FmtArg::DEBUG);

    let _: [const_panic::PanicVal<'static>; <Qux<u8> as const_panic::PanicFmt>::PV_COUNT] =
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
    impl Foo<'_>;

    struct Foo {
        x: &[u8],
        y: u8,
        z: Bar,
        w: Baz,
        a: Qux<u8>,
        b: Qux<u16>,
        c: Qux<u32>,
    }
}

#[derive(Debug)]
struct Bar(bool, bool);

const_panic::impl_panicfmt! {
    impl Bar;
    struct Bar(bool, bool);
}

#[derive(Debug)]
struct Baz;

const_panic::impl_panicfmt! {
    impl Baz;

    lifetime = 'static;

    struct Baz
}

#[derive(Debug)]
enum Qux<T> {
    Up,
    Down { x: T, y: T },
    Left(u64),
}

const_panic::inline_macro! {
    (u8, ()),
    (u16, (lifetime = 'static;)),
    (u32, ());

    ($T:ty , ($($other:tt)*)) =>
        const_panic::impl_panicfmt!{
            impl Qux<$T>;

            $($other)*

            enum Qux {
                Up,
                Down{
                    x: $T,
                    y: $T,
                },
                Left(u64),
            }
        }
}

#[test]
fn to_panicvals_lifetime_test() {
    use const_panic::PanicVal;

    let u32_ = 5;
    let u8_ = 3;

    let _: &[PanicVal<'static>] = &StaticStruct(8).to_panicvals(FmtArg::DEBUG);

    let _: &[PanicVal<'static>] = &StaticEnum::Foo.to_panicvals(FmtArg::DEBUG);

    let _: &[PanicVal<'_>] = &NonStaticStruct(&u32_).to_panicvals(FmtArg::DEBUG);

    let _: &[PanicVal<'_>] = &NonStaticEnum::Bar(&u8_).to_panicvals(FmtArg::DEBUG);
}

struct StaticStruct(u32);

const_panic::impl_panicfmt! {
    impl StaticStruct;

    lifetime = 'static;

    struct StaticStruct(u32);
}

enum StaticEnum {
    Foo,
    #[allow(dead_code)]
    Bar,
}

const_panic::impl_panicfmt! {
    impl StaticEnum;

    lifetime = 'static;

    enum StaticEnum {Foo, Bar}
}

struct NonStaticStruct<'a>(&'a u32);

const_panic::impl_panicfmt! {
    impl NonStaticStruct<'_>;

    lifetime = '_;

    struct NonStaticStruct(&u32);
}

enum NonStaticEnum<'a> {
    #[allow(dead_code)]
    Foo,
    Bar(&'a u8),
}

const_panic::impl_panicfmt! {
    impl NonStaticEnum<'_>;

    lifetime = '_;

    enum NonStaticEnum {Foo, Bar(&u8)}
}
