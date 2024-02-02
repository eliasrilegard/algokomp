use std::collections::HashMap;
use std::time::Instant;

use crate::part2;

#[allow(dead_code)]
pub fn main() {
  determine_start();
  // increment_n();
  // double_n();
}

#[allow(dead_code)]
pub fn double_n() {
  let mut elapsed: Vec<String> = vec![];

  for n in (0..6).map(|x| 2_u32.pow(x) * 216).collect::<Vec<_>>() {
    // println!("{n}");
    let k = n / 2;
    let p: f64 = 0.99;

    let start = Instant::now();
    let mut cache: HashMap<(u32, u32), f64> = HashMap::new();
    let _ = part2::a::f(n, k, &mut cache, k, p);
    let elapsed_ms = start.elapsed().as_millis();

    elapsed.push(format!("{: >4} | {} ms", n, elapsed_ms));
  }

  println!("{}", elapsed.join("\n"));
}

#[allow(dead_code)]
pub fn increment_n() {
  let mut elapsed: Vec<String> = vec![];

  for n in 825..840 {
    // println!("{n}");
    let k = n / 2;
    let p: f64 = 0.99;
  
    let start = Instant::now();
    let mut cache: HashMap<(u32, u32), f64> = HashMap::new();
    let _ = part2::a::f(n, k, &mut cache, k, p);
    let elapsed_ms = start.elapsed().as_millis();

    elapsed.push(format!("{: >3} | {} ms", n, elapsed_ms));
  }

  println!("{}", elapsed.join("\n"));
}

#[allow(dead_code)]
pub fn determine_start() {
  // 825
  let mut n = 800;
  let mut elapsed_ms: u128 = 0;

  while elapsed_ms < 1000 {
    n = n + 1;
    println!("{n}");
    let k = n / 2;
    let p: f64 = 0.99;

    let start = Instant::now();
    let mut cache: HashMap<(u32, u32), f64> = HashMap::new();
    let _ = part2::a::f(n, k, &mut cache, k, p);

    elapsed_ms = start.elapsed().as_millis();
  }

  println!("Reached {n} before exceeding 1 second of runtime");
}
