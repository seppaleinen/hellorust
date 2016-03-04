#[macro_use]
extern crate log;

/// #Public function yo
/// ###Takes String and returns String
pub fn yo(string: String) -> String {
    let result = string + " OK";
    info!("Result: {:?}", result);
    return result;
}

#[cfg(test)]
mod unittests {
   #[test]
   fn first_test() {
      let result = "Hello";
      assert_eq!("Hello", result);
   }
}
