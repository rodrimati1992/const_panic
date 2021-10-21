use const_panic::{
    __set_fmt_from_kw as set_fmt_fkw,
    fmt::{self, FmtArg, FmtKind},
};

#[test]
fn set_fmt_from_kw_test() {
    let mut inita = FmtArg::DISPLAY;
    #[cfg(feature = "non_basic")]
    {
        inita.indentation = fmt::INDENTATION_STEP;
    }
    inita.is_alternate = false;
    inita.fmt_kind = FmtKind::Display;

    let mut initb = FmtArg::DISPLAY;
    #[cfg(feature = "non_basic")]
    {
        initb.indentation = fmt::INDENTATION_STEP;
    }
    initb.is_alternate = true;
    initb.fmt_kind = FmtKind::Display;

    let mut initc = FmtArg::DISPLAY;
    #[cfg(feature = "non_basic")]
    {
        initc.indentation = fmt::INDENTATION_STEP;
    }
    initc.is_alternate = false;
    initc.fmt_kind = FmtKind::Debug;

    let mut initd = FmtArg::DISPLAY;
    #[cfg(feature = "non_basic")]
    {
        initd.indentation = fmt::INDENTATION_STEP;
    }
    initd.is_alternate = true;
    initd.fmt_kind = FmtKind::Debug;

    macro_rules! case {
        ($kw:tt, $init:ident, |$other:ident| $expected:block) => ({
            #[allow(unused_mut)]
            let mut fmt_ = $init;

            #[allow(unused_mut)]
            let mut $other = $init;

            $expected

            assert_eq!(set_fmt_fkw!($kw, fmt_), $other);
        });
    }

    #[cfg(feature = "non_basic")]
    {
        case! {open, inita, |fmt| {fmt.indentation = fmt::INDENTATION_STEP * 2;}}
        case! {close, inita, |fmt| {fmt.indentation = 0;}}

        {
            let mut fmt_ = inita;
            set_fmt_fkw!(open, fmt_);
            assert_eq!(fmt_.indentation, fmt::INDENTATION_STEP * 2);
        }
        {
            let mut fmt_ = inita;
            set_fmt_fkw!(close, fmt_);
            assert_eq!(fmt_.indentation, 0);
        }
    }

    case! {display, initd, |fmt| {
        fmt.fmt_kind = FmtKind::Display;
        fmt.is_alternate = false;
    }}
    case! {{}, initd, |fmt| {
        fmt.fmt_kind = FmtKind::Display;
        fmt.is_alternate = false;
    }}
    case! {alt_display, initc, |fmt| {
        fmt.fmt_kind = FmtKind::Display;
        fmt.is_alternate = true;
    }}
    case! {{#}, initc, |fmt| {
        fmt.fmt_kind = FmtKind::Display;
        fmt.is_alternate = true;
    }}

    case! {debug, initb, |fmt| {
        fmt.fmt_kind = FmtKind::Debug;
        fmt.is_alternate = false;
    }}
    case! {{?}, initb, |fmt| {
        fmt.fmt_kind = FmtKind::Debug;
        fmt.is_alternate = false;
    }}
    case! {alt_debug, inita, |fmt| {
        fmt.fmt_kind = FmtKind::Debug;
        fmt.is_alternate = true;
    }}
    case! {{#?}, inita, |fmt| {
        fmt.fmt_kind = FmtKind::Debug;
        fmt.is_alternate = true;
    }}
}
