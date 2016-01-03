use std::thread;

/*
  1.0.0:
  -----
  1. spawn() is now a method belonging to the Thread type
  2. proc() is deprecated, the new syntax is "move || { do things; }"
  3. {:s} is no longer a valid format trait. Now it's {}, Python style.

  1.5.0:
  -----
  1. 'u' suffix is no longer valid. 'u32' is the new joint
  2. also, seems like it's fine to just use 0 here (not 0u32)
  3. 'range' is no longer a thing. now there is a literal range syntax
  4. std::thread::Thread is now just std::thread

*/

fn main() {
  for _ in 0..10 {
    thread::spawn(move || {
      let greeting_message = "Hello?";
      println!("{}", greeting_message);
    });
  }
}
