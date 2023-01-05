use trybuild::TestCases;

#[test]
fn ui() {
    let cases = TestCases::new();
    cases.pass("tests/ui/**/pass_*.rs");
    cases.compile_fail("tests/ui/**/fail_*.rs");
}
