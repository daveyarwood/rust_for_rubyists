/*
  1.9.0:
  -----
  1. `u` is no longer a valid suffix for a numeric literal
  2. `range` is no longer a thing. now there is a literal range syntax
  3. unused variables cause a warning; use _
*/

fn main() {
  for _ in 0..500 {
    println!("Hello");
  }
}
