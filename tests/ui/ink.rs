use trybuild::TestCases;

#[test]
fn ui() {
    let cases = TestCases::new();
    cases.pass("tests/ui/ink/**/pass_*.rs");
    cases.compile_fail("tests/ui/ink/**/fail_*.rs");
}
