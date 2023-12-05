const EXAMPLE1: &'static str = include_str!("example1.txt");
const EXAMPLE2: &'static str = include_str!("example2.txt");
const INPUT: &'static str = include_str!("input.txt");

fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(|line| {
      let v: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
      v[0].to_digit(10).unwrap() * 10 + v[v.len() - 1].to_digit(10).unwrap()
    })
    .sum()
}

fn part2(input: &str) -> u32 {
  input.lines().map(reduce).sum()
}

fn reduce(mut input: &str) -> u32 {
  let mut digits = Vec::default();

  let d = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];

  'a: while !input.is_empty() {
    for (i, d) in d.iter().enumerate() {
      if input.starts_with(*d) {
        digits.push(i as u32 + 1);
        input = &input[d.len()..];
        continue 'a;
      }
    }

    if let Some(next) = input.chars().next() {
      if next.is_ascii_digit() {
        digits.push(next.to_digit(10).unwrap());
      }

      input = &input[1..];
    }
  }

  digits[0] * 10 + digits[digits.len() - 1]
}

fn main() {
  println!("example 1: {}", part1(EXAMPLE1));
  println!("input 1: {}", part1(INPUT));
  println!("example 2: {}", part2(EXAMPLE2));
  println!("input 2: {}", part2(INPUT));
}
