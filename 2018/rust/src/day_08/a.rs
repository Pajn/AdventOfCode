use std::fs::File;
use std::io::prelude::*;

pub struct Node {
  sub_nodes: Vec<Node>,
  metadata: Vec<u32>,
}

impl Node {
  fn sum_metadata(&self) -> u32 {
    let own_sum: u32 = self.metadata.iter().sum();
    let child_sum: u32 = self.sub_nodes.iter().map(Node::sum_metadata).sum();

    own_sum + child_sum
  }

  pub fn get_value(&self) -> u32 {
    if self.sub_nodes.len() == 0 {
      let own_sum: u32 = self.metadata.iter().sum();
      own_sum
    } else {
      self
        .metadata
        .iter()
        .filter_map(|index| match index {
          0 => None,
          index => self
            .sub_nodes
            .get((index - 1) as usize)
            .map(Node::get_value),
        }).sum()
    }
  }
}

pub fn parse_node<I>(contents: &mut I) -> Node
where
  I: Iterator<Item = u32>,
{
  let child_node_count = contents.next().expect("child node header not found");
  let metadata_count = contents.next().expect("metadata header not found");

  Node {
    sub_nodes: (0..child_node_count)
      .map(|_| parse_node(contents))
      .collect(),
    metadata: contents.take(metadata_count as usize).collect(),
  }
}

pub fn run() {
  let mut f = File::open("src/day_08/a.input").expect("File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Something went wrong reading the file");

  let mut values = contents.split(" ").map(|val| val.parse().unwrap());
  let node = parse_node(&mut values);
  let val = node.sum_metadata();

  println!("Value: {}", val);
}
