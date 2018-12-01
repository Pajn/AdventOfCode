use std::collections::HashSet;
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
      .cycle()    
      .try_fold((0, HashSet::new()), |(freq, mut visited_values), row| {
        let val: i32 = row[1..].parse().expect("Value not number?");
        let new_freq = match &row[0..1] {
          "+" => freq + val,
          "-" => freq - val,
          op => panic!("Invalid operation {}", op),
        };

        if visited_values.contains(&new_freq) {
          return Err(new_freq)
        }

        visited_values.insert(new_freq);
        
        Ok((new_freq, visited_values))
      })
      .unwrap_err();

    println!("Visited Twice: {}", val);
}