use num::integer;
use num::integer::Integer;
use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize
  }
  println!("{}", if solve(n) { "Yes" } else { "No" });
  println!("{:?}", divisor(n));
  println!("{:?}", prime_factor(n));
}

fn solve(n: usize) -> bool {
  is_prime(n)
}

// 素数判定 O(√n)
fn is_prime(n: usize) -> bool {
  let rn = integer::sqrt(n);
  match (2..rn).find(|&i| n % i == 0) {
    Some(_) => false,
    None => n != 1,
  }
}

// 約数列挙 O(√n)
fn divisor(n: usize) -> Vec<usize> {
  let rn = integer::sqrt(n);
  let mut v = vec![];
  for i in 2..rn {
    if n.is_multiple_of(&i) {
      v.push(i);
      if n / i != i {
        v.push(n / i);
      }
    }
  }
  v
}

fn prime_factor(mut n: usize) -> HashMap<usize, usize> {
  let mut h = HashMap::new();
  let rn = integer::sqrt(n);
  for i in 2..rn {
    while n.is_multiple_of(&i) {
      if let Some(v) = h.get_mut(&i) {
        *v += 1;
        n /= i;
      } else {
        h.insert(i, 1);
        n /= i;
      }
    }
  }
  if n != 1 {
    h.insert(n, 1);
  }
  h
}

#[cfg(test)]
mod tests {
  use super::divisor;
  use super::prime_factor;
  use super::solve;

  use maplit::hashmap;

  #[test]
  fn example1() {
    assert_eq!(solve(53), true);
  }
  #[test]
  fn example2() {
    assert_eq!(solve(295927), false);
  }
  #[test]
  fn test_divisor() {
    assert_eq!(divisor(295927), vec![541, 547]);
  }
  #[test]
  fn test_prime_factor() {
    assert_eq!(prime_factor(295927), hashmap!(541 => 1, 547 => 1));
  }
}
