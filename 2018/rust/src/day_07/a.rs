use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::fs::File;
use std::io::prelude::*;

pub type Steps<'a> = BTreeMap<&'a str, BTreeSet<&'a str>>;

pub fn parse(input: &str) -> Steps {
  input.lines().map(|line| (&line[5..6], &line[36..37])).fold(
    BTreeMap::new(),
    |mut steps, (pre_req, step)| {
      steps.entry(pre_req).or_insert(BTreeSet::new());
      steps.entry(step).or_insert(BTreeSet::new()).insert(pre_req);
      steps
    },
  )
}

fn execute(mut steps: Steps) -> String {
  let mut visited = String::new();

  while steps.len() > 0 {
    let mut possible_steps: Vec<_> = {
      steps
        .iter()
        .filter(|(_, depends_on)| depends_on.len() == 0)
        .map(|(step, _)| step.to_owned())
        .collect()
    };

    possible_steps.sort();

    let step_to_take = possible_steps
      .first()
      .expect("There are no possible steps to take");

    write!(visited, "{}", step_to_take).expect("Write error");
    steps.remove(step_to_take);
    steps.values_mut().for_each(|depends_on| {
      depends_on.remove(step_to_take);
    });
  }

  visited
}

pub fn run() {
  let mut f = File::open("src/day_07/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let steps = parse(&contents);
  let val = execute(steps);

  println!("Value: {}", val);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let steps = parse(
      "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
",
    );

    assert_eq!(steps.len(), 6);
    assert_eq!(steps["E"].len(), 3);
  }

  #[test]
  fn test_execute() {
    let result = execute(parse(
      "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
",
    ));

    assert_eq!(result, "CABDFE");
  }
}
