use std::thread::Thread;
use std::sync::mpsc::{channel, Sender, Receiver};

/*
  Like channel, Sender and Receiver now live in std::sync::mpsc.

  Unresolved -- this file will compile, but the receiver blocks while
  waiting for a value. Documentation on the Rust site (the Rust book)
  seems to be out-of-date for the nightly build. I can't figure out
  how to get plus_one to run as an asynchronous, non-blocking thread.
*/

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
  let mut value: int;
  loop {
    println!("waiting on a value...");
    value = receiver.recv().unwrap();
    println!("plus_one got a value {}.", value.to_string());
    println!("sending value + 1.");
    sender.send(value + 1).unwrap();
    if value == 0 { break; }
  }
}

fn main() {
  println!("Starting program.");

  let (from_parent_sender, from_parent_receiver) = channel();
  let (from_child_sender, from_child_receiver) = channel();

  println!("defined channels.");

  Thread::spawn(move || {
    plus_one(&from_child_sender, &from_parent_receiver);
  });

  println!("test");

  from_parent_sender.send(22).unwrap();
  from_parent_sender.send(23).unwrap();
  from_parent_sender.send(24).unwrap();
  from_parent_sender.send(25).unwrap();

  for _ in range(0u, 4) {
    let answer = from_child_receiver.recv().unwrap();
    println!("{}", answer.to_string());
  }
}