use crate::{panic_val::PanicVal, utils::WasTruncated};

#[cold]
#[inline(never)]
#[track_caller]
pub const fn concat_panic(args: &[&[PanicVal<'_>]]) -> ! {
    let len = compute_length(args);

    macro_rules! lengths {
        ($($length: expr),*; $max_length:expr) => (
            match () {
                $(
                    _ if len < $length => panic_inner::<$length>(args),
                )*
                _ => panic_inner::<$max_length>(args)
            }
        )
    }

    lengths! {
        128,
        1024,
        4096;
        32768
    }
}

const fn compute_length(mut args: &[&[PanicVal<'_>]]) -> usize {
    let mut len = 0usize;

    while let [mut outer, ref nargs @ ..] = args {
        while let [arg, nouter @ ..] = outer {
            len += arg.len();
            outer = nouter;
        }
        args = nargs;
    }

    len
}

macro_rules! write_to_buffer {
    ($args:ident, $buffer:ident, $len:ident, $capacity:expr $(,)*) => {
        let mut $buffer = [0u8; LEN];
        let mut $len = 0usize;

        let mut args = $args;
        'outer: while let [mut outer, ref nargs @ ..] = args {
            while let [arg, nouter @ ..] = outer {
                let rem_space = $capacity - $len;
                let (mut string, was_truncated) = arg.string(rem_space);

                while let [byte, ref rem @ ..] = *string {
                    $buffer[$len] = byte;
                    $len += 1;

                    string = rem;
                }

                if let WasTruncated::Yes = was_truncated {
                    break 'outer;
                }
                outer = nouter;
            }
            args = nargs;
        }
    };
}

#[cold]
#[inline(never)]
#[track_caller]
const fn panic_inner<const LEN: usize>(args: &[&[PanicVal<'_>]]) -> ! {
    write_to_buffer! {
        args,
        buffer,
        len,
        LEN,
    }

    unsafe {
        let str = core::str::from_utf8_unchecked(&buffer);
        panic!("{}", str)
    }
}
