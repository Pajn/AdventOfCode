use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Mutex;

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
  current_index: usize,
  score: u64,
  current_marble: u64,
}

fn play_rounds() -> impl Iterator<Item = Round> {
  let mut marbles = Rc::new(vec![]);
  let mut current_index = 0;

  (0..).map(move |marble| match marble % 23 {
    0 if marble != 0 => {
      current_index = if current_index < 7 {
        marbles.len() - 7 + current_index
      } else {
        current_index - 7
      };
      let removed_marble = Rc::get_mut(&mut marbles).unwrap().remove(current_index);

      Round {
        current_index: current_index,
        score: removed_marble + marble,
        current_marble: marble,
      }
    }
    _ => {
      if marbles.len() > 1 {
        current_index = (current_index + 2) % (marbles.len() + 0);
        if current_index == 0 {
          current_index = marbles.len();
          Rc::get_mut(&mut marbles).unwrap().push(marble);
        } else {
          Rc::get_mut(&mut marbles)
            .unwrap()
            .insert(current_index, marble);
        }
      } else {
        current_index = marbles.len();
        Rc::get_mut(&mut marbles).unwrap().push(marble);
      }

      Round {
        current_index: current_index,
        score: 0,
        current_marble: marble,
      }
    }
  })
}

pub fn player_tracker(num_players: u32) -> impl Iterator<Item = u32> {
  Some(0).into_iter().chain((1..=num_players).cycle())
}

#[derive(Debug)]
struct GameRound {
  round: Round,
  player: u32,
}

fn play(
  num_players: u32,
) -> (
  Rc<Mutex<BTreeMap<u32, u64>>>,
  impl Iterator<Item = GameRound>,
) {
  let player_scores = Rc::new(Mutex::new(BTreeMap::new()));

  (
    player_scores.clone(),
    play_rounds()
      .zip(player_tracker(num_players))
      .map(move |(round, player)| {
        player_scores
          .lock()
          .expect("Can not borrow as mutable")
          .entry(player)
          .and_modify(|e| *e += round.score)
          .or_insert(0);

        GameRound { round, player }
      }),
  )
}

pub fn run() {
  let mut f = File::open("src/day_09/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let (num_players, highest_score) = parse(&contents);

  let highest_score = highest_score as u64 * 100;
  let (player_scores, game) = play(num_players);
  game
    .take_while(|round| round.round.current_marble <= highest_score)
    .last();

  println!(
    "{} players; last marble is worth {} points\n",
    num_players, highest_score
  );

  let (winning_player, winning_score) = {
    player_scores
      .try_lock()
      .expect("Can not lock scores at result")
      .clone()
      .into_iter()
      .max_by_key(|(_, score)| *score)
      .unwrap()
  };

  println!(
    "Winning player: {}, with score: {}",
    winning_player, winning_score
  );
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
