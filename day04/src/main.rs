use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Card {
  winning: HashSet<u32>,
  ours: HashSet<u32>,
}

fn parse(input: &str) -> Vec<Card> {
  input
    .lines()
    .map(|l| {
      let mut parts = l.split(':').nth(1).unwrap().trim().split('|').map(|part| {
        part
          .trim()
          .split_ascii_whitespace()
          .map(|n| n.parse().unwrap())
          .collect()
      });

      Card {
        winning: parts.next().unwrap(),
        ours: parts.next().unwrap(),
      }
    })
    .collect()
}

fn part1(input: &str) -> u32 {
  parse(input)
    .iter()
    .map(|card| {
      let intersections = card.winning.intersection(&card.ours).count() as u32;
      if intersections > 0 {
        2u32.pow(intersections - 1)
      } else {
        0
      }
    })
    .sum()
}

fn part2(input: &str) -> usize {
  let cards = parse(input);
  let mut copies = vec![1usize; cards.len()]; // we start with 1 copy of each card

  for (i, card) in cards.iter().enumerate() {
    let intersections = card.winning.intersection(&card.ours).count();
    for j in i + 1..i + 1 + intersections {
      copies[j] += copies[i];
    }
  }

  copies.into_iter().sum()
}

fn main() {
  println!("example part 1: {:?}", part1(EXAMPLE));
  println!("input part 1: {:?}", part1(INPUT));
  println!("example part 2: {:?}", part2(EXAMPLE));
  println!("input part 2: {:?}", part2(INPUT));
}
