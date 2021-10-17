use crate::str_utils::{truncate_str, WasTruncated};

#[non_exhaustive]
pub enum PanicArg<'a> {
    Str(&'a str),
}

impl<'a> PanicArg<'a> {
    pub(crate) const fn string(&self, truncate_to: usize) -> (&[u8], WasTruncated) {
        match self {
            Self::Str(str) => truncate_str(str.as_bytes(), truncate_to),
        }
    }
    pub(crate) const fn len(&self) -> usize {
        match self {
            Self::Str(str) => str.len(),
        }
    }
}
