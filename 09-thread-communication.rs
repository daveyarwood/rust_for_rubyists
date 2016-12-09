use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

/*
  1.9.0:
  -----
  1. Like channel, Sender and Receiver now live in std::sync::mpsc.
  2. u is no longer a valid numeric literal suffix
  3. `int` is not a valid type; must use isize, i32, i64, etc.
  4. `range` is no longer a thing. now there is a literal range syntax
  5. unused variables cause a warning; use _
  6. std::thread::Thread is now just std::thread
*/

fn plus_one(sender: &Sender<i64>, receiver: &Receiver<i64>) {
  let mut value: i64;
  loop {
    value = receiver.recv().unwrap();
    println!("child: got {}", value.to_string());
    sender.send(value + 1).unwrap();
    if value == 0 { break; }
  }
}

fn main() {
  let (from_parent_sender, from_parent_receiver) = channel();
  let (from_child_sender, from_child_receiver) = channel();

  thread::spawn(move || {
    plus_one(&from_child_sender, &from_parent_receiver);
  });

  from_parent_sender.send(22).unwrap();
  from_parent_sender.send(23).unwrap();
  from_parent_sender.send(24).unwrap();
  from_parent_sender.send(25).unwrap();

  for _ in 0..4 {
    let answer = from_child_receiver.recv().unwrap();
    println!("parent: got {}", answer.to_string());
  }
}
