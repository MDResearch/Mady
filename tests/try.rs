use trybuild::TestCases;

#[test]
fn depth1() {
    let t=TestCases::new();
    t.pass("./tests/depth1/*.rs")
}