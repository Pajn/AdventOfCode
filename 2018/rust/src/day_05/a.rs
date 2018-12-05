use std::fs::File;
use std::io::prelude::*;

pub fn compact_polymers(mut chars: Vec<char>) -> Vec<char> {
  loop {
    let mut found_pair = false;
    let mut remove_next = false;

    chars = chars
      .iter()
      .enumerate()
      .filter_map(|(i, c)| {
        if remove_next {
          remove_next = false;
          return None;
        }

        let c_next = chars.get(i + 1).unwrap_or(&' ');

        if c != c_next && c.to_lowercase().to_string() == c_next.to_lowercase().to_string() {
          remove_next = true;
          found_pair = true;
          None
        } else {
          Some(c.to_owned())
        }
      }).collect();

    if !found_pair {
      break;
    }
  }

  chars
}

pub fn run() {
  let mut f = File::open("src/day_05/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let chars = contents.chars().collect();
  let chars = compact_polymers(chars);

  println!("Value: {}", chars.iter().collect::<String>());
  println!("Value: {}", chars.len());
}
