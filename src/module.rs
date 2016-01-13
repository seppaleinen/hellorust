pub fn yo(string: String) -> String {
    let result = string + " OK";
    return result;
}

#[test]
fn first_test() {
    let result = "Hello";
    assert_eq!("Hello", result);
}

