use const_panic::concat_assert;

#[test]
fn test_concat_assert() {
    let zero = 0;
    concat_assert!(zero == 0);
    std::panic::catch_unwind(|| concat_assert!(zero == 1)).unwrap_err();

    concat_assert!(zero == 0, "hello", 100u8);
    std::panic::catch_unwind(|| concat_assert!(zero == 1, "hello", 100u8)).unwrap_err();
}
