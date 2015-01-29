use std::thread::Thread;

fn main() {
  for num in range(0u, 500) {
    Thread::spawn(move || println!("Hello"));
  }
}