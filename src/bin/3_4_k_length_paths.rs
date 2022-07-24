use proconio::input;
use ndarray::prelude::*;

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [[usize; n]; n],
  }
  println!("{}", solve(n, k, a));
}

fn solve(_n: usize, k: usize, a: Vec<Vec<usize>>) -> usize {
  let a = vec_to_array(a);
  pow(&a, k).sum()
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
    let n = 4;
    let k = 2;
    let a = vec![
      vec![0, 1, 1, 0],
      vec![0, 0, 1, 0],
      vec![0, 0, 0, 1],
      vec![1, 0, 0, 0],
      ];
    assert_eq!(solve(n, k, a), 6);
  }
}