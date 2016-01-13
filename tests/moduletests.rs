extern crate module;

#[test]
fn first_test() {
    let result = module::yo("Hello".to_string());
    assert_eq!("Hello OK", result);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn should_panic() {
    assert_eq!("1", "2");
}
