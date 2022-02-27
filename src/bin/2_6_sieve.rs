// エラトステネスのふるい
use proconio::input;

fn main() {
  input! {
    n: usize
  }
  println!("{}", solve(n));
}

fn solve(n: usize) -> usize {
  sieve(n).iter().map(|b| if *b { 1 } else { 0 }).sum()
}

// 素数列挙 O(n log log n)
fn sieve(n: usize) -> Vec<bool> {
  // let mut prime = vec![0; n + 1];
  let mut is_prime = vec![true; n + 1];
  // let mut p = 0;
  is_prime[0] = false;
  is_prime[1] = false;
  for i in 2..=n {
    if is_prime[i] {
      // prime[p] = i;
      // p += 1;
      // https://scrapbox.io/nwtgck/Rust%E3%81%AErange%E3%81%A7%E3%82%B9%E3%83%86%E3%83%83%E3%83%97%E6%95%B0(=by)%E3%82%92%E6%8C%87%E5%AE%9A%E3%81%97%E3%81%9F%E3%81%84%E3%81%A8%E3%81%8D
      for j in (2 * i..=n).step_by(i) {
        is_prime[j] = false;
      }
    }
  }
  is_prime
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    assert_eq!(solve(11), 5);
  }
  #[test]
  fn example2() {
    assert_eq!(solve(1000000), 78498);
  }
}
