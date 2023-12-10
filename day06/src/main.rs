const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Race {
  time: usize,
  dist: usize,
}

impl Race {
  fn win_presses(&self) -> usize {
    (1..self.time)
      .map(|time_pressed| time_pressed * (self.time - time_pressed))
      .filter(|dist| *dist > self.dist)
      .count() as _
  }
}

fn parse(input: &str) -> Vec<Race> {
  let mut lines = input.lines().map(|line| {
    line
      .split(':')
      .nth(1)
      .unwrap()
      .split_whitespace()
      .map(|n| n.parse().unwrap())
      .collect()
  });
  let times: Vec<_> = lines.next().unwrap();
  let dist: Vec<_> = lines.next().unwrap();

  times
    .into_iter()
    .zip(dist)
    .map(|(time, dist)| Race { time, dist })
    .collect()
}

fn parse2(input: &str) -> Vec<Race> {
  let mut lines = input.lines().map(|line| {
    vec![line
      .split(':')
      .nth(1)
      .unwrap()
      .trim()
      .replace(' ', "")
      .parse()
      .unwrap()]
  });
  let times: Vec<_> = lines.next().unwrap();
  let dist: Vec<_> = lines.next().unwrap();

  times
    .into_iter()
    .zip(dist)
    .map(|(time, dist)| Race { time, dist })
    .collect()
}

fn solve(races: &[Race]) -> usize {
  races.iter().map(|race| race.win_presses()).product()
}

fn main() {
  let example_races = parse(EXAMPLE);
  let input_races = parse(INPUT);

  println!("example part 1: {}", solve(&example_races));
  println!("input part 1: {}", solve(&input_races));

  let example_races2 = parse2(EXAMPLE);
  let input_races2 = parse2(INPUT);

  println!("example part 2: {}", solve(&example_races2));
  println!("input part 2: {}", solve(&input_races2));
}
