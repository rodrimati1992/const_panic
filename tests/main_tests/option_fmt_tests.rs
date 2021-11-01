use const_panic::FmtArg;

use core::{
    cmp::Ordering,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
    ptr::NonNull,
};

#[test]
fn test_option_fmt() {
    macro_rules! test_case {
        ($expr:expr, $fmt:expr, $expected:expr) => {
            assert_eq!(trunc_fmt!(1024; $fmt; $expr), $expected);
        };
    }

    test_case! {Some("hello"), FmtArg::DEBUG, "Some(\"hello\")"}
    test_case! {Some(false), FmtArg::DEBUG, "Some(false)"}
    test_case! {Some(3u8), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3u16), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3u32), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3u64), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3u128), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3usize), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3i8), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3i16), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3i32), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3i64), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3i128), FmtArg::DEBUG, "Some(3)"}
    test_case! {Some(3isize), FmtArg::DEBUG, "Some(3)"}

    test_case! {NonNull::new(&mut 100), FmtArg::DEBUG, "Some(<pointer>)"}

    test_case! {NonZeroU8::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroI8::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroU16::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroI16::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroU32::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroI32::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroU64::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroI64::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroU128::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroI128::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroUsize::new(5), FmtArg::DEBUG, "Some(5)"}
    test_case! {NonZeroIsize::new(5), FmtArg::DEBUG, "Some(5)"}

    test_case! {Some(Ordering::Less), FmtArg::DEBUG, "Some(Less)"}
    test_case! {Some(Ordering::Equal), FmtArg::DEBUG, "Some(Equal)"}
    test_case! {Some(Ordering::Greater), FmtArg::DEBUG, "Some(Greater)"}

    test_case! {Some([3u8, 5, 8]), FmtArg::DEBUG, "Some([3, 5, 8])"}
    test_case! {Some(&[3u16, 5, 8]), FmtArg::DEBUG, "Some([3, 5, 8])"}
    test_case! {Some(&[3u32, 5, 8][..]), FmtArg::DEBUG, "Some([3, 5, 8])"}

    test_case! {Some([false, true]), FmtArg::DEBUG, "Some([false, true])"}
    test_case! {
        Some( [false, true]),
        FmtArg::ALT_DEBUG,
        concat!(
            "Some(\n",
            "    [\n",
            "        false,\n",
            "        true,\n",
            "    ],\n",
            ")",
        )
    }

    test_case! {Some(["hello", "world"]), FmtArg::DEBUG, "Some([\"hello\", \"world\"])"}
    test_case! {Some(["hello", "world"]), FmtArg::DISPLAY, "Some([hello, world])"}
    test_case! {
        Some( ["hello", "world"]),
        FmtArg::ALT_DEBUG,
        concat!(
            "Some(\n",
            "    [\n",
            "        \"hello\",\n",
            "        \"world\",\n",
            "    ],\n",
            ")",
        )
    }
    test_case! {
        Some( ["hello", "world"]),
        FmtArg::ALT_DISPLAY,
        concat!(
            "Some(\n",
            "    [\n",
            "        hello,\n",
            "        world,\n",
            "    ],\n",
            ")",
        )
    }
}
