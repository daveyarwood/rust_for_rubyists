/*
  1.9.0:
  -----
  1. `u` is no longer a valid suffix for a numeric literal
  2. `range` is no longer a thing. now there is a literal range syntax
  3. unused variables cause a warning; use _
  4. std::thread::Thread is now just std::thread
*/

use std::thread;

fn main() {
  for _ in 0..500 {
    thread::spawn(move || println!("Hello"));
  }
}
