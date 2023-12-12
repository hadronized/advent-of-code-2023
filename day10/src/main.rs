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

fn find_loop(grid: &Grid) -> HashSet<Pos> {
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

  visited
}

fn enclosed_area(grid: &Grid, pipe_loop: &HashSet<Pos>) -> usize {
  let mut area = 0;

  for (i, line) in grid.iter().enumerate() {
    let mut oddity = 0;

    for (j, c) in line.iter().enumerate() {
      let pos = (i, j);
      let in_pipe_loop = pipe_loop.contains(&pos);
      println!(
        "{pos:?} {c} (in pipe: {in_pipe_loop}; oddity: {oddity})",
        c = *c as char
      );

      if in_pipe_loop && [b'S', b'J', b'7', b'F', b'L', b'|'].contains(c) {
        oddity = 1 - oddity;
      } else if *c == b'.' && (oddity % 2 == 1) {
        println!("  incrementing");
        area += 1;
      }
    }
  }

  area
}

fn main() {
  let example = parse(EXAMPLE);
  let example_loop = find_loop(&example);
  let input = parse(INPUT);
  let input_loop = find_loop(&input);

  println!("example part 1: {}", example_loop.len() / 2);
  println!("input part 1: {}", input_loop.len() / 2);
  println!("example part 2: {}", enclosed_area(&example, &example_loop));
  //println!("input part 2: {}", enclosed_area(&input, &input_loop));
}
