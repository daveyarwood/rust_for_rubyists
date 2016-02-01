/*
  1.0.0:
  -----
  failing is now called panicking
  fail!() -> panic!()

  1.5.0:
  -----
  - tests now require a `main` function
*/

#[test]
fn this_tests_code() {
  panic!("Fail!");
}

fn main() {}
