use std::fs::File;
use std::io::prelude::*;
use std::ops::Add;

pub fn run() {
    let mut f = File::open("src/day_one/a.input").expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let val = contents
      .split("\n")
      .filter(|row| !row.is_empty())
      .map(|row| row.parse::<i32>().expect("Value not number?"))
      .fold(0, Add::add);

    println!("Value: {}", val);
}