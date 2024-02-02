use std::time::Instant;

use crate::part1;

#[allow(dead_code)]
pub fn main() {
  determine_start();
  // increment_n();
  // double_n();
}

#[allow(dead_code)]
pub fn double_n() {
  let mut elapsed: Vec<String> = vec![];

  for n in (0..7).map(|x| (2_i32.pow(x) * 786_432) as usize).collect::<Vec<_>>() {
    // println!("{n}");

    let start = Instant::now();
    let _ = part1::e::iterative(n, 5, 6, 7);
    let elapsed_ms = start.elapsed().as_millis();

    elapsed.push(format!("{: >8} | {} ms", n, elapsed_ms));
  }

  println!("{}", elapsed.join("\n"));
}

#[allow(dead_code)]
pub fn increment_n() {
  let mut elapsed: Vec<String> = vec![];

  for n in 3_145_728..3_145_738 {
    // println!("{n}");

    let start = Instant::now();
    let _ = part1::e::iterative(n, 5, 6, 7);
    let elapsed_ms = start.elapsed().as_millis();

    elapsed.push(format!("{: >6} | {} ms", n, elapsed_ms));
  }

  println!("{}", elapsed.join("\n"));
}

#[allow(dead_code)]
pub fn determine_start() {
  // 3145728
  let mut n = 0;
  let mut elapsed_ms: u128 = 0;

  while elapsed_ms < 1000 {
    n = n + 65_536;
    println!("{n}");

    let start = Instant::now();
    let _ = part1::e::iterative(n, 5, 6, 7);
    elapsed_ms = start.elapsed().as_millis();
  }

  println!("Reached {n} before exceeding 1 second of runtime");
}
