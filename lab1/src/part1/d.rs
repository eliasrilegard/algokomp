use std::collections::HashMap;
use std::thread;
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
  let handler = thread::Builder::new()
    .stack_size(256 * 1024 * 1024)
    .spawn(|| {
      let mut elapsed: Vec<String> = vec![];

      for n in (0..7).map(|x| 2_i32.pow(x) * 68_352).collect::<Vec<_>>() {
        // println!("{n}");

        let start = Instant::now();
        let mut cache: HashMap<i32, i32> = HashMap::new();
        let _ = part1::c::cached(n, &mut cache, 5, 6, 7);
        let elapsed_ms = start.elapsed().as_millis();

        elapsed.push(format!("{: >7} | {} ms", n, elapsed_ms));
      }

      println!("{}", elapsed.join("\n"));
    })
    .unwrap();

  handler.join().unwrap();
}

#[allow(dead_code)]
pub fn increment_n() {
  let handler = thread::Builder::new()
    .stack_size(64 * 1024 * 1024)
    .spawn(|| {
      let mut elapsed: Vec<String> = vec![];

      for n in 273408..273418 {
        // println!("{n}");

        let start = Instant::now();
        let mut cache: HashMap<i32, i32> = HashMap::new();
        let _ = part1::c::cached(n, &mut cache, 5, 6, 7);
        let elapsed_ms = start.elapsed().as_millis();

        elapsed.push(format!("{: >6} | {} ms", n, elapsed_ms));
      }

      println!("{}", elapsed.join("\n"));
    })
    .unwrap();

  handler.join().unwrap();
}

#[allow(dead_code)]
pub fn determine_start() {
  // 273408
  let handler = thread::Builder::new()
    .stack_size(64 * 1024 * 1024)
    .spawn(|| {
      let mut n = 262144;
      let mut elapsed_ms: u128 = 0;

      while elapsed_ms < 1000 {
        n = n + 1024;
        // println!("{n}");

        let start = Instant::now();
        let mut cache: HashMap<i32, i32> = HashMap::new();
        let _ = part1::c::cached(n, &mut cache, 5, 6, 7);
        elapsed_ms = start.elapsed().as_millis();
      }

      println!("Reached {n} before exceeding 1 second of runtime");
    })
    .unwrap();

  handler.join().unwrap();
}
