use crate::day_07::a::*;
use std::fs::File;
use std::io::prelude::*;

fn execute(mut steps: Steps, workers: usize, extra_time: u32) -> u32 {
  let mut executing_steps: Vec<(u32, String)> = vec![];

  for time in 0.. {
    executing_steps = executing_steps
      .into_iter()
      .filter(|(done_time, step)| {
        let is_done = done_time < &time;
        if is_done {
          steps.values_mut().for_each(|depends_on| {
            depends_on.remove(step as &str);
          });
        }

        !is_done
      }).collect();

    if executing_steps.len() < workers {
      if steps.len() == 0 && executing_steps.len() == 0 {
        return time;
      }

      let mut possible_steps: Vec<_> = {
        steps
          .iter()
          .filter(|(_, depends_on)| depends_on.len() == 0)
          .map(|(step, _)| step.to_owned())
          .collect()
      };

      possible_steps.sort();

      let steps_to_take = possible_steps.iter().take(workers - executing_steps.len());

      for step_to_take in steps_to_take {
        executing_steps.push((
          time + extra_time + (step_to_take.bytes().next().unwrap() - b'A') as u32,
          step_to_take.to_string(),
        ));
        steps.remove(step_to_take);
      }
    }
    println!("{:?} {}", executing_steps, time);
  }

  0
}

pub fn run() {
  let mut f = File::open("src/day_07/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let steps = parse(&contents);
  let val = execute(steps, 5, 60);

  println!("Value: {}", val);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_execute() {
    let result = execute(
      parse(
        "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
",
      ),
      2,
      0,
    );

    assert_eq!(result, 15);
  }
}
