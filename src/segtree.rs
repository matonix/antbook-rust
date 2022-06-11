use itertools::Itertools;
use num::{Bounded, Zero};
use std::cmp::min;

// Segment tree による Range Minimum Query (Min) (3-3 節)
// s, t が与えられたとき、[s, t] の最小値を求める O(log n)
// i, x が与えられたとき、[i] = x に変更する O(log n)

// 一般にはモノイド （X, op, e) 上の構造を持つ
pub trait Monoid {
  fn op(a: &Self, b: &Self) -> Self;
  fn e() -> Self;
}

#[derive(Debug)]
pub struct SegTree<X>(Vec<X>, usize);

impl<X> SegTree<X>
where
  X: Clone + Monoid,
{
  pub fn new(v: Vec<X>) -> Self {
    let len = v.len();
    // n == x^2 かつ x は len < x^2 を満たす最小の x
    let mut n = 1;
    while len > n {
      n = n << 1;
    }
    // 2冪にする
    let mut vec = vec![vec![X::e(); n - 1], v, vec![X::e(); n - len]].concat();
    for k in (0..=len - 2).rev() {
      vec[k] = X::op(&vec[k * 2 + 1], &vec[k * 2 + 2]);
    }
    SegTree(vec, len)
  }

  // [s, t) の最小値を求める
  // 見つからない場合は X::e() を返す
  pub fn query(&self, s: usize, t: usize) -> X {
    // k は接点の番号であり、 [l, r) に対応する
    fn go<X>(st: &SegTree<X>, s: usize, t: usize, k: usize, l: usize, r: usize) -> X
    where
      X: Clone + Monoid,
    {
      // [s, t), [l, r) に共通部分がない場合は終了
      if r <= s || t <= l {
        X::e()
        // [s, t) が [l, r) を包摂する場合はノードの値を返す
      } else if s <= l && r <= t {
        st.0[k].clone()
      } else {
        let vl = go(st, s, t, k * 2 + 1, l, (l + r) / 2);
        let vr = go(st, s, t, k * 2 + 2, (l + r) / 2, r);
        X::op(&vl, &vr)
      }
    }
    go(&self, s, t, 0, 0, (self.0.len() + 1) / 2)
  }

  pub fn update(&mut self, i: usize, x: X) {
    self.update_by(i, |_| x.clone())
  }

  pub fn update_by<F>(&mut self, i: usize, mut f: F)
  where
    F: FnMut(&X) -> X,
  {
    // 葉の接点
    let mut k = i + (self.0.len() + 1) / 2 - 1;
    self.0[k] = f(&self.0[k]);
    // 登りながら更新
    while k > 0 {
      k = (k - 1) / 2;
      self.0[k] = X::op(&self.0[k * 2 + 1], &self.0[k * 2 + 2]);
    }
  }

  pub fn size(&self) -> usize {
    self.1
  }
}

// 具体的なモノイド

// (X, +, 0)
// プリミティブ型向け
impl<X> Monoid for X
where
  X: Zero + Copy, // Zero は Add 上に定義されている
{
  fn op(a: &Self, b: &Self) -> Self {
    a.add(*b)
  }
  fn e() -> Self {
    X::zero()
  }
}

// Range Minimum Query
// (M∪{∞}, min, ∞)
#[derive(Debug, Clone, PartialEq)]
pub struct Min<X>(X);

impl<X> Monoid for Min<X>
where
  X: Ord + Bounded + Clone,
{
  fn op(a: &Self, b: &Self) -> Self {
    Self(min(a.0.clone(), b.0.clone()))
  }
  fn e() -> Self {
    Self(X::max_value())
  }
}

// RMQ 向け生成関数
impl<X> Min<X>
where
  X: Ord + Bounded + Clone,
{
  pub fn rmq(vec: Vec<X>) -> SegTree<Min<X>> {
    let vec = vec.into_iter().map(|v| Min(v)).collect_vec();
    SegTree::new(vec)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let st = Min::<usize>::rmq(v);
    assert_eq!(st.query(0, 7).0, 1);
  }
  #[test]
  fn sample_update_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut st = Min::<usize>::rmq(v);
    st.update(0, Min(2));
    assert_eq!(st.query(0, 5).0, 2);
  }
  #[test]
  fn sample_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let st = Min::<usize>::rmq(v);
    assert_eq!(st.query(0, 6).0, 3);
  }
  #[test]
  fn sample_update_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let mut st = Min::<usize>::rmq(v);
    st.update(0, Min(2));
    assert_eq!(st.query(0, 5).0, 2);
  }
}
