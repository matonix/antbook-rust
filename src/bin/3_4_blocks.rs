use proconio::input;
use ndarray::prelude::*;

fn main() {
  input! {
    n: usize,
  }
  println!("{}", solve(n));
}

// see: 3_4_blocks.jpg
fn solve(n: usize) -> usize {
  let a = array![
    [2, 1, 1, 0],
    [1, 2, 0, 1],
    [1, 0, 2, 1],
    [0, 1, 1, 2],
  ];
  pow(&a, n - 1).dot(&array![2, 1, 1, 0])[0]
}

// 繰り返し二乗法
fn pow(a: &Array2<usize>, n: usize) -> Array2<usize> {
  let mut n = n;
  let mut a = a.clone();
  let mut b = ArrayBase::eye(a.ncols());
  while n > 0 {
    if n & 1 == 1 {
      b = b.dot(&a);
    }
    a = a.dot(&a);
    n >>= 1;
  }
  b
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 2;
    assert_eq!(solve(n), 6);
  }
}