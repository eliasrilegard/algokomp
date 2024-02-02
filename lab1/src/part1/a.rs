use std::cmp::Ordering;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn main() {
  // Passes the first 39 cases
  let lines = io::stdin()
    .lock()
    .lines()
    .map(|l| l.unwrap().parse::<i32>().unwrap())
    .collect::<Vec<_>>();

  let result = direct(lines[0], lines[1], lines[2], lines[3]);

  println!("{result}");
}

// RIP my eyes
pub fn direct(n: i32, a: i32, b: i32, c: i32) -> i32 {
  match n.cmp(&0_i32) {
    Ordering::Less => i32::MAX - 1,
    Ordering::Equal => 0_i32,
    Ordering::Greater => n
      .min(1_i32 + direct(n - a, a, b, c))
      .min(1_i32 + direct(n - b, a, b, c))
      .min(1_i32 + direct(n - c, a, b, c))
  }
}
