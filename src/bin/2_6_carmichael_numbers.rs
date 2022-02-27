// カーマイケル数
use num::traits::Pow;
use antbook::modulo::ModFactory;
use proconio::input;
use num::integer;

fn main() {
  input! {
    n: usize
  }
  println!("{}", if solve(n) { "Yes" } else { "No" });
}

// 素朴
fn solve(n: usize) -> bool {
  if is_prime(n) {
    return false
  }
  let f = ModFactory::new(n);
  for i in 1..n {
    let x = f.create(i);
    if x.pow(n) == x {
      return true
    }
  }
  false
}

// 素数判定 O(√n)
fn is_prime(n: usize) -> bool {
  let rn = integer::sqrt(n);
  match (2..rn).find(|&i| n % i == 0) {
    Some(_) => false,
    None => n != 1,
  }
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    assert_eq!(solve(17), false);
  }
  #[test]
  fn example2() {
    assert_eq!(solve(561), true);
  }
  #[test]
  fn example3() {
    assert_eq!(solve(4), false);
  }
}
