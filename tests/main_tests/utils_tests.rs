use const_panic::utils::bytes_up_to;

macro_rules! case {
    ($bytes:expr, $upto:expr) => {{
        const SLICE: &[u8] = bytes_up_to($bytes, $upto);
        assert_eq!(slice,);
    }};
}

#[test]
fn test_bytes_up_to_isconst() {
    const SLICE: &[u8] = bytes_up_to(&[10, 20], 1);

    assert_eq!(SLICE, &[10][..]);
}

#[test]
fn test_bytes_up_to() {
    const BYTES: &[u8] = &[3, 5, 8, 13, 21, 34];

    let iter = (0..=BYTES.len() + 2).chain([usize::MAX - 1, usize::MAX]);

    for bytes_len in iter.clone() {
        let bytes = BYTES.get(..bytes_len).unwrap_or(BYTES);
        for upto in iter.clone() {
            assert_eq!(bytes_up_to(bytes, upto), bytes.get(..upto).unwrap_or(bytes),)
        }
    }
}
