use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn main() {
  let lines = io::stdin()
    .lock()
    .lines()
    .map(|l| l.unwrap().parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  let result = iterative(lines[0], lines[1], lines[2], lines[3]);

  println!("{result}");
}

pub fn iterative(n: usize, a: usize, b: usize, c: usize) -> i32 {
  let mut min_coins = vec![i32::MAX; n + 1];
  min_coins[0] = 0;

  for i in 1..=n {
    // Result from adding copper coin
    min_coins[i] = min_coins[i].min(min_coins[i - 1] + 1);

    // Try adding a silver coin instead. Stick to the option
    // that uses the fewest coins.
    if i >= a {
      min_coins[i] = min_coins[i].min(min_coins[i - a] + 1);
    }

    // Test adding gold coin
    if i >= b {
      min_coins[i] = min_coins[i].min(min_coins[i - b] + 1);
    }

    // Test adding platinum coin
    if i >= c {
      min_coins[i] = min_coins[i].min(min_coins[i - c] + 1);
    }
  }

  min_coins[n]
}
