use chrono::prelude::*;
use day_04::a::{as_guard_schema, GuardSchema};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {
  let mut f = File::open("src/day_04/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let mut lines: Vec<_> = contents.trim().lines().collect();

  lines.sort();

  let guards_schemas: Vec<_> = lines.iter().batching(as_guard_schema).collect();

  let guards: BTreeMap<usize, Vec<GuardSchema>> =
    guards_schemas
      .into_iter()
      .fold(BTreeMap::new(), |mut guards, schema| {
        guards.entry(schema.guard).or_insert(vec![]).push(schema);
        guards
      });

  let mut minute_asleep_ratio: Vec<_> = (00..=59)
    .map(|minute| {
      (
        minute,
        guards
          .iter()
          .fold(None, |most_asleep, (id, schemas)| {
            let sleep_count = schemas
              .iter()
              .filter(|schema| {
                schema
                  .asleep
                  .iter()
                  .any(|nap| nap.start().minute() <= minute && nap.end().minute() > minute)
              }).count();

            if let Some((most_asleep_id, highest_count)) = most_asleep {
              if sleep_count > highest_count {
                Some((id, sleep_count))
              } else {
                Some((most_asleep_id, highest_count))
              }
            } else {
              Some((id, sleep_count))
            }
          }).unwrap(),
      )
    }).collect();

  minute_asleep_ratio.sort_by(|a, b| (b.1).1.partial_cmp(&(a.1).1).unwrap());

  println!(
    "Value: {:?} {:?}: {}",
    minute_asleep_ratio[0],
    minute_asleep_ratio[1],
    minute_asleep_ratio[0].0 * *(minute_asleep_ratio[0].1).0 as u32
  );
}
