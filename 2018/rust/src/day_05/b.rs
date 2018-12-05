use day_05::a::compact_polymers;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
  let mut f = File::open("src/day_05/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let chars: Vec<_> = contents.chars().collect();

  let val = (b'a'..=b'z')
    .map(|c_to_remove| {
      println!("{}", (c_to_remove as char).to_string());
      compact_polymers(
        chars
          .iter()
          .map(ToOwned::to_owned)
          .filter(|c| c.to_lowercase().to_string() != (c_to_remove as char).to_string())
          .collect(),
      ).len()
    }).min()
    .unwrap();

  println!("Value: {}", val);
}
