use std::collections::HashMap;
use std::thread;
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
  // 2 GiB required, wtf
  let handler = thread::Builder::new()
    .stack_size(2048 * 1024 * 1024)
    .spawn(|| {
      let mut elapsed: Vec<String> = vec![];

      for n in (0..8).map(|x| 2_u32.pow(x) * 131_072).collect::<Vec<_>>() {
        // println!("{n}");
        let k = n / 2;
        let p: f64 = 0.99;

        let start = Instant::now();
        let mut cache: HashMap<u32, f64> = HashMap::new();
        let _ = part2::c::g(n, &mut cache, k, p);
        let elapsed_ms = start.elapsed().as_millis();

        elapsed.push(format!("{: >4} | {} ms", n, elapsed_ms));
      }

      println!("{}", elapsed.join("\n"));
    })
    .unwrap();

  handler.join().unwrap();
}

#[allow(dead_code)]
pub fn increment_n() {
  let handler = thread::Builder::new()
    .stack_size(256 * 1024 * 1024)
    .spawn(|| {
      let mut elapsed: Vec<String> = vec![];

      for n in 524288..524303 {
        println!("{n}");
        let k = n / 2;
        let p: f64 = 0.99;

        let start = Instant::now();
        let mut cache: HashMap<u32, f64> = HashMap::new();
        let _ = part2::c::g(n, &mut cache, k, p);
        let elapsed_ms = start.elapsed().as_millis();

        elapsed.push(format!("{: >3} | {} ms", n, elapsed_ms));
      }

      println!("{}", elapsed.join("\n"));
    })
    .unwrap();

  handler.join().unwrap();
}

#[allow(dead_code)]
pub fn determine_start() {
  let handler = thread::Builder::new()
    .stack_size(256 * 1024 * 1024)
    .spawn(|| {
      // 524288
      let mut n = 32_768;
      let mut elapsed_ms: u128 = 0;

      while elapsed_ms < 1000 {
        n = n * 2;
        println!("{n}");
        let k = n / 2;
        let p: f64 = 0.99;

        let start = Instant::now();
        let mut cache: HashMap<u32, f64> = HashMap::new();
        let _ = part2::c::g(n, &mut cache, k, p);
        elapsed_ms = start.elapsed().as_millis();
      }

      println!("Reached {n} before exceeding 1 second of runtime");
    })
    .unwrap();

  handler.join().unwrap();
}
