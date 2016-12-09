/*
  1.0.0:
  -----
  to_str() is now to_string()

  1.5.0:
  -----
  1. `i` is no longer a valid suffix for a numeric literal.
  2. `int` is not a valid type; must use isize, i32, i64, etc.
  3. `range` is no longer a thing. now there is a literal range syntax
*/


fn div_by_three(num: i64) -> bool {
  num % 3 == 0
}

fn div_by_five(num: i64) -> bool {
  num % 5 == 0
}

fn div_by_fifteen(num: i64) -> bool {
  div_by_three(num) && div_by_five(num)
}

#[test]
fn test_div_by_three() {
  assert!(!div_by_three(1));
}

#[test]
fn test_div_by_three_with_three() {
  assert!(div_by_three(3));
}

#[test]
fn test_div_by_five() {
  assert!(!div_by_five(1));
}

#[test]
fn test_div_by_five_with_five() {
  assert!(div_by_five(5));
}

#[test]
fn test_div_by_fifteen() {
  assert!(!div_by_fifteen(1));
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
  assert!(div_by_fifteen(15));
}

fn main() {
  for num in 1..101 {
    println!("{}",
      if div_by_fifteen(num) { "FizzBuzz".to_string() }
      else if div_by_three(num) { "Fizz".to_string() }
      else if div_by_five(num) { "Buzz".to_string() }
      else { num.to_string() }
    );
  }
}
