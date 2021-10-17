pub(crate) enum WasTruncated {
    Yes,
    No,
}

// truncates a utf8-encoded string to the character before the `truncate_to` index
//
pub(crate) const fn truncate_str(mut bytes: &[u8], truncate_to: usize) -> (&[u8], WasTruncated) {
    if bytes.len() <= truncate_to {
        (bytes, WasTruncated::No)
    } else {
        while bytes.len() > truncate_to {
            while let [ref rem @ .., b] = *bytes {
                bytes = rem;

                // if it's a non-continuation byte, break
                if (b as i8) >= -0x40 {
                    break;
                }
            }
        }

        (bytes, WasTruncated::Yes)
    }
}

pub(crate) const fn trim_trailing_nul(mut bytes: &[u8]) -> &[u8] {
    while let [ref rem @ .., 0] = *bytes {
        bytes = rem;
    }
    bytes
}
