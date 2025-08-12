// The command I'm currently using:
// clear;clear; env TRYBUILD=overwrite cargo test \
// --features "__ui_tests rust_latest_stable derive"
//

#[cfg(feature = "__ui_tests")]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    for dir in ["panicfmt_ui_tests"] {
        t.compile_fail(format!("tests/main_tests/{}/*-err.rs", dir));
        t.pass(format!("tests/main_tests/{}/*fine.rs", dir));
    }
}
