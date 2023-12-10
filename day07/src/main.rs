use std::cmp::Reverse;

use itertools::Itertools;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
  N2,
  N3,
  N4,
  N5,
  N6,
  N7,
  N8,
  N9,
  T,
  J,
  Q,
  K,
  A,
}

impl Card {
  fn parse(c: char) -> Self {
    match c {
      '2' => Self::N2,
      '3' => Self::N3,
      '4' => Self::N4,
      '5' => Self::N5,
      '6' => Self::N6,
      '7' => Self::N7,
      '8' => Self::N8,
      '9' => Self::N9,
      'T' => Self::T,
      'J' => Self::J,
      'Q' => Self::Q,
      'K' => Self::K,
      'A' => Self::A,
      _ => panic!("nope"),
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOfAKind,
  FullHouse,
  FourOfAKind,
  FiveOfAKind,
}

#[derive(Clone, Debug)]
struct Hand {
  cards: Vec<Card>,
  bid: usize,
}

impl Hand {
  fn ty(&self) -> Type {
    let mut cards = self.cards.clone();
    cards.sort();

    let groups = cards.into_iter().group_by(|x| *x);
    let mut grouped: Vec<_> = groups.into_iter().map(|(k, g)| (k, g.count())).collect();
    grouped.sort_by_key(|(_, len)| Reverse(*len));

    if grouped.len() == 1 {
      Type::FiveOfAKind
    } else if grouped[0].1 == 4 {
      Type::FourOfAKind
    } else if grouped[0].1 == 3 {
      if grouped.len() == 2 {
        Type::FullHouse
      } else {
        Type::ThreeOfAKind
      }
    } else if grouped[0].1 == 2 {
      if grouped.len() == 3 {
        Type::TwoPair
      } else {
        Type::OnePair
      }
    } else {
      Type::HighCard
    }
  }
}

fn parse(input: &str) -> Vec<Hand> {
  input
    .lines()
    .map(|line| {
      let mut parts = line.split_whitespace();
      let cards = parts.next().unwrap().chars().map(Card::parse).collect();
      let bid = parts.next().unwrap().parse().unwrap();

      Hand { cards, bid }
    })
    .collect()
}

fn part1(hands: &[Hand]) -> usize {
  let mut hands = hands.to_owned();
  hands.sort_by_key(|hand| (hand.ty(), hand.cards.clone()));
  hands
    .iter()
    .enumerate()
    .map(|(rank, hand)| (rank + 1) * hand.bid)
    .sum()
}

fn main() {
  let example_hands = parse(EXAMPLE);
  let input_hands = parse(INPUT);

  println!("example part 1: {}", part1(&example_hands));
  println!("input part 1: {}", part1(&input_hands));
}
