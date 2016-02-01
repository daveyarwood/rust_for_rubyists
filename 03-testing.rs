/*
  1.5.0: Tests now require a `main` function, even if it's not doing anything.

  Running `rustc 03-testing.rs --test && ./03-testing` still works.
*/

#[test]
fn this_tests_code() {
  println!("");
}

fn main() {}
