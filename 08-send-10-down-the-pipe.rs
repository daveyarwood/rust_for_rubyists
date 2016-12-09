use std::thread;
use std::sync::mpsc::channel;

/*
  1.0.0:
  -----
  1. channel() now lives in std::sync::mpsc
  2. unwrap() is now a thing -- without it, you get a Result ("Ok(10)")

  1.9.0:
  -----
  1. 'u' suffix is no longer valid
  4. std::thread::Thread is now just std::thread
*/

fn main() {
  let (chan, port) = channel();

  thread::spawn(move || chan.send(10));

  println!("{}", port.recv().unwrap().to_string());
}
