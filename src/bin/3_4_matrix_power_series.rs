use ndarray::{prelude::*, stack};
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    m: usize,
    a: [[usize; n]; n]
  }
  let a = vec_to_array(a);
  println!("{}", solve(n, k, m, a));
}

fn solve(n: usize, k: usize, _m: usize, a: Array2<usize>) -> Array2<usize> {
  let z: Array2<usize> = Array2::zeros((n, n));
  let i: Array2<usize> = ArrayBase::eye(a.nrows());
  let big = stack![Axis(0), 
    stack![Axis(1), a, z], 
    stack![Axis(1), i, i]
    ];
  pow(&big, k + 1).slice_move(s![n.., 0..n]) - i
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

// https://github.com/rust-ndarray/ndarray/issues/590
fn vec_to_array<T: Clone>(v: Vec<Vec<T>>) -> Array2<T> {
  if v.is_empty() {
    return Array2::from_shape_vec((0, 0), Vec::new()).unwrap();
  }
  let nrows = v.len();
  let ncols = v[0].len();
  let mut data = Vec::with_capacity(nrows * ncols);
  for row in &v {
    assert_eq!(row.len(), ncols);
    data.extend_from_slice(&row);
  }
  Array2::from_shape_vec((nrows, ncols), data).unwrap()
}

#[cfg(test)]
mod tests {
  use super::solve;
  use ndarray::prelude::*;

  #[test]
  fn example1() {
    let n = 2;
    let k = 2;
    let m = 4;
    let a = array![[0, 1], [1, 1]];
    assert_eq!(solve(n, k, m, a), array![[1, 2], [2, 3]]);
  }
}
