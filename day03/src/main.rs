use std::collections::{HashMap, HashSet};

const EXAMPLE: &'static str = include_str!("example.txt");
const INPUT: &'static str = include_str!("input.txt");

type Schematic = Vec<Vec<char>>;

fn parse(input: &str) -> Schematic {
  input.lines().map(|line| line.chars().collect()).collect()
}

fn nearby_symbol(schematic: &Schematic, i: isize, j: isize) -> bool {
  [0, -1, 1]
    .into_iter()
    .flat_map(|l| [0, -1, 1].into_iter().map(move |c| (l, c)))
    .skip(1)
    .flat_map(|(l, c)| schematic.get((i + l) as usize)?.get((j + c) as usize))
    .find(|&&c| !(c == '.' || c.is_ascii_digit()))
    .is_some()
}

fn nearby_gears(schematic: &Schematic, i: isize, j: isize) -> Vec<(usize, usize)> {
  [0, -1, 1]
    .into_iter()
    .flat_map(|l| [0, -1, 1].into_iter().map(move |c| (l, c)))
    .skip(1)
    .filter(|(l, c)| {
      let v = schematic
        .get((i + l) as usize)
        .and_then(|line| line.get((j + c) as usize));
      v == Some(&'*')
    })
    .map(|(l, c)| ((i + l) as usize, (j + c) as usize))
    .collect()
}

fn solve(input: &str) -> (u32, usize) {
  let mut part1 = 0;
  let mut numbers = Vec::default();
  let schematic = parse(input);
  let mut gears: HashMap<(usize, usize), HashSet<usize>> = HashMap::default();

  for (i, line) in schematic.iter().enumerate() {
    let mut j = 0;

    while j < line.len() {
      let c = line[j];

      if c.is_ascii_digit() {
        let mut has_nearby_symbol = false;
        let mut n = 0;

        while j < line.len() && line[j].is_ascii_digit() {
          n = 10 * n + line[j].to_digit(10).unwrap();

          if !has_nearby_symbol {
            has_nearby_symbol = nearby_symbol(&schematic, i as _, j as _);
          }

          for gear in nearby_gears(&schematic, i as _, j as _) {
            gears.entry(gear).or_default().insert(numbers.len());
          }

          j += 1;
        }

        numbers.push(n);

        if has_nearby_symbol {
          part1 += n;
        }
      } else {
        j += 1;
      }
    }
  }

  let part2 = gears
    .into_iter()
    .filter(|(_, nearby)| nearby.len() == 2)
    .map(|(_, nearby)| {
      nearby
        .into_iter()
        .map(|i| numbers[i] as usize)
        .product::<usize>()
    })
    .sum();

  (part1, part2)
}

fn main() {
  println!("example part 1/2: {:?}", solve(EXAMPLE));
  println!("input part 1/2: {:?}", solve(INPUT));
}
