use crate::test_utils::StrExt;

use alloc::string::{String, ToString};

fn process_str(s: &str) -> Result<String, String> {
    syn::parse_str(s)
        .and_then(crate::derive_debug::derive_constdebug_impl)
        .map(|x| x.to_string())
        .map_err(|e| e.to_compile_error().to_string())
}

#[test]
fn ignored_generic_arguments() {
    {
        let s = process_str(
            r#"
            #[pfmt(ignore(A, B))]
            pub struct IgnoredGenericParams<'a, A, B, const X: u32, const Y: char> (
                pub &'a u32,
                pub PhantomData<A>,
                pub PhantomData<B>,
            );
        "#,
        )
        .unwrap();
        assert!(
            s.consecutive_unspace(&[r#"
                    ;
                        <
                        IgnoredGenericParams<
                            '_,
                            (),
                            (),
                            {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT},
                            {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT}
                        >
                        as __cp_bCj7dq3Pud::PanicFmt
                        >::PV_COUNT
                    ]
                "#]),
            "\n{}\n",
            s,
        );
    }

    {
        let s = process_str(
            r#"
            #[pfmt(ignore(A, B = u64, X = 100, Y = '_'))]
            pub struct IgnoredGenericParams<'a, A, B, const X: u32, const Y: char> (
                pub &'a u32,
                pub PhantomData<A>,
                pub PhantomData<B>,
            );
        "#,
        )
        .unwrap();
        assert!(
            s.consecutive_unspace(&[r#"
                    ;
                    <
                        IgnoredGenericParams<
                            '_,
                            (),
                            u64,
                            100,
                            '_'
                        >
                        as __cp_bCj7dq3Pud::PanicFmt
                        >::PV_COUNT
                    ]
                "#]),
            "\n{}\n",
            s,
        );
    }
}

#[test]
fn ignored_generic_arguments_and_impl() {
    let s = process_str(
        r#"
        #[pfmt(ignore(A))]
        #[pfmt(impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u8, H, I>)]
        #[pfmt(impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u16, H, I>)]
        #[pfmt(impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u32, H, I>)]
        pub struct IgnoredAndImpl<'a, A, B, const X: u32, const Y: char> (
            pub &'a u32,
            pub PhantomData<A>,
            pub PhantomData<B>,
        );
    "#,
    )
    .unwrap();
    assert!(
        s.consecutive_unspace(&[
            "impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u8, H, I>",
            "IgnoredAndImpl<
                '_,
                (),
                u8,
                {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT},
                {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT}
            >",
        ]),
        "\n{}\n",
        s,
    );

    assert!(
        s.consecutive_unspace(&[
            "impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u16, H, I>",
            "IgnoredAndImpl<
                '_,
                (),
                u16,
                {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT},
                {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT}
            >",
        ]),
        "\n{}\n",
        s,
    );

    assert!(
        s.consecutive_unspace(&[
            "impl<'b, G, const H: u32, const I: char> IgnoredAndImpl<'b, G, u32, H, I>",
            "IgnoredAndImpl<
                '_,
                (),
                u32,
                {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT},
                {__cp_bCj7dq3Pud::__::ConstDefault::DEFAULT}
            >",
        ]),
        "\n{}\n",
        s,
    );
}

#[test]
fn generic_type_error() {
    for case in [
        r#"
            pub struct IgnoredAndImpl<'a, A, B, const X: u32, const Y: char> (
                pub &'a u32,
                pub PhantomData<A>,
                pub PhantomData<B>,
            );
        "#,
        r#"
            pub struct IgnoredAndImpl<'a, A, B> (
                pub &'a u32,
                pub PhantomData<A>,
                pub PhantomData<B>,
            );
        "#,
    ] {
        let err = process_str(case).unwrap_err();

        assert!(
            err.consecutive_unspace(&["`#[pfmt(ignore(", "`#[pfmt(impl",]),
            "\n{}\n",
            err,
        );
    }
}

#[test]
fn impl_attribute_typename_error() {
    let input = r#"
        #[pfmt(impl<G> Baaa<G>)]
        pub struct Fooo<'a, A> (
            pub PhantomData<B>,
        );
    "#;

    let err = process_str(input).unwrap_err();

    assert!(err.consecutive_unspace(&["expected `Fooo`"]), "\n{}\n", err,);
}
