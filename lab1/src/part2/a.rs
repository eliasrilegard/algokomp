use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[allow(dead_code)]
pub fn main() {
  // Passes the first 27 cases on Kattis
  let lines = io::stdin()
    .lock()
    .lines()
    .map(|l| l.unwrap().to_string())
    .collect::<Vec<_>>();

  let n = lines[0].parse::<u32>().unwrap();
  let k = lines[1].parse::<u32>().unwrap();
  let p = lines[2].parse::<f64>().unwrap();

  let mut cache: HashMap<(u32, u32), f64> = HashMap::new();
  let result = f(n, k, &mut cache, k, p);

  println!("{result}");
}

pub fn f(x: u32, y: u32, cache: &mut HashMap<(u32, u32), f64>, k: u32, p: f64) -> f64 {
  match cache.get(&(x, y)) { // This can and should be written in a better way
    Some(&result) => result,
    None => match y.cmp(&0_u32) {
      Ordering::Less => unreachable!(),
      Ordering::Equal => 1_f64,
      Ordering::Greater => match x.cmp(&0_u32) {
        Ordering::Less => unreachable!(),
        Ordering::Equal => 0_f64,
        Ordering::Greater => {
          let xy = f(x - 1, y - 1, cache, k, p);
          let xk = f(x - 1, k, cache, k, p);
          cache.insert((x - 1, y - 1), xy);
          cache.insert((x - 1, k), xk);

          p * f(x - 1, y - 1, cache, k, p) + (1_f64 - p) * f(x - 1, k, cache, k, p)
        }
      }
    }
  }
}
