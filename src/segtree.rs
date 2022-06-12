use num::{Bounded, Zero};

// Segment tree による Range Minimum Query (Min) (3-3 節)
// s, t が与えられたとき、[s, t] の最小値を求める O(log n)
// i, x が与えられたとき、[i] = x に変更する O(log n)

// 一般にはモノイド （X, op, e) 上の構造を持つ
pub trait Monoid {
  type X;
  fn op(a: &Self::X, b: &Self::X) -> Self::X;
  fn e() -> Self::X;
}

#[derive(Debug)]
pub struct SegTree<T: Monoid> {
  nodes: Vec<T::X>,
  n: usize, 
  leaves: usize
}

impl<T: Monoid> SegTree<T>
where
  T::X: Clone,
{
  pub fn new(v: Vec<T::X>) -> Self {
    let len = v.len();
    // n == x^2 かつ x は len < x^2 を満たす最小の x
    let mut n = 1;
    while len > n {
      n = n << 1;
    }
    // 2冪にする
    let mut vec = vec![vec![T::e(); n - 1], v, vec![T::e(); n - len]].concat();
    for k in (0..=n - 2).rev() {
      vec[k] = T::op(&vec[k * 2 + 1], &vec[k * 2 + 2]);
    }
    SegTree {
      nodes: vec,
      n, 
      leaves: len
    }
  }

  // [s, t) の最小値を求める
  // 見つからない場合は X::e() を返す
  pub fn query(&self, s: usize, t: usize) -> T::X {
    // k は接点の番号であり、 [l, r) に対応する
    self.query_rec(s, t, 0, 0, self.n)
  }

  fn query_rec(&self, s: usize, t: usize, k: usize, l: usize, r: usize) -> T::X {
    if r <= s || t <= l { // [s, t), [l, r) に共通部分がない場合は終了
      T::e()
    } else if s <= l && r <= t { // [s, t) が [l, r) を包摂する場合はノードの値を返す
      self.nodes[k].clone()
    } else {
      let vl = self.query_rec(s, t, k * 2 + 1, l, (l + r) / 2);
      let vr = self.query_rec(s, t, k * 2 + 2, (l + r) / 2, r);
      T::op(&vl, &vr)
    }
  }

  pub fn update(&mut self, i: usize, x: T::X) {
    self.update_by(i, |_| x.clone())
  }

  pub fn update_by<F>(&mut self, i: usize, f: F)
  where
    F: FnOnce(&T::X) -> T::X,
  {
    // 葉の接点
    let mut k = i + self.n - 1;
    self.nodes[k] = f(&self.nodes[k]);
    // 登りながら更新
    while k > 0 {
      k = (k - 1) / 2;
      self.nodes[k] = T::op(&self.nodes[k * 2 + 1], &self.nodes[k * 2 + 2]);
    }
  }

  pub fn size(&self) -> usize {
    self.leaves
  }
}

// 具体的なモノイド

// Range Sum Query + Point Add Query
// (T, +, 0)
#[derive(Debug)]
pub struct RSQ<T>(T);
impl<T> Monoid for RSQ<T>
where
  T: Zero + Clone, // Zero は Add 上に定義されている
{
  type X = T;
  fn op(a: &Self::X, b: &Self::X) -> Self::X {
    a.clone() + b.clone()
  }
  fn e() -> Self::X {
    T::zero()
  }
}

// Range Minimum Query + Point Update Query
// (M∪{∞}, min, ∞)
#[derive(Debug)]
pub struct RMQ<T>(T);
impl<T> Monoid for RMQ<T>
where
  T: Ord + Bounded + Clone,
{
  type X = T;
  fn op(a: &Self::X, b: &Self::X) -> Self::X {
    a.min(b).clone()
  }
  fn e() -> Self::X {
    T::max_value()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let st = SegTree::<RMQ<usize>>::new(v);
    assert_eq!(st.query(0, 7), 1);
  }
  #[test]
  fn sample_update_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut st = SegTree::<RMQ<usize>>::new(v);
    st.update(0, 2);
    assert_eq!(st.query(0, 5), 2);
  }
  #[test]
  fn sample_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let st = SegTree::<RMQ<usize>>::new(v);
    assert_eq!(st.query(0, 6), 3);
  }
  #[test]
  fn sample_update_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let mut st = SegTree::<RMQ<usize>>::new(v);
    st.update(0, 2);
    assert_eq!(st.query(0, 5), 2);
  }
}
