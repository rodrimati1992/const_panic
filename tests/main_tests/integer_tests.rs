use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use const_panic::FmtArg;

fn tn_of_val<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

macro_rules! test_case {
    ($num:expr)=> (
        let int = $num;

        for (fmt, string) in [
            (FmtArg::DEBUG, format!("{:?}", int)),
            (FmtArg::ALT_DEBUG, format!("{:?}", int)),
            (FmtArg::DISPLAY, format!("{:?}", int)),
            (FmtArg::ALT_DISPLAY, format!("{:?}", int)),
            (FmtArg::HEX, format!("{:X}", int)),
            (FmtArg::ALT_HEX, format!("{:#X}", int)),
            (FmtArg::BIN, format!("{:b}", int)),
            (FmtArg::ALT_BIN, format!("{:#b}", int)),
        ] {
            let msg = || format!(
                "string.len(): {} num: {:?} fmt_override: {:?} type: {}",
                string.len(), int, fmt, tn_of_val(int),
            );

            assert_eq!(
                overf_fmt!(string.len(); fmt; int).unwrap(),
                *string,
                "{}",
                msg(),
            );
            assert_eq!(trunc_fmt!(string.len(); fmt; int), *string, "{}", msg());

            overf_fmt!(string.len() - 1; fmt; int).unwrap_err();
            assert_eq!(
                trunc_fmt!(string.len() - 1; fmt; int),
                "",
                "{}", msg()
            );
        }
    )
}

macro_rules! int_test {
    ($ty:ty) => {{
        let zero: $ty = 0;
        let iter = (zero..10)
            .chain(
                std::iter::successors(Some::<$ty>(1), |n| n.checked_mul(10))
                    .chain(std::iter::successors(Some::<$ty>(1), |n| n.checked_mul(2)))
                    .flat_map(|x| [x - 1, x, x + 1]),
            )
            .chain((0..3).flat_map(|x| [<$ty>::MIN + x, <$ty>::MAX - x]))
            .flat_map(|x| [zero.saturating_sub(x), x]);

        println!("{}:", stringify!($ty));
        for int in iter {
            print!("{:?} ", int);
            test_case! {int}
        }
        println!();
    }};
}

#[test]
fn integer_test() {
    int_test! {u8}
    int_test! {u16}
    int_test! {u32}
    int_test! {u64}
    int_test! {u128}
    int_test! {usize}
    int_test! {i8}
    int_test! {i16}
    int_test! {i32}
    int_test! {i64}
    int_test! {i128}
    int_test! {isize}
}

// Tests aren't so thorough, since NonZero integers just delegate to the built-in ones.
#[test]
#[cfg(feature = "non_basic")]
fn nonzero_integer_test() {
    test_case! {NonZeroU8::new(5).unwrap()}
    test_case! {NonZeroI8::new(-5).unwrap()}
    test_case! {NonZeroU16::new(8).unwrap()}
    test_case! {NonZeroI16::new(-8).unwrap()}
    test_case! {NonZeroU32::new(13).unwrap()}
    test_case! {NonZeroI32::new(-13).unwrap()}
    test_case! {NonZeroU64::new(21).unwrap()}
    test_case! {NonZeroI64::new(-21).unwrap()}
    test_case! {NonZeroU128::new(34).unwrap()}
    test_case! {NonZeroI128::new(-34).unwrap()}
    test_case! {NonZeroUsize::new(55).unwrap()}
    test_case! {NonZeroIsize::new(-55).unwrap()}
}
