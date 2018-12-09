use itertools::Itertools;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

pub fn parse(input: &str) -> (u32, u32) {
  let pattern = Regex::new(r"^(\d+) players; last marble is worth (\d+) points$").unwrap();

  let captures = pattern.captures(input).expect("Not matching pattern");

  (
    captures.get(1).unwrap().as_str().parse().unwrap(),
    captures.get(2).unwrap().as_str().parse().unwrap(),
  )
}

struct Round {
  marbles: Vec<u32>,
  current_index: usize,
  score: u32,
  current_marble: u32,
  removed_marble: Option<u32>,
}

impl fmt::Display for Round {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    for (i, marble) in self.marbles.iter().enumerate() {
      if i == self.current_index {
        write!(f, " ({})", marble)?;
      } else {
        write!(f, " {}", marble)?;
      }
    }
    Ok(())
  }
}

impl fmt::Debug for Round {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(
      f,
      "{} -- {} {:?} {}",
      self, self.current_marble, self.removed_marble, self.score
    )?;
    Ok(())
  }
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
        marbles: (*marbles).clone(),
        current_index: current_index,
        score: removed_marble + marble,
        current_marble: marble,
        removed_marble: Some(removed_marble),
      }
    }
    _ => {
      if marbles.len() > 1 {
        current_index = (current_index + 2) % marbles.len();
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
        marbles: (*marbles).clone(),
        current_index: current_index,
        score: 0,
        current_marble: marble,
        removed_marble: None,
      }
    }
  })
}

pub fn player_tracker(num_players: u32) -> impl Iterator<Item = u32> {
  Some(0).into_iter().chain((1..=num_players).cycle())
}

#[allow(unused)]
#[derive(Debug)]
struct GameRound {
  round: Round,
  player: u32,
  player_scores: BTreeMap<u32, u32>,
}

fn play(num_players: u32) -> impl Iterator<Item = GameRound> {
  play_rounds().zip(player_tracker(num_players)).scan(
    BTreeMap::new(),
    |player_scores, (round, player)| {
      player_scores
        .entry(player)
        .and_modify(|e| *e += round.score)
        .or_insert(0);

      Some(GameRound {
        round,
        player,
        player_scores: player_scores.clone(),
      })
    },
  )
}

#[allow(unused)]
fn print<'a>(game: impl Iterator<Item = &'a GameRound>) -> () {
  let output = game
    .map(|game_round| format!("[{}] {:?}", game_round.player, game_round.round))
    .join("\n");

  println!("Value: {}", output);
}

pub fn run() {
  let mut f = File::open("src/day_09/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let (num_players, highest_score) = parse(&contents);
  // let (num_players, highest_score) = parse("9 players; last marble is worth 32 points");
  // let (num_players, highest_score) = parse("10 players; last marble is worth 1618 points");

  let game = play(num_players).take_while(|round| round.round.current_marble <= highest_score);

  // let game = game.collect::<Vec<_>>();
  let last_round = game.last().unwrap();

  // print(game.iter());
  println!(
    "{} players; last marble is worth {} points\n",
    num_players, highest_score
  );

  println!(
    "Winning player: {}, with score: {}",
    last_round
      .player_scores
      .iter()
      .max_by_key(|(_, score)| *score)
      .unwrap()
      .0,
    last_round.player_scores.values().max().unwrap()
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

  #[test]
  fn test_play_rounds() {
    let rounds = play_rounds().take(26).join("\n");

    println!("{}", rounds);

    assert_eq!(
      format!("{}", rounds),
      " (0)
 0 (1)
 0 (2) 1
 0 2 1 (3)
 0 (4) 2 1 3
 0 4 2 (5) 1 3
 0 4 2 5 1 (6) 3
 0 4 2 5 1 6 3 (7)
 0 (8) 4 2 5 1 6 3 7
 0 8 4 (9) 2 5 1 6 3 7
 0 8 4 9 2 (10) 5 1 6 3 7
 0 8 4 9 2 10 5 (11) 1 6 3 7
 0 8 4 9 2 10 5 11 1 (12) 6 3 7
 0 8 4 9 2 10 5 11 1 12 6 (13) 3 7
 0 8 4 9 2 10 5 11 1 12 6 13 3 (14) 7
 0 8 4 9 2 10 5 11 1 12 6 13 3 14 7 (15)
 0 (16) 8 4 9 2 10 5 11 1 12 6 13 3 14 7 15
 0 16 8 (17) 4 9 2 10 5 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 (18) 9 2 10 5 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 9 (19) 2 10 5 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 9 19 2 (20) 10 5 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 9 19 2 20 10 (21) 5 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 9 19 2 20 10 21 5 (22) 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 (19) 2 20 10 21 5 22 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 19 2 (24) 20 10 21 5 22 11 1 12 6 13 3 14 7 15
 0 16 8 17 4 18 19 2 24 20 (25) 10 21 5 22 11 1 12 6 13 3 14 7 15"
    );
  }
}
