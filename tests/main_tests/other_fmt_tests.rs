use const_panic::{FmtArg, StdWrapper};

use core::{
    cmp::Ordering,
    marker::{PhantomData, PhantomPinned},
    ptr::NonNull,
    sync::atomic::Ordering as AtomicOrdering,
};

macro_rules! test_val {
    ($value:expr, $expected:expr) => ({
        let val = $value;
        assert_eq!(trunc_fmt!(1024; StdWrapper(&val)), $expected);
        assert_eq!(trunc_fmt!(1024; StdWrapper(&val).to_panicvals(FmtArg::DEBUG)), $expected);
        assert_eq!(trunc_fmt!(1024; StdWrapper(&val).to_panicval(FmtArg::DEBUG)), $expected);
    })
}

#[test]
fn fmt_pointer() {
    test_val! {&3u8 as *const u8, "<pointer>"}
    test_val! {&mut 3u8 as *mut u8, "<pointer>"}
    test_val! {NonNull::<u8>::from(&mut 3), "<pointer>"}

    test_val! {"hello" as *const str, "<pointer>"}

    test_val! {&[][..] as *const [u8], "<pointer>"}
    test_val! {&mut [][..] as *mut [u8], "<pointer>"}
    test_val! {NonNull::<[u8]>::from(&mut [][..]), "<pointer>"}
}

#[test]
fn fmt_units() {
    test_val! {PhantomData::<u8>, "PhantomData"}
    test_val! {PhantomData::<str>, "PhantomData"}

    test_val! {PhantomPinned, "PhantomPinned"}

    test_val! {(), "()"}
}

#[test]
fn fmt_orderings() {
    test_val! {Ordering::Less, "Less"}
    test_val! {Ordering::Equal, "Equal"}
    test_val! {Ordering::Greater, "Greater"}

    test_val! {AtomicOrdering::Relaxed, "Relaxed"}
    test_val! {AtomicOrdering::Release, "Release"}
    test_val! {AtomicOrdering::Acquire, "Acquire"}
    test_val! {AtomicOrdering::AcqRel, "AcqRel"}
    test_val! {AtomicOrdering::SeqCst, "SeqCst"}
}
