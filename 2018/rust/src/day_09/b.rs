use regex::Regex;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

type MarblePointer = Rc<RefCell<MarbleNode>>;

struct MarbleNode {
  value: u64,
  next: Option<MarblePointer>,
  previous: Option<MarblePointer>,
}

impl MarbleNode {
  fn new(value: u64) -> MarblePointer {
    let node = Rc::new(RefCell::new(Self {
      value,
      next: None,
      previous: None,
    }));

    {
      let mut mut_node = node.borrow_mut();

      mut_node.next = Some(node.clone());
      mut_node.previous = Some(node.clone());
    }

    node
  }
}

trait Marble {
  fn next(&self, steps: u32) -> MarblePointer;
  fn previous(&self, steps: u32) -> MarblePointer;
  fn insert(self, marble: MarblePointer) -> MarblePointer;
  fn destroy(&self) -> (MarblePointer, u64);
}

impl Marble for MarblePointer {
  fn next(&self, steps: u32) -> MarblePointer {
    match steps {
      1 => self.borrow().next.as_ref().unwrap().clone(),
      _ => self.borrow().next.as_ref().unwrap().next(steps - 1),
    }
  }

  fn previous(&self, steps: u32) -> MarblePointer {
    match steps {
      1 => self.borrow().previous.as_ref().unwrap().clone(),
      _ => self.borrow().previous.as_ref().unwrap().previous(steps - 1),
    }
  }

  fn insert(self, marble: MarblePointer) -> MarblePointer {
    let next = self.borrow_mut().next.clone().unwrap();
    next.borrow_mut().previous = Some(marble.clone());
    marble.borrow_mut().next = self.borrow_mut().next.take();
    marble.borrow_mut().previous = Some(self.clone());
    self.borrow_mut().next = Some(marble.clone());
    marble
  }

  fn destroy(&self) -> (MarblePointer, u64) {
    let previous = self.borrow_mut().previous.clone().unwrap();
    let next = self.borrow_mut().next.clone().unwrap();
    let dsadas = Some(next.clone());
    next.borrow_mut().previous = Some(previous.clone());
    previous.borrow_mut().next = dsadas;
    (next.clone(), self.borrow().value)
  }
}

fn parse(input: &str) -> (u32, u32) {
  let pattern = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();

  let captures = pattern.captures(input).expect("Not matching pattern");

  (
    captures.get(1).unwrap().as_str().parse().unwrap(),
    captures.get(2).unwrap().as_str().parse().unwrap(),
  )
}

#[derive(Debug)]
struct Round {
  score: u64,
  current_marble: u64,
}

fn play_rounds() -> impl Iterator<Item = Round> {
  let mut current_marble = MarbleNode::new(0);

  (1..).map(move |marble| match marble % 23 {
    0 => {
      let marble_to_remove = current_marble.previous(7).clone();
      let (next_marble, removed_marble) = marble_to_remove.destroy();
      current_marble = next_marble;
      Round {
        score: removed_marble + marble,
        current_marble: marble,
      }
    }
    _ => {
      let next_marble = current_marble.next(1).clone();
      let new_marble = next_marble.insert(MarbleNode::new(marble));
      current_marble = new_marble;

      Round {
        score: 0,
        current_marble: marble,
      }
    }
  })
}

pub fn player_tracker(num_players: u32) -> impl Iterator<Item = u32> {
  (1..=num_players).cycle()
}

#[derive(Debug)]
struct GameRound {
  round: Round,
  player: u32,
  score: u64,
}

fn play(num_players: u32, num_rounds: u64) -> u64 {
  let mut player_scores = BTreeMap::new();

  play_rounds()
    .zip(player_tracker(num_players))
    .map(|(round, player)| {
      let score = *player_scores
        .entry(player)
        .and_modify(|e| *e += round.score)
        .or_insert(0);

      GameRound {
        round,
        player,
        score,
      }
    })
    .find(|round| round.round.current_marble == num_rounds);

  *player_scores.values().max().unwrap()
}

pub fn run() {
  let mut f = File::open("src/day_09/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let (num_players, num_rounds) = parse(&contents);

  let num_rounds = num_rounds as u64 * 100;
  println!("num_rounds: {}", num_rounds);
  let highest_score = play(num_players, num_rounds);

  println!(
    "{} players; last marble is worth {} points\n",
    num_players, num_rounds
  );

  println!("Winning score: {}", highest_score);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let input1 = parse("9 players; last marble is worth 32 points");
    let input2 = parse("10 players; last marble is worth 1618 points");

    assert_eq!(input1, (9, 32));
    assert_eq!(input2, (10, 1618));
  }
}
