use const_panic::MAX_PANIC_MSG_LEN;

#[test]
fn max_length_env_var_test() {
    #[track_caller]
    fn inner(msg: &str, truncated_expected: &str, truncate_to: usize) {
        assert_eq!(truncate_to, MAX_PANIC_MSG_LEN);

        let err = std::panic::catch_unwind(|| {
            const_panic::concat_panic(&[&[const_panic::PanicVal::write_str(msg)]])
        })
        .unwrap_err();

        let truncated_found: &str = if let Some(x) = err.downcast_ref::<&str>() {
            x
        } else if let Some(x) = err.downcast_ref::<String>() {
            x
        } else {
            panic!("could not downcast into either &str or String")
        };

        assert_eq!(truncated_expected, truncated_found);
    }

    assert_eq!('風'.len_utf8(), 3);

    match dbg!(option_env!("CONST_PANIC_MAX_LENGTH")) {
        Some("" | "_") | None => {
            #[allow(clippy::assertions_on_constants)]
            {
                assert!(MAX_PANIC_MSG_LEN >= 16, "{MAX_PANIC_MSG_LEN}");
            }

            inner("Hello, World!", "Hello, World!", MAX_PANIC_MSG_LEN);
        }
        Some("0") => {
            inner("foo", "", 0);
        }
        Some("3") => {
            inner("foobar", "foo", 3);
            inner("fo風", "fo", 3);
            inner("f風", "f", 3);
            inner("風", "風", 3);
        }
        Some("10") => {
            inner("Hello", "Hello", 10);
            inner("Hello, world!", "Hello, wor", 10);
            inner("Hello, w風", "Hello, w", 10);
        }
        Some("80000") => {
            let trunc = ('a'..='z').cycle().take(80000).collect::<String>();
            let msg = trunc.chars().chain("what?".chars()).collect::<String>();

            inner(&trunc, &trunc, 80000);
            inner(&msg, &trunc, 80000);
        }
        Some(arg) => panic!("there is no test for CONST_PANIC_MAX_LENGTH={arg:?}"),
    }
}
