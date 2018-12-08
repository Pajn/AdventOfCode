use crate::day_08::a::*;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
  let mut f = File::open("src/day_08/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let mut values = contents.split(" ").map(|val| val.parse().unwrap());
  let node = parse_node(&mut values);
  let val = node.get_value();

  println!("Value: {}", val)
}
