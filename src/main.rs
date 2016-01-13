extern crate module;

///# Main
///1. Does nothing.
///2. Continue doing nothing.
///3. Print hello world
#[cfg(not(test))]
fn main() {
	let result = module::yo("Hello".to_string());

    println!("Hello, world! {:?}", result);
}
