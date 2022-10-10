use super::extend_byte_array;

#[test]
fn extend_byte_array_eq_gt_tests() {
    assert_eq!(extend_byte_array::<0, 1>([]), [0]);
    assert_eq!(extend_byte_array::<0, 2>([]), [0, 0]);

    assert_eq!(extend_byte_array::<1, 1>([3]), [3]);
    assert_eq!(extend_byte_array::<1, 2>([3]), [3, 0]);
    assert_eq!(extend_byte_array::<1, 3>([3]), [3, 0, 0]);

    assert_eq!(extend_byte_array::<2, 2>([3, 5]), [3, 5]);
    assert_eq!(extend_byte_array::<2, 3>([3, 5]), [3, 5, 0]);
    assert_eq!(extend_byte_array::<2, 4>([3, 5]), [3, 5, 0, 0]);
}

#[test]
#[should_panic]
fn extend_byte_array_smaller_test_0() {
    let _: [u8; 0] = extend_byte_array([1]);
}

#[test]
#[should_panic]
fn extend_byte_array_smaller_test_1() {
    let _: [u8; 1] = extend_byte_array([1, 2]);
}

#[test]
#[should_panic]
fn extend_byte_array_smaller_test_2() {
    let _: [u8; 0] = extend_byte_array([1, 2]);
}
