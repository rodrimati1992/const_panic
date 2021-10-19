use const_panic::fmt::IsDisplay;

const MAX_L: usize = 1024;

#[test]
fn escaped_string() {
    for (string, debug_escaped) in [
        (ALL_ASCII, ALL_ASCII_ESCAPED),
        ("\\", r#""\\""#),
        (r#"\u\u{}"#, r#""\\u\\u{}""#),
    ] {
        assert_eq!(trunc_fmt!(MAX_L; string), debug_escaped);
        assert_eq!(trunc_fmt!(MAX_L; string), debug_escaped);
        assert_eq!(trunc_fmt!(MAX_L; string), debug_escaped);

        assert_eq!(trunc_fmt!(MAX_L; {}: string), string);
        assert_eq!(trunc_fmt!(MAX_L; {}: string), string);
        assert_eq!(trunc_fmt!(MAX_L; {}: string), string);

        assert_eq!(trunc_fmt!(MAX_L; display: string), string);
        assert_eq!(trunc_fmt!(MAX_L; display: string), string);
        assert_eq!(trunc_fmt!(MAX_L; display: string), string);
    }

    macro_rules! literal_strings {
        ($($lit:literal,)*) => (
            $(assert_eq!(trunc_fmt!(MAX_L; $lit), $lit);)*
        )
    }
    literal_strings! {
        "\
         \x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\
         \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \
         !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
         ^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\u{80}\u{81}\u{90}\u{91}\
        ",
        "\\",
        r#"\u\u{}"#,
    }
}

// copied from const_format
pub const ALL_ASCII: &str = "\
 \x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\
 \x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f \
 !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]\
 ^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\u{80}\u{81}\u{90}\u{91}\
";

// copied from const_format
pub const ALL_ASCII_ESCAPED: &str = "\
 \"\
 \\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\t\\n\\x0B\\x0C\\r\\x0E\\x0F\
 \\x10\\x11\\x12\\x13\\x14\\x15\\x16\\x17\\x18\\x19\\x1A\\x1B\\x1C\\x1D\\x1E\\x1F \
 !\\\"#$%&\\\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\\\]\
 ^_`abcdefghijklmnopqrstuvwxyz{|}~\x7f\u{80}\u{81}\u{90}\u{91}\
 \"\
";

pub const DEBUG_AND_DISPLAY: &[(&str, &str)] = &[
    (r#"\x00"#, "\x00"),
    (r#"\n"#, "\n"),
    (r#"\x01"#, "\x01"),
    (r#"\r"#, "\r"),
    (r#"ñ"#, "ñ"),
    (r#"ö"#, "ö"),
    (r#"f"#, "f"),
    (r#"o"#, "o"),
    (r#"个"#, "个"),
    (r#"a"#, "a"),
    (r#"人"#, "人"),
];

#[test]
fn test_trunc_overflow() {
    let mut debug_and_display = DEBUG_AND_DISPLAY.to_vec();

    for _rotation in 0..debug_and_display.len() {
        for up_to in 0..debug_and_display.len() {
            let (debug_vec, display_vec): (Vec<&str>, Vec<&str>) =
                debug_and_display[..up_to].iter().copied().unzip();

            let display_str = &*display_vec.iter().copied().collect::<String>();
            let display_len = display_str.len();
            let mut display_substr = String::new();

            for i in 0..display_str.len() {
                fill_buffer(&mut display_substr, &display_vec, i, IsDisplay::Yes);
                assert_eq!(trunc_fmt!(i; display: display_str), *display_substr);
                overf_fmt!(i; display: display_str).unwrap_err();
            }

            {
                assert_eq!(trunc_fmt!(display_len; display: display_str), *display_str);
                assert_eq!(
                    overf_fmt!(display_len; display: display_str).unwrap(),
                    *display_str
                );
            }

            let debug_str = &*std::iter::once("\"")
                .chain(debug_vec.iter().copied())
                .chain(std::iter::once("\""))
                .collect::<String>();
            let debug_len = debug_str.len();
            let mut debug_substr = String::new();

            for i in 0..debug_len {
                fill_buffer(&mut debug_substr, &debug_vec, i, IsDisplay::No);
                assert_eq!(trunc_fmt!(i; debug: display_str), *debug_substr, "i:{}", i);
                overf_fmt!(i; debug: display_str).unwrap_err();
            }

            {
                assert_eq!(trunc_fmt!(debug_len; debug: display_str), debug_str);
                assert_eq!(
                    overf_fmt!(debug_len; debug: display_str).unwrap(),
                    debug_str
                );
            }
        }

        debug_and_display.rotate_right(1);
    }
}

#[test]
fn test_fill_buffer() {
    let mut buffer = String::new();

    for (len, expected) in [
        (11, "helloworld"),
        (10, "helloworld"),
        (9, "hello"),
        (8, "hello"),
        (5, "hello"),
        (4, ""),
        (3, ""),
        (2, ""),
        (1, ""),
        (0, ""),
    ] {
        fill_buffer(&mut buffer, &["hello", "world"], len, IsDisplay::Yes);
        assert_eq!(buffer, expected);
    }

    for (len, expected) in [
        (12, r#""helloworld""#),
        (11, r#""helloworld"#),
        (10, r#""hello"#),
        (9, r#""hello"#),
        (8, r#""hello"#),
        (6, r#""hello"#),
        (5, r#"""#),
        (4, r#"""#),
        (3, r#"""#),
        (2, r#"""#),
        (1, r#"""#),
        (0, r#""#),
    ] {
        fill_buffer(&mut buffer, &["hello", "world"], len, IsDisplay::No);
        assert_eq!(buffer, expected);
    }
}

fn fill_buffer(buffer: &mut String, chars: &[&str], max_len: usize, is_display: IsDisplay) {
    buffer.clear();
    if max_len == 0 {
        return;
    }
    if let IsDisplay::No = is_display {
        push_if_fits(buffer, "\"", max_len);
    }
    for char in chars.iter().copied() {
        if let DidFit::No = push_if_fits(buffer, char, max_len) {
            return;
        }
    }
    if let IsDisplay::No = is_display {
        push_if_fits(buffer, "\"", max_len);
    }
}

enum DidFit {
    Yes,
    No,
}

fn push_if_fits(buffer: &mut String, char: &str, max_len: usize) -> DidFit {
    if buffer.len() + char.len() <= max_len {
        buffer.push_str(char);
        DidFit::Yes
    } else {
        DidFit::No
    }
}
