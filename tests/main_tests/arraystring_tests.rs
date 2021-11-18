use const_panic::{
    fmt::{ShortString, SHORT_STRING_CAP},
    ArrayString, PanicVal,
};

#[test]
fn concat_test() {
    for strings in [
        &[""][..],
        &["hello", ""],
        &["hello", "world"][..],
        &["", "world"][..],
    ] {
        let string = ArrayString::<1024>::concat(strings);
        assert_eq!(string, *strings.concat())
    }

    assert_eq!(ArrayString::<10>::concat(&["abcd", "efghij"]), "abcdefghij");
    std::panic::catch_unwind(|| ArrayString::<10>::concat(&["abcd", "efghij", "k"])).unwrap_err();
}

#[test]
fn concat_and_from_panicvals_test() {
    for (left, right, expected) in [
        ("helloworld", "", Some("helloworld")),
        ("", "helloworld", Some("helloworld")),
        ("hello", "world", Some("helloworld")),
        ("hello", "world!", None),
        ("helloworld", "!", None),
        ("helloworld!", "", None),
        ("", "helloworld!", None),
    ] {
        match expected {
            Some(exp) => {
                assert_eq!(ArrayString::<10>::concat(&[left, right]), exp);
            }
            None => {
                std::panic::catch_unwind(|| ArrayString::<10>::concat(&[left, right])).unwrap_err();

                let concat = &[left, right].concat();
                std::panic::catch_unwind(|| ArrayString::<10>::new(concat)).unwrap_err();
            }
        }

        assert_eq!(
            ArrayString::<10>::from_panicvals(&[left, right].map(PanicVal::write_str))
                .as_ref()
                .map(|s| s.to_str()),
            expected,
        );
        assert_eq!(
            ArrayString::<10>::concat_panicvals(&[
                &[PanicVal::write_str(left)],
                &[PanicVal::write_str(right)],
            ])
            .as_ref()
            .map(|s| s.to_str()),
            expected,
        );
    }
}

use rand::{rngs::SmallRng, Rng, SeedableRng};

fn strings_iter<'a>(cap: usize, rng: &'a mut SmallRng) -> impl Iterator<Item = String> + 'a {
    const CHARS: &[char] = &['ñ', 'ö', 'f', 'o', '个', '人'];

    std::iter::repeat_with(move || {
        let mut len = 0;
        let used_cap = rng.gen_range(0..=cap);
        let out = std::iter::repeat_with(|| CHARS[rng.gen_range(0..CHARS.len())])
            .take_while(move |c| {
                len += c.len_utf8();
                len <= used_cap
            })
            .collect::<String>();
        assert!(used_cap <= cap);
        assert!(out.len() <= used_cap);
        out
    })
}

#[test]
fn fmt_arraystring_test() {
    let mut rng = SmallRng::from_seed(6249204433781597762u128.to_ne_bytes());

    fn as_test_case<const LEN: usize>(rng: &mut SmallRng) {
        let strings = strings_iter(LEN, rng);

        for string in strings.take(100) {
            let string = &*string;
            let string_debug = &*format!("{:?}", string);

            assert_eq!(
                trunc_fmt!(200; ArrayString::<LEN>::new(string)),
                string_debug
            );
            assert_eq!(
                trunc_fmt!(200; debug: ArrayString::<LEN>::new(string)),
                string_debug
            );
            assert_eq!(
                trunc_fmt!(200; display: ArrayString::<LEN>::new(string)),
                string
            );
        }
    }

    macro_rules! test_lengths {
        ($($len:tt)*) => (
            $(as_test_case::<$len>(&mut rng);)*
        )
    }

    test_lengths! {
        0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
        17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
    }

    for string in strings_iter(SHORT_STRING_CAP, &mut rng).take(100) {
        let string = &*string;
        let short = PanicVal::write_short_str(ShortString::new(string));
        assert_eq!(trunc_fmt!(200; short), string);
        assert_eq!(trunc_fmt!(200; debug: short), string);
        assert_eq!(trunc_fmt!(200; display: short), string);
    }
}
