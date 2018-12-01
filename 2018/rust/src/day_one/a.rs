use std::fs::File;
use std::io::prelude::*;

pub fn run() {
    let mut f = File::open("src/day_one/a.input").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let val = contents
      .split("\n")
      .filter(|row| !row.is_empty())
      .fold(0, |freq, row| {
        let val: i32 = row[1..].parse().expect("Value not number?");

        match &row[0..1] {
          "+" => freq + val,
          "-" => freq - val,
          op => panic!("Invalid operation {}", op),
        }
      });

    println!("Value: {}", val);
}