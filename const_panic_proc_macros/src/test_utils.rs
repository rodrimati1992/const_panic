use alloc::{string::String, vec::Vec};

pub(crate) fn remove_whitespaces(x: &str) -> String {
    x.chars().filter(|x| !x.is_whitespace()).collect()
}

pub trait StrExt {
    fn as_str(&self) -> &str;

    /// Checks that these needles exist consequtively in self.
    ///
    /// Example: `"hello world".consecutive_in_set(&["he", "wor"])` returns `true`.
    /// Example: `"hello world".consecutive_in_set(&["wor", "he"])` returns `false`.
    fn consecutive_in_self<S: AsRef<str>>(&self, needles: &[S]) -> bool {
        let mut rem = self.as_str();
        for needle in needles {
            let needle: &str = needle.as_ref();
            rem = match rem.find(needle) {
                Some(next) => &rem[next + needle.len()..],
                None => return false,
            };
        }
        true
    }

    fn consecutive_unspace(&self, needles: &[&str]) -> bool {
        let rem = remove_whitespaces(self.as_str());
        let needles = needles
            .iter()
            .map(|x| remove_whitespaces(x))
            .collect::<Vec<String>>();
        ::std::dbg!(&needles);
        rem.consecutive_in_self(&needles)
    }
}

impl StrExt for str {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }
}

impl StrExt for alloc::string::String {
    #[inline(always)]
    fn as_str(&self) -> &str {
        self
    }
}
