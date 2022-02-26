// 線分上の格子点の個数
// ユークリッドの互除法
// use num::integer::gcd; // これで一発
use proconio::input;

fn main() {
  input! {
    p1: (i32, i32),
    p2: (i32, i32)
  }
  println!("{}", solve(p1, p2));
}

fn solve(p1: (i32, i32), p2: (i32, i32)) -> i32 {
  let x = i32::abs(p2.0 - p1.0);
  let y = i32::abs(p2.1 - p1.1);
  match (x, y) {
    (0, 0) => 0,
    (x, 0) => x - 2,
    (0, y) => y - 2,
    (x, y) => gcd(x, y) - 1,
  }
}

fn gcd(x: i32, y: i32) -> i32 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
  }
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let p1 = (1, 11);
    let p2 = (5, 3);
    assert_eq!(solve(p1, p2), 3);
  }
  #[test]
  fn example2() {
    let p1 = (1, 1);
    let p2 = (1, 1);
    assert_eq!(solve(p1, p2), 0);
  }
}
