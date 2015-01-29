use std::thread::Thread;

/*
  1. spawn() is now a method belonging to the Thread type
  2. proc() is deprecated, the new syntax is "move || { do things; }"
  3. {:s} is no longer a valid format trait. Now it's {}, Python style.
*/

fn main() {
  for _ in range(0u, 10) {
    Thread::spawn(move || {
      let greeting_message = "Hello?";
      println!("{}", greeting_message);
    });
  }
}