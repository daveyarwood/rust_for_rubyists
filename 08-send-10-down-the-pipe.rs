use std::thread::Thread;
use std::sync::mpsc::channel;

/*
  1. channel() now lives in std::sync::mpsc
  2. unwrap() is now a thing -- without it, you get a Result ("Ok{10}")
*/

fn main() {
  let (chan, port) = channel();

  Thread::spawn(move || chan.send(10u));

  println!("{}", port.recv().unwrap().to_string());
}