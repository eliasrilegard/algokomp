use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn main() {
  let lines = io::stdin()
    .lock()
    .lines()
    .map(|l| l.unwrap().to_string())
    .collect::<Vec<_>>();

  let n = lines[0].parse::<u32>().unwrap();
  let k = lines[1].parse::<u32>().unwrap();
  let p = lines[2].parse::<f64>().unwrap();

  let mut cache: HashMap<u32, f64> = HashMap::new();
  let result = g(n, &mut cache, k, p);

  println!("{result}");
}

pub fn g(x: u32, cache: &mut HashMap<u32, f64>, k: u32, p: f64) -> f64 {
  if cache.contains_key(&x) {
    return *cache.get(&x).unwrap();
  }

  match x.cmp(&k) {
    Ordering::Less => 0_f64,
    Ordering::Equal => p.powi(k as i32),
    Ordering::Greater => {
      let gx = g(x - 1, cache, k, p);
      let gxk = g(x - k - 1, cache, k, p);
      cache.insert(x - 1, gx);
      cache.insert(x - k - 1, gxk);

      gx + p.powi(k as i32) * (1_f64 - p) * (1_f64 - gxk)
    }
  }
}
