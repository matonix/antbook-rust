use num::Zero;
use std::ops::AddAssign;

// BIT
// SegTree の特殊系（モノイド M(Z,+,0) を与えることに等しい）
// a_i = 0 で初期化
// sum: i に対して a_1 + ... + a_i を計算 O(log n)
// add: i, x に対して a_i += x する O(log n)
// 1-indexed

#[derive(Debug)]
pub struct BIT<T>(Vec<T>);

impl<T> BIT<T>
where
  T: Clone + Zero + AddAssign,
{
  pub fn new(n: usize) -> Self {
    BIT(vec![T::zero(); n + 1])
  }

  pub fn from(v: Vec<T>) -> Self {
    let mut bit = Self::new(v.len());
    for (i, x) in v.iter().enumerate() {
      bit.add(i + 1, x.clone());
    }
    bit
  }

  pub fn size(&self) -> usize {
    self.0.len() - 1
  }

  pub fn sum(&self, i: usize) -> T {
    if i == 0 {
      return T::zero();
    }
    let mut s = T::zero();
    let mut i = i as isize;
    while i > 0 {
      s += self.0[i as usize].clone();
      i -= i & -i; // i & -i で i ビット表現で最後に現れる 1 だけを残した値を得る
    }
    s
  }

  pub fn add(&mut self, i: usize, x: T) {
    if i == 0 {
      panic!("i should be >= 1.");
    }
    let mut i = i as isize;
    while (i as usize) <= self.size() {
      self.0[i as usize] += x.clone();
      i += i & -i;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let bit = BIT::<usize>::from(v);
    assert_eq!(bit.sum(7), 35);
  }
  #[test]
  fn sample_update_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut bit = BIT::<usize>::from(v);
    bit.add(1, 5);
    assert_eq!(bit.sum(7), 40);
  }
  #[test]
  fn sample_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let bit = BIT::<usize>::from(v);
    assert_eq!(bit.sum(7), 35);
  }
  #[test]
  fn sample_update_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let mut bit = BIT::<usize>::from(v);
    bit.add(1, 5);
    assert_eq!(bit.sum(7), 40);
  }
}
