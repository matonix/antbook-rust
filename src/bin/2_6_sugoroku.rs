// use num::integer; // これで一発
// https://docs.rs/num/0.4.0/num/integer/trait.Integer.html#method.extended_gcd
// ↑バグっているので使わないほうがいい https://github.com/rust-num/num-integer/issues/40
use proconio::input;
use std::cmp::{max, min};

fn main() {
  input! {
    a: i32,
    b: i32
  }
  dbg!(solve(a, b));
}

// a, b が満たすべき等式: ある x, y に対して ax + by = 1
// これはベズーの等式 https://ja.wikipedia.org/wiki/%E3%83%99%E3%82%BA%E3%83%BC%E3%81%AE%E7%AD%89%E5%BC%8F
// における d = 1 の場合と見なせて、拡張ユークリッドの互除法でベズー係数 x, y を算出できる
fn solve(a: i32, b: i32) -> Option<(i32, i32, i32, i32)> {
  let mut x = 1;
  let mut y = 0; // extgcd 側で初期化されるのであんま意味ない
  let d = extgcd(a, b, &mut x, &mut y);
  if d == 1 {
    Some((max(x, 0), max(y, 0), -min(x, 0), -min(y, 0)))
  } else {
    None
  }
}

// a, bを小さくしながら再帰していき、 b == 0 まできたら x, y を構築し始める
// https://qiita.com/drken/items/b97ff231e43bce50199a
fn extgcd(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
  if b == 0 {
    *x = 1;
    *y = 0;
    a
  } else {
    let d = extgcd(b, a % b, y, x);
    *y -= a / b * *x;
    d
  }
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let a = 4;
    let b = 11;
    let ans = solve(a, b);
    // dbg!(ans);
    assert!(matches!(ans, Some((x, y, z, w)) if x * a + y * b - z * a - w * b == 1));
  }
}
