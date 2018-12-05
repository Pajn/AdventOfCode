use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;

pub struct Claim {
  pub id: u32,
  pub left: u32,
  pub top: u32,
  pub width: u32,
  pub height: u32,
}

pub fn parse_claims(contents: &str) -> Vec<Claim> {
  let row_pattern = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

  contents
    .split("\n")
    .filter(|row| !row.is_empty())
    .map(|row| {
      let caps = row_pattern.captures(row).unwrap();

      Claim {
        id: (&caps[1]).parse().expect("id is not number"),
        left: (&caps[2]).parse().expect("left is not number"),
        top: (&caps[3]).parse().expect("top is not number"),
        width: (&caps[4]).parse().expect("width is not number"),
        height: (&caps[5]).parse().expect("height is not number"),
      }
    }).collect()
}

pub fn find_overlaps(claims: &Vec<Claim>) -> BTreeSet<(u32, u32)> {
  let (overlaps, _) = claims.iter().fold(
    (BTreeSet::new(), BTreeMap::new()),
    |(mut overlaps, mut rows), claim| {
      for x in (claim.left + 1)..=(claim.left + claim.width) {
        let mut col = rows.entry(x).or_insert(BTreeSet::new());
        for y in (claim.top + 1)..=(claim.top + claim.height) {
          if !col.insert(y) {
            overlaps.insert((x, y));
          }
        }
      }

      (overlaps, rows)
    },
  );

  overlaps
}

pub fn run() {
  let mut f = File::open("src/day_03/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let overlaps = find_overlaps(&parse_claims(&contents));

  println!("Value: {}", overlaps.iter().count());
}
