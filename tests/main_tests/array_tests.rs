#[test]
fn overflow_truncating_integer_array_test() {
    for (slice, expected) in [
        (&[][..], "[]"),
        (&[3u8], "[3]"),
        (&[3u8, 5], "[3, 5]"),
        (&[3u8, 5, 8], "[3, 5, 8]"),
        (&[3u8, 5, 8, 13], "[3, 5, 8, 13]"),
        (&[3u8, 5, 8, 13, 21], "[3, 5, 8, 13, 21]"),
    ] {
        assert_eq!(dbg!(overf_fmt!(20; slice).unwrap()), expected);
        assert_eq!(trunc_fmt!(20; slice), expected);
    }

    // numbers shouldn't be truncated, they should just not be printed
    let upto34 = [3i8, 5, 8, -3, 21, 34];
    for (len, expected) in [
        (0, ""),
        (1, "["),
        (2, "[3"),
        (3, "[3,"),
        (4, "[3, "),
        (5, "[3, 5"),
        (6, "[3, 5,"),
        (12, "[3, 5, 8, -3"),
        (13, "[3, 5, 8, -3,"),
        (14, "[3, 5, 8, -3, "),
        (15, "[3, 5, 8, -3, "),
        (15, "[3, 5, 8, -3, "),
        (16, "[3, 5, 8, -3, 21"),
        (17, "[3, 5, 8, -3, 21,"),
        (18, "[3, 5, 8, -3, 21, "),
        (19, "[3, 5, 8, -3, 21, "),
        (20, "[3, 5, 8, -3, 21, 34"),
    ] {
        assert_eq!(trunc_fmt!(len; upto34), expected);
    }

    overf_fmt!(19; upto34).unwrap_err();
    overf_fmt!(20; upto34).unwrap_err();
}

#[test]
fn string_test() {
    assert_eq!(trunc_fmt!(0; ["h\nllo", "人ö个"]), r#""#);
    assert_eq!(trunc_fmt!(1; ["h\nllo", "人ö个"]), r#"["#);
    assert_eq!(trunc_fmt!(2; ["h\nllo", "人ö个"]), r#"[""#);
    assert_eq!(trunc_fmt!(3; ["h\nllo", "人ö个"]), r#"["h"#);
    assert_eq!(trunc_fmt!(4; ["h\nllo", "人ö个"]), r#"["h"#);
    assert_eq!(trunc_fmt!(5; ["h\nllo", "人ö个"]), r#"["h\n"#);
    assert_eq!(trunc_fmt!(6; ["h\nllo", "人ö个"]), r#"["h\nl"#);
    assert_eq!(trunc_fmt!(7; ["h\nllo", "人ö个"]), r#"["h\nll"#);
    assert_eq!(trunc_fmt!(8; ["h\nllo", "人ö个"]), r#"["h\nllo"#);
    assert_eq!(trunc_fmt!(9; ["h\nllo", "人ö个"]), r#"["h\nllo""#);
    assert_eq!(trunc_fmt!(10; ["h\nllo", "人ö个"]), r#"["h\nllo","#);
    assert_eq!(trunc_fmt!(11; ["h\nllo", "人ö个"]), r#"["h\nllo", "#);
    assert_eq!(trunc_fmt!(12; ["h\nllo", "人ö个"]), r#"["h\nllo", ""#);
    assert_eq!(trunc_fmt!(13; ["h\nllo", "人ö个"]), r#"["h\nllo", ""#);
    assert_eq!(trunc_fmt!(14; ["h\nllo", "人ö个"]), r#"["h\nllo", ""#);
    assert_eq!(trunc_fmt!(15; ["h\nllo", "人ö个"]), r#"["h\nllo", "人"#);
    assert_eq!(trunc_fmt!(16; ["h\nllo", "人ö个"]), r#"["h\nllo", "人"#);
    assert_eq!(trunc_fmt!(17; ["h\nllo", "人ö个"]), r#"["h\nllo", "人ö"#);
    assert_eq!(trunc_fmt!(18; ["h\nllo", "人ö个"]), r#"["h\nllo", "人ö"#);
    assert_eq!(trunc_fmt!(19; ["h\nllo", "人ö个"]), r#"["h\nllo", "人ö"#);
    assert_eq!(trunc_fmt!(20; ["h\nllo", "人ö个"]), r#"["h\nllo", "人ö个"#);
    assert_eq!(trunc_fmt!(21; ["h\nllo", "人ö个"]), r#"["h\nllo", "人ö个""#);
    assert_eq!(
        trunc_fmt!(22; ["h\nllo", "人ö个"]),
        r#"["h\nllo", "人ö个"]"#
    );
}

#[test]
fn bin_integer_test() {
    let array = [-4, -3, -2, -1, 0i8, 1, 2, 3, 4];
    assert_eq!(
        overf_fmt!(1024; bin: array).unwrap(),
        "[11111100, 11111101, 11111110, 11111111, 0, 1, 10, 11, 100]"
    );

    assert_eq!(
        overf_fmt!(1024; alt_bin: array).unwrap(),
        concat!(
            "[\n",
            "    0b11111100,\n",
            "    0b11111101,\n",
            "    0b11111110,\n",
            "    0b11111111,\n",
            "    0b0,\n",
            "    0b1,\n",
            "    0b10,\n",
            "    0b11,\n",
            "    0b100,\n",
            "]",
        )
    );
}
#[test]
fn integer_test() {
    macro_rules! test_case {
        ($array:expr) => ({
            let array = $array;

            assert_eq!(
                overf_fmt!(1024; array).unwrap(),
                *format!("{:?}", array),
            );

            assert_eq!(
                overf_fmt!(1024; debug: array).unwrap(),
                *format!("{:?}", array),
            );

            assert_eq!(
                overf_fmt!(1024; alt_debug: array).unwrap(),
                *format!("{:#?}", array),
            );

            assert_eq!(
                overf_fmt!(1024; hex: array).unwrap(),
                *format!("{:X?}", array),
            );

            assert_eq!(
                overf_fmt!(1024; alt_hex: array).unwrap(),
                *format!("{:#X?}", array),
            );
        })
    }

    test_case!([0u16, 1, 2, u16::MAX / 2, u16::MAX]);
    test_case!([0u32, 1, 2, u32::MAX / 2, u32::MAX]);
    test_case!([0u64, 1, 2, u64::MAX / 2, u64::MAX]);
    test_case!([0u128, 1, 2, u128::MAX / 2, u128::MAX]);
    test_case!([0usize, 1, 2, usize::MAX / 2, usize::MAX]);

    test_case!([
        i16::MIN,
        i16::MIN / 2,
        -2,
        -1,
        0i16,
        1,
        2,
        i16::MAX / 2,
        i16::MAX
    ]);
    test_case!([
        i32::MIN,
        i32::MIN / 2,
        -2,
        -1,
        0i32,
        1,
        2,
        i32::MAX / 2,
        i32::MAX
    ]);
    test_case!([
        i64::MIN,
        i64::MIN / 2,
        -2,
        -1,
        0i64,
        1,
        2,
        i64::MAX / 2,
        i64::MAX
    ]);
    test_case!([
        i128::MIN,
        i128::MIN / 2,
        -2,
        -1,
        0i128,
        1,
        2,
        i128::MAX / 2,
        i128::MAX
    ]);
    test_case!([
        isize::MIN,
        isize::MIN / 2,
        -2,
        -1,
        0isize,
        1,
        2,
        isize::MAX / 2,
        isize::MAX
    ]);
}

#[test]
fn test_bool_arrays() {
    let empty_bool: [bool; 0] = [];

    assert_eq!(trunc_fmt!(10; empty_bool), "[]");
    assert_eq!(trunc_fmt!(10; [false]), "[false]");
    assert_eq!(trunc_fmt!(10; [true]), "[true]");
    assert_eq!(trunc_fmt!(10; [true, true]), "[true, tru");
    assert_eq!(trunc_fmt!(11; [true, true]), "[true, true");
    assert_eq!(trunc_fmt!(12; [true, true]), "[true, true]");
}
