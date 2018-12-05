use chrono::prelude::*;
use itertools::{Itertools, PeekingNext};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct GuardSchema {
  pub guard: usize,
  pub start_time: NaiveDateTime,
  pub asleep: Vec<RangeInclusive<NaiveDateTime>>,
}

fn parse_time(row: &str) -> NaiveDateTime {
  NaiveDateTime::parse_from_str(&row[1..17], "%Y-%m-%d %H:%M").unwrap()
}

pub fn calculate_asleep_ratio(guards: &Vec<GuardSchema>, minute: u32) -> f64 {
  guards
    .iter()
    .filter(|guard| {
      guard
        .asleep
        .iter()
        .any(|asleep| asleep.start().minute() <= minute && minute < asleep.end().minute())
    }).count() as f64
    / guards.len() as f64
}

fn as_range(
  iterator: &mut impl Iterator<Item = NaiveDateTime>,
) -> Option<RangeInclusive<NaiveDateTime>> {
  match iterator.next() {
    None => None,
    Some(start) => match iterator.next() {
      None => None,
      Some(end) => Some(start..=end),
    },
  }
}

pub fn as_guard_schema<'a>(
  iterator: &mut impl PeekingNext<Item = &'a &'a str>,
) -> Option<GuardSchema> {
  iterator.next().map(|head| GuardSchema {
    guard: head[26..(head
                  .chars()
                  .enumerate()
                  .skip(26)
                  .find_map(|(i, c)| if c.is_numeric() { None } else { Some(i) })
                  .unwrap())]
      .parse()
      .expect("Guard id is not number"),
    start_time: parse_time(head),
    asleep: iterator
      .peeking_take_while(|row| !row.contains("Guard"))
      .map(|row| parse_time(row))
      .batching(as_range)
      .collect::<Vec<_>>(),
  })
}

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

  let (most_asleep, _) = guards
    .iter()
    .fold(None, |most_asleep: Option<(usize, u32)>, (id, schemas)| {
      let sleep_time = schemas
        .iter()
        .flat_map(|schema| &schema.asleep)
        .map(|asleep| asleep.end().minute() - asleep.start().minute())
        .sum();

      if let Some((most_asleep_id, highest_sleep_time)) = most_asleep {
        if sleep_time > highest_sleep_time {
          Some((*id, sleep_time))
        } else {
          Some((most_asleep_id, highest_sleep_time))
        }
      } else {
        Some((*id, sleep_time))
      }
    }).unwrap();

  let mut minute_asleep_ratio: Vec<_> = (00..=59)
    .map(|minute| {
      (
        minute,
        calculate_asleep_ratio(guards.get(&most_asleep).unwrap(), minute),
      )
    }).collect();

  minute_asleep_ratio.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

  println!(
    "Value: {} {:?}: {}",
    most_asleep,
    minute_asleep_ratio[0],
    most_asleep * minute_asleep_ratio[0].0 as usize
  );
}
