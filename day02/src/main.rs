const EXAMPLE: &'static str = include_str!("example.txt");
const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Game {
  id: usize,
  rounds: Vec<Round>,
}

#[derive(Debug, Default)]
struct Round {
  reds: u32,
  blues: u32,
  greens: u32,
}

fn parse(input: &str) -> Vec<Game> {
  input
    .lines()
    .enumerate()
    .map(|(id, line)| {
      let games = line
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(";")
        .map(|game| {
          let mut round = Round::default();
          for s in game.trim().split(",") {
            let mut iter = s.trim().split_whitespace();
            let n: u32 = iter.next().unwrap().parse().unwrap();

            match iter.next().unwrap() {
              "red" => round.reds = n,
              "green" => round.greens = n,
              "blue" => round.blues = n,
              _ => (),
            }
          }

          round
        })
        .collect();

      Game {
        id: id + 1,
        rounds: games,
      }
    })
    .collect()
}

fn part1(input: &str) -> u32 {
  parse(input)
    .iter()
    .filter(|game| {
      game
        .rounds
        .iter()
        .all(|round| round.reds <= 12 && round.greens <= 13 && round.blues <= 14)
    })
    .map(|game| game.id as u32)
    .sum()
}

fn part2(input: &str) -> u32 {
  parse(input)
    .iter()
    .map(|game| {
      let max = game
        .rounds
        .iter()
        .fold(Round::default(), |max, round| Round {
          reds: max.reds.max(round.reds),
          greens: max.greens.max(round.greens),
          blues: max.blues.max(round.blues),
        });

      max.reds * max.greens * max.blues
    })
    .sum()
}

fn main() {
  println!("example part 1: {}", part1(EXAMPLE));
  println!("input part 1: {}", part1(INPUT));
  println!("example part 2: {}", part2(EXAMPLE));
  println!("input part 2: {}", part2(INPUT));
}
