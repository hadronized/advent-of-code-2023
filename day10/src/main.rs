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

enum ScanLine {
  Out,
  WallIn,
  WallOut,
  In,
}

fn enclosed_area(grid: &Grid, pipe_loop: &HashSet<Pos>) -> usize {
  let mut horiz = HashSet::new();
  let mut vert = HashSet::new();

  for (i, line) in grid.iter().enumerate() {
    let mut scan_line = ScanLine::Out;

    for (j, c) in line.iter().enumerate() {
      scan(pipe_loop, (i, j), *c, &mut scan_line, &mut horiz, false);
    }
  }

  for j in 0..grid[0].len() {
    let mut scan_line = ScanLine::Out;

    for i in 0..grid.len() {
      let c = grid[i][j];
      scan(pipe_loop, (i, j), c, &mut scan_line, &mut vert, true);
    }
  }

  println!("{horiz:?}");
  println!("{vert:?}");
  horiz.intersection(&vert).count()
}

fn scan(
  pipe_loop: &HashSet<Pos>,
  pos: Pos,
  c: u8,
  scan_line: &mut ScanLine,
  area: &mut HashSet<Pos>,
  vert: bool,
) {
  let in_pipe_loop = pipe_loop.contains(&pos);
  println!("{pos:?} {c} (in pipe: {in_pipe_loop})", c = c as char);

  match scan_line {
    ScanLine::Out => {
      if in_pipe_loop
        && (!vert && [b'S', b'|', b'F', b'L'].contains(&c)
          || vert && [b'S', b'-', b'F', b'7'].contains(&c))
      {
        println!("  out -> wallin");
        *scan_line = ScanLine::WallIn;
      }
    }

    ScanLine::WallIn => {
      if c == b'.' {
        println!("  wallin -> in XXX");
        *scan_line = ScanLine::In;
        area.insert(pos);
      } else if in_pipe_loop
        && (!vert && [b'7', b'J'].contains(&c) || vert && [b'J', b'L'].contains(&c))
      {
        println!("  wallin -> out");
        *scan_line = ScanLine::Out;
      } else if in_pipe_loop
        && (!vert && [b'|', b'L', b'F'].contains(&c) || vert && [b'-', b'7', b'F'].contains(&c))
      {
        println!("  wallin -> wallout");
        *scan_line = ScanLine::WallOut;
      }
    }

    ScanLine::WallOut => {
      if c == b'.' {
        println!("  wallout -> out");
        *scan_line = ScanLine::Out;
      } else if in_pipe_loop
        && [if vert { b'-' } else { b'|' }, b'7', b'J', b'L', b'F'].contains(&c)
      {
        println!("  wallout -> wallin");
        *scan_line = ScanLine::WallIn;
      }
    }

    ScanLine::In => {
      if c == b'.' {
        println!("  XXX");
        area.insert(pos);
      } else {
        println!("  in -> wallout");
        *scan_line = ScanLine::WallOut;
      }
    }
  }
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
