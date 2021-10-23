use const_panic::{ComputePvCount, TypeDelim};

#[test]
fn compute_pvcount_test() {
    for (field_amount, summed_pv_count, delimiter, expected) in [
        (4, 0, TypeDelim::Tupled, 7),
        (4, 0, TypeDelim::Braced, 11),
        (4, 4, TypeDelim::Tupled, 11),
        (4, 4, TypeDelim::Braced, 15),
        (1, 0, TypeDelim::Tupled, 4),
        (1, 0, TypeDelim::Braced, 5),
        (1, 4, TypeDelim::Tupled, 8),
        (1, 4, TypeDelim::Braced, 9),
    ] {
        assert_eq!(
            ComputePvCount {
                field_amount,
                summed_pv_count,
                delimiter
            }
            .call(),
            expected
        );
    }
}
