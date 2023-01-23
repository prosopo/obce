use trybuild::TestCases;

#[test]
fn ui() {
    let cases = TestCases::new();
    cases.pass("tests/ui/substrate/**/pass_*.rs");
    cases.compile_fail("tests/ui/substrate/**/fail_*.rs");
}
