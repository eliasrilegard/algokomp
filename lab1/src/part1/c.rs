use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn main() {
  let lines = io::stdin()
    .lock()
    .lines()
    .map(|l| l.unwrap().parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  let mut cache: HashMap<i32, i32> = HashMap::new();
  let result = cached(lines[0], &mut cache, lines[1], lines[2], lines[3]);

  println!("{result}");
}

pub fn cached(n: i32, cache: &mut HashMap<i32, i32>, a: i32, b: i32, c: i32) -> i32 {
  match cache.get(&n) {
    Some(&result) => result,
    None => {
      match n.cmp(&0) {
        Ordering::Less => i32::MAX - 1, // Compatibility
        Ordering::Equal => 0,
        Ordering::Greater => {
          let na = cached(n - a, cache, a, b, c);
          let nb = cached(n - b, cache, a, b, c);
          let nc = cached(n - c, cache, a, b, c);
          cache.insert(n - a, na);
          cache.insert(n - b, nb);
          cache.insert(n - c, nc);

          n.min(1 + na).min(1 + nb).min(1 + nc)
        }
      }
    }
  }
}
