#[test]
fn integer_test() {
    macro_rules! test_case {
        ($($nums:expr),* $(,)*)=> ({
            for int in [$($nums),*] {
                let string = format!("{:?}", int);

                assert_eq!(
                    overf_fmt!(string.len(); int).unwrap().as_str(),
                    string,
                );
                assert_eq!(trunc_fmt!(string.len(); int).as_str(), string);

                overf_fmt!(string.len() - 1; int).unwrap_err();
                assert_eq!(trunc_fmt!(string.len() - 1; int).as_str(), "");
            }
        })
    }

    test_case!(0u8, 1, 2, u8::MAX / 2, u8::MAX);
    test_case!(0u16, 1, 2, u16::MAX / 2, u16::MAX);
    test_case!(0u32, 1, 2, u32::MAX / 2, u32::MAX);
    test_case!(0u64, 1, 2, u64::MAX / 2, u64::MAX);
    test_case!(0u128, 1, 2, u128::MAX / 2, u128::MAX);
    test_case!(0usize, 1, 2, usize::MAX / 2, usize::MAX);

    test_case!(
        i8::MIN,
        i8::MIN / 2,
        -2,
        -1,
        0i8,
        1,
        2,
        i8::MAX / 2,
        i8::MAX
    );
    test_case!(
        i16::MIN,
        i16::MIN / 2,
        -2,
        -1,
        0i16,
        1,
        2,
        i16::MAX / 2,
        i16::MAX
    );
    test_case!(
        i32::MIN,
        i32::MIN / 2,
        -2,
        -1,
        0i32,
        1,
        2,
        i32::MAX / 2,
        i32::MAX
    );
    test_case!(
        i64::MIN,
        i64::MIN / 2,
        -2,
        -1,
        0i64,
        1,
        2,
        i64::MAX / 2,
        i64::MAX
    );
    test_case!(
        i128::MIN,
        i128::MIN / 2,
        -2,
        -1,
        0i128,
        1,
        2,
        i128::MAX / 2,
        i128::MAX
    );
    test_case!(
        isize::MIN,
        isize::MIN / 2,
        -2,
        -1,
        0isize,
        1,
        2,
        isize::MAX / 2,
        isize::MAX
    );
}
