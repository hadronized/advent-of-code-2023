use std::collections::HashSet;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

type Grid = Vec<Vec<u8>>;
type Pos = (usize, usize);

fn parse(input: &str) -> Grid {
  input.lines().map(|line| line.bytes().collect()).collect()
}

fn find_start(grid: &Grid) -> Pos {
  for (i, line) in grid.iter().enumerate() {
    for (j, x) in line.iter().enumerate() {
      if x == &b'S' {
        return (i, j);
      }
    }
  }

  unreachable!()
}

fn reachable(grid: &Grid, (i, j): Pos) -> Vec<Pos> {
  let mut res = Vec::new();
  let current = grid[i][j];

  if [b'S', b'|', b'L', b'J'].contains(&current) {
    if let Some(&b'|' | &b'7' | &b'F') = grid.get(i.wrapping_sub(1)).and_then(|l| l.get(j)) {
      res.push((i - 1, j));
    }
  }

  if [b'S', b'|', b'7', b'F'].contains(&current) {
    if let Some(&b'|' | &b'L' | &b'J') = grid.get(i + 1).and_then(|l| l.get(j)) {
      res.push((i + 1, j));
    }
  }

  if [b'S', b'-', b'J', b'7'].contains(&current) {
    if let Some(&b'-' | &b'L' | &b'F') = grid.get(i).and_then(|l| l.get(j.wrapping_sub(1))) {
      res.push((i, j - 1));
    }
  }

  if [b'S', b'-', b'L', b'F'].contains(&current) {
    if let Some(&b'-' | &b'J' | &b'7') = grid.get(i).and_then(|l| l.get(j + 1)) {
      res.push((i, j + 1));
    }
  }

  res
}

fn part1(grid: &Grid) -> usize {
  let start = find_start(grid);
  let mut current = vec![start];
  let mut visited = HashSet::new();
  visited.insert(start);

  while let Some(pos) = current.pop() {
    let next: Vec<_> = reachable(grid, pos)
      .into_iter()
      .filter(|p| !visited.contains(p))
      .collect();
    visited.extend(&next);
    current.extend(next);
  }

  visited.len() / 2
}

fn main() {
  let example = parse(EXAMPLE);
  let input = parse(INPUT);

  println!("example part 1: {:?}", part1(&example));
  println!("input part 1: {:?}", part1(&input));
}
