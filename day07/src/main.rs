use std::cmp::Reverse;

use itertools::Itertools;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Card {
  NJ,
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
  fn all() -> &'static [Self] {
    &[
      Self::NJ,
      Self::N2,
      Self::N3,
      Self::N4,
      Self::N5,
      Self::N6,
      Self::N7,
      Self::N8,
      Self::N9,
      Self::T,
      Self::J,
      Self::Q,
      Self::K,
      Self::A,
    ]
  }

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
  fn best_j(&self) -> Type {
    Card::all()
      .into_iter()
      .map(|new_card| {
        let cards = self
          .cards
          .iter()
          .map(|card| if let Card::J = card { *new_card } else { *card })
          .collect();
        Hand {
          cards,
          bid: self.bid,
        }
        .ty(false)
      })
      .max()
      .unwrap()
  }

  fn replace_j(&self) -> Self {
    let cards = self
      .cards
      .iter()
      .map(|card| if let Card::J = card { Card::NJ } else { *card })
      .collect();
    Self {
      cards,
      bid: self.bid,
    }
  }

  fn ty(&self, j: bool) -> Type {
    if j {
      return self.best_j();
    }

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

fn solve(hands: &[Hand], j: bool) -> usize {
  let mut hands = hands.to_owned();
  hands.sort_by_key(|hand| {
    (
      hand.ty(j),
      if j {
        hand.replace_j().cards
      } else {
        hand.cards.clone()
      },
    )
  });
  hands
    .iter()
    .enumerate()
    .map(|(rank, hand)| (rank + 1) * hand.bid)
    .sum()
}

fn main() {
  let example_hands = parse(EXAMPLE);
  let input_hands = parse(INPUT);

  println!("example part 1: {}", solve(&example_hands, false));
  println!("input part 1: {}", solve(&input_hands, false));
  println!("example part 2: {}", solve(&example_hands, true));
  println!("input part 2: {}", solve(&input_hands, true));
}
