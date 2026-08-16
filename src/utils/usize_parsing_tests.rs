use super::parse_usize;

use core::fmt::Write;


type UsizeFmtBuffer = arrayvec::ArrayString<{(usize::BITS as usize) / 2}>;

fn format_usize(n: usize) -> UsizeFmtBuffer {
    let mut s = UsizeFmtBuffer::new();
    write!(s, "{n}").unwrap();
    s
}



#[test]
fn from_literals_ok_parsing_test() {
    for lit in [
        "0",
        "1",
        "6",
        "9",
        "09",
        "000012",
        "10",
        "16",
        "99",
        "100",
        "101",
        "10003",
        "12345",
        "54321",
    ] {
        let parsed = parse_usize(lit);
        assert!(parsed.is_some(), "lit = {lit:?}  parsed = {parsed:?}");

        assert_eq!(parsed, usize::from_str_radix(lit, 10).ok(), "lit = {lit:?}");
    }
}

#[test]
fn err_parsing_test() {
    let mut too_large = format_usize(usize::MAX);
    too_large.push('0');

    for lit in [
        "",
        "_",
        " ",
        "1A4",
        "0x",
        "0x9",
        "_100003",
        "100_003",
        "100003_",
        &too_large,
    ] {
        let parsed = parse_usize(lit);
        assert!(parsed.is_none(), "lit = {lit:?}  parsed = {parsed:?}");

        assert_eq!(parsed, usize::from_str_radix(lit, 10).ok(), "lit = {lit:?}");
    }
}


#[test]
fn first_and_last_integers_test() {
    let mid = isize::MAX as usize;

    for n in 
        (0..=200)
            .chain((mid - 2) ..= (mid + 2))
            .chain((usize::MAX - 2) ..= usize::MAX)
    {
        let s = format_usize(n);
        assert_eq!(parse_usize(&s), usize::from_str_radix(&s, 10).ok(), "s = {s:?}");
    }
}




