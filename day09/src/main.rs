use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

fn parse(input: &str) -> Vec<Vec<i64>> {
  input
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
    })
    .collect()
}

fn hist_next(hist: &[i64]) -> i64 {
  let diffs: Vec<_> = hist
    .iter()
    .zip(hist.iter().skip(1))
    .map(|(a, b)| b - a)
    .collect();

  let set: HashSet<_> = diffs.iter().collect();
  if set.len() == 1 {
    // no need to go any deeper; start going back up with this value
    hist.last().unwrap() + *diffs.last().unwrap()
  } else {
    hist.last().unwrap() + hist_next(&diffs)
  }
}

fn hist_prev(hist: &[i64]) -> i64 {
  let diffs: Vec<_> = hist
    .iter()
    .zip(hist.iter().skip(1))
    .map(|(a, b)| b - a)
    .collect();

  let set: HashSet<_> = diffs.iter().collect();
  if set.len() == 1 {
    // no need to go any deeper; start going back up with this value
    hist.first().unwrap() - *diffs.first().unwrap()
  } else {
    hist.first().unwrap() - hist_prev(&diffs)
  }
}

fn part1(hist: &[Vec<i64>]) -> i64 {
  hist.iter().map(|hist| hist_next(hist)).sum()
}

fn part2(hist: &[Vec<i64>]) -> i64 {
  hist.iter().map(|hist| hist_prev(hist)).sum()
}

fn main() {
  let example = parse(EXAMPLE);
  let input = parse(INPUT);

  println!("example part 1: {}", part1(&example));
  println!("input part 1: {}", part1(&input));
  println!("example part 2: {}", part2(&example));
  println!("input part 2: {}", part2(&input));
}
