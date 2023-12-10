const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct CatMap {
  src: u64,
  dest: u64,
  range: u64,
}

impl CatMap {
  fn get(&self, src: u64) -> Option<u64> {
    if src >= self.src && src <= (self.src + self.range) {
      Some(self.dest + src - self.src)
    } else {
      None
    }
  }

  /// Resolve a range by splitting it according to the rule.
  ///
  /// Range (a,b) and rule (x,y):
  ///
  /// Case not generating any resolved range:
  ///
  /// Case 1:
  /// -------a···b--x···y---
  ///
  /// Case 2:
  /// -------x···y--a···b---
  ///
  /// Cases generating a resolved range:
  ///
  /// Case 3:
  /// -------a···x··b···y---
  ///
  /// Case 4:
  /// -------a···x··y···b---
  ///
  /// Case 5:
  /// -------x···a··b···y---
  ///
  /// Case 6:
  /// -------x···a··y···b---
  fn resolve_range(&self, range: (u64, u64)) -> (Option<(u64, u64)>, [Option<(u64, u64)>; 2]) {
    let src_end = self.src + self.range;

    if range.1 < self.src || range.0 > src_end {
      // case 1 & 2
      (None, [Some(range), None])
    } else if range.0 < self.src {
      if range.1 <= src_end {
        // case 3
        (
          Some((self.dest, self.dest + range.1 - self.src)),
          [Some((range.0, self.src - 1)), None],
        )
      } else {
        // case 4
        (
          Some((self.dest, self.dest + self.range)),
          [Some((range.0, self.src - 1)), Some((src_end + 1, range.1))],
        )
      }
    } else if range.1 <= src_end {
      // case 5
      (
        Some((
          self.dest + range.0 - self.src,
          self.dest + range.1 - self.src,
        )),
        [None, None],
      )
    } else {
      // case 6
      (
        Some((self.dest + range.0 - self.src, self.dest + self.range)),
        [Some((src_end + 1, range.1)), None],
      )
    }
  }
}

#[derive(Debug)]
struct Maps {
  cat_maps: Vec<CatMap>,
}

impl Maps {
  fn get(&self, src: u64) -> u64 {
    self
      .cat_maps
      .iter()
      .find_map(|cm| cm.get(src))
      .unwrap_or(src)
  }

  fn resolve_ranges(&self, ranges: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut resolved_ranges = Vec::new();

    for range in ranges {
      let mut unresolved_ranges = vec![*range];

      for map in &self.cat_maps {
        if let Some(range) = unresolved_ranges.pop() {
          let (resolved, unresolved) = map.resolve_range(range);

          if let Some(resolved) = resolved {
            resolved_ranges.push(resolved);
          }

          unresolved_ranges.extend(unresolved.into_iter().flatten());
        } else {
          break;
        }
      }

      resolved_ranges.extend(unresolved_ranges);
    }

    resolved_ranges
  }
}

#[derive(Debug)]
struct Almanac {
  seeds: Vec<u64>,
  maps: Vec<Maps>,
}

impl Almanac {
  fn parse(input: &str) -> Self {
    let mut parts = input.split("\n\n");
    let seeds = parts
      .next()
      .unwrap()
      .split(':')
      .nth(1)
      .unwrap()
      .split_whitespace()
      .map(|n| n.parse().unwrap())
      .collect();

    let maps = parts
      .map(|part| {
        let cat_maps = part
          .lines()
          .skip(1)
          .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse().unwrap());
            let dest = numbers.next().unwrap();
            let src = numbers.next().unwrap();
            let range = numbers.next().unwrap();
            CatMap { src, dest, range }
          })
          .collect();

        Maps { cat_maps }
      })
      .collect();

    Almanac { seeds, maps }
  }

  fn seeds_as_ranges(&self) -> Vec<(u64, u64)> {
    self
      .seeds
      .iter()
      .step_by(2)
      .zip(self.seeds.iter().skip(1).step_by(2))
      .map(|(a, b)| (*a, *a + *b - 1))
      .collect()
  }
}

fn part1(almanac: &Almanac) -> u64 {
  almanac
    .seeds
    .iter()
    .map(|seed| almanac.maps.iter().fold(*seed, |src, maps| maps.get(src)))
    .min()
    .unwrap()
}

fn part2(almanac: &Almanac) -> u64 {
  almanac
    .maps
    .iter()
    .fold(almanac.seeds_as_ranges(), |ranges, maps| {
      maps.resolve_ranges(&ranges)
    })
    .into_iter()
    .map(|(a, _)| a)
    .min()
    .unwrap()
}

fn main() {
  let example = Almanac::parse(EXAMPLE);
  let input = Almanac::parse(INPUT);

  println!("example part 1: {}", part1(&example));
  println!("input part 1: {}", part1(&input));

  println!("example part 2: {}", part2(&example));
  println!("input part 2: {}", part2(&input));
}

#[cfg(test)]
mod test {
  use crate::CatMap;

  #[test]
  fn resolve_ranges() {
    let map = CatMap {
      src: 10,
      dest: 100,
      range: 20,
    };

    assert_eq!(
      map.resolve_range((10, 29)),
      (Some((100, 119)), [None, None])
    );
    assert_eq!(
      map.resolve_range((10, 30)),
      (Some((100, 120)), [None, None])
    );
    assert_eq!(
      map.resolve_range((10, 35)),
      (Some((100, 120)), [Some((31, 35)), None])
    );
    assert_eq!(map.resolve_range((0, 9)), (None, [Some((0, 9)), None]));
    assert_eq!(
      map.resolve_range((0, 10)),
      (Some((100, 100)), [Some((0, 9)), None])
    );
    assert_eq!(
      map.resolve_range((0, 50)),
      (Some((100, 120)), [Some((0, 9)), Some((31, 50))])
    );
  }
}
