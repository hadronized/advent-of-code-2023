use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Dirs {
  dirs: Vec<char>,
}

#[derive(Debug)]
struct Nodes {
  nodes: HashMap<String, (String, String)>,
}

impl Nodes {
  fn starts(&self) -> Vec<String> {
    self
      .nodes
      .keys()
      .filter(|node| node.ends_with('A'))
      .cloned()
      .collect()
  }
}

fn parse(input: &str) -> (Dirs, Nodes) {
  let mut iter = input.split("\n\n");
  let dirs = Dirs {
    dirs: iter.next().unwrap().chars().collect(),
  };
  let nodes = Nodes {
    nodes: iter
      .next()
      .unwrap()
      .lines()
      .map(|line| {
        let mut iter = line.split('=');
        let key = iter.next().unwrap().trim();
        let pair = iter.next().unwrap().trim();
        (
          key.to_owned(),
          (pair[1..4].to_owned(), pair[6..9].to_owned()),
        )
      })
      .collect(),
  };

  (dirs, nodes)
}

fn solve(dirs: &Dirs, nodes: &Nodes, starts: Vec<String>) -> usize {
  let mut dirs = dirs.dirs.iter().cycle();
  let mut current_nodes = starts;
  let mut step = 0;

  while !current_nodes
    .iter()
    .all(|current_node| current_node.ends_with('Z'))
  {
    let dir = dirs.next().unwrap();

    current_nodes.iter_mut().for_each(|current_node| {
      let (left, right) = nodes.nodes.get(current_node.as_str()).unwrap();

      match dir {
        'L' => *current_node = left.clone(),
        'R' => *current_node = right.clone(),
        _ => (),
      }
    });

    step += 1;
  }

  step
}

fn main() {
  let (example_dirs, example_nodes) = parse(EXAMPLE);
  let (input_dirs, input_nodes) = parse(INPUT);

  println!(
    "example part 1: {}",
    solve(&example_dirs, &example_nodes, vec!["AAA".to_string()])
  );
  println!(
    "input part 1: {}",
    solve(&input_dirs, &input_nodes, vec!["AAA".to_string()])
  );
  println!(
    "example part 2: {}",
    solve(&example_dirs, &example_nodes, example_nodes.starts())
  );
  println!(
    "input part 2: {}",
    solve(&input_dirs, &input_nodes, input_nodes.starts())
  );
}
