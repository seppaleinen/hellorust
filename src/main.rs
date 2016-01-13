///# Main
///1. Does nothing.
///2. Continue doing nothing.
///3. Print hello world
#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
