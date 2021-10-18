use core::fmt::{self, Debug};

pub struct ArrayString<const LEN: usize> {
    pub(crate) len: usize,
    pub(crate) buffer: [u8; LEN],
}

impl<const LEN: usize> ArrayString<LEN> {
    pub fn get(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.get()).unwrap()
    }
}

impl<const LEN: usize> Debug for ArrayString<LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match core::str::from_utf8(self.get()) {
            Ok(x) => Debug::fmt(x, f),
            Err(_) => f
                .debug_struct("ArrayString")
                .field("len", &self.len)
                .field("buffer", &self.buffer)
                .finish(),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! concat_fmt {
    ($length:expr, $max_len:expr; $($args:tt)*) => (
        $crate::__concat_func!{
            (|args| $crate::for_tests::format_panic_message(args, $length, $max_len))
            []
            [$($args)*,]
        }
    )
}
