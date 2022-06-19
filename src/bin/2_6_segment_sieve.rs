// 区間ふるい
use num::integer;
use num::integer::Integer;
use proconio::input;

fn main() {
  input! {
    a: usize,
    b: usize
  }
  println!("{}", solve(a, b));
}

fn solve(a: usize, b: usize) -> usize {
  segment_sieve(a, b)
    .iter()
    .map(|b| if *b { 1 } else { 0 })
    .sum()
}

// 区間素数列挙 O(n log log n)
#[allow(unstable_name_collisions)]
fn segment_sieve(a: usize, b: usize) -> Vec<bool> {
  let rb = integer::sqrt(b);
  let mut is_prime_small = vec![true; rb]; // [0, √b)
  let mut is_prime_segment = vec![true; b - a]; // [a, b) => [0, b - a)
  for i in 2..rb {
    if is_prime_small[i] {
      for j in (2 * i..rb).step_by(i) {
        is_prime_small[j] = false;
      }
      // a と等しいかより大きい最小の i の倍数 k: k|i && k >= a
      let k = a.next_multiple_of(&i);
      // k を a で割った余りが [0, b - a) で最初に篩にかけられる値
      for j in (k % a..b - a).step_by(i) {
        is_prime_segment[j] = false;
      }
    }
  }
  is_prime_segment
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    assert_eq!(solve(22, 37), 3);
  }
  #[test]
  fn example2() {
    assert_eq!(solve(22801763489, 22801787297), 1000);
  }
}
