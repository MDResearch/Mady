use trybuild::TestCases;

#[test]
fn depth1() {
    let t = TestCases::new();
    t.pass("./tests/depth1/*.rs");
}

#[test]
fn syntax() {
    let t = TestCases::new();
    t.pass("./tests/syntax/*.rs");
}

#[test]
fn fail() {
    let t = TestCases::new();
    t.compile_fail("./tests/fail/*.rs");
}
