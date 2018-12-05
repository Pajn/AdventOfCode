use day_03::a::{find_overlaps, parse_claims};
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
  let mut f = File::open("src/day_03/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let claims = parse_claims(&contents);
  let overlaps = find_overlaps(&claims);

  for claim in claims {
    let mut has_overlap = false;

    for x in (claim.left + 1)..=(claim.left + claim.width) {
      for y in (claim.top + 1)..=(claim.top + claim.height) {
        if overlaps.contains(&(x, y)) {
          has_overlap = true;
        }
      }
    }

    if !has_overlap {
      println!("No overlap: {}", claim.id);
      break;
    }
  }
}
