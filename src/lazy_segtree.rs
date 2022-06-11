use itertools::Itertools;
use num::{Bounded, Zero};
use std::cmp::min;

// Lazy Segment tree による Range Update Query
// s, t が与えられたとき、[s, t] の最小値を求める O(log n)
// s, t, x が与えられたとき、[i, t] = x に変更する O(log n)

pub trait Monoid {
  fn op(a: &Self, b: &Self) -> Self;
  fn e() -> Self;
}

// nodes, lazy, #power2leaves, #leaves
#[derive(Debug)]
pub struct LazySegTree<X> {
  nodes: Vec<X>, 
  lazys: Vec<X>, 
  n: usize, 
  leaves: usize
}

impl<X> LazySegTree<X>
where
  X: Clone + Monoid + PartialEq,
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
    let size = vec.len();
    LazySegTree {
      nodes: vec,
      lazys: vec![X::e(); size], 
      n, 
      leaves: len
    }
  }

  // [s, t) の最小値を求める
  // 見つからない場合は X::e() を返す
  pub fn query(&mut self, s: usize, t: usize) -> X {
    // k は接点の番号であり、 [l, r) に対応する
    self.query_rec(s, t, 0, 0, self.n)
  }

  fn query_rec(&mut self, s: usize, t: usize, k: usize, l: usize, r: usize) -> X {
    self.eval(k);
    if r <= s || t <= l { // [s, t), [l, r) に共通部分がない場合は終了
      X::e()
    } else if s <= l && r <= t { // [s, t) が [l, r) を包摂する場合はノードの値を返す
      self.nodes[k].clone()
    } else {
      let vl = self.query_rec(s, t, k * 2 + 1, l, (l + r) / 2);
      let vr = self.query_rec(s, t, k * 2 + 2, (l + r) / 2, r);
      X::op(&vl, &vr)
    }
  }

  pub fn update(&mut self, s: usize, t: usize, x: X) {
    self.update_by(s, t, &mut |_| x.clone())
  }

  pub fn update_by<F>(&mut self, s: usize, t: usize, f: &mut F)
  where
    F: FnMut(&X) -> X,
  {
    self.update_rec(s, t, f, 0, 0, self.n)
  }

  fn update_rec<F>(&mut self, s: usize, t: usize, f: &mut F, k: usize, l: usize, r: usize) 
  where
    F: FnMut(&X) -> X,
  {
    self.eval(k);
    if s <= l && r <= t { // [s, t) が [l, r) を包摂する場合
      self.lazys[k] = f(&self.lazys[k]);
      self.eval(k);
    } else if s < r && l < t { // [s, t), [l, r) に一部共通部分がある場合
      self.update_rec(s, t, f, k * 2 + 1, l, (l + r) / 2);
      self.update_rec(s, t, f, k * 2 + 2, (l + r) / 2, r);
      self.nodes[k] = X::op(&self.nodes[k * 2 + 1], &self.nodes[k * 2 + 2]);
    }
  }

  fn eval(&mut self, k: usize) {
    if self.lazys[k] == X::e() {
      return
    } else if k < self.n - 1 {
      self.lazys[k * 2 + 1] = self.lazys[k].clone();
      self.lazys[k * 2 + 2] = self.lazys[k].clone();
    }
    self.nodes[k] = self.lazys[k].clone();
    self.lazys[k] = X::e();
  }

  pub fn size(&self) -> usize {
    self.leaves
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
  pub fn rmq(vec: Vec<X>) -> LazySegTree<Min<X>> {
    let vec = vec.into_iter().map(|v| Min(v)).collect_vec();
    LazySegTree::new(vec)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut st = Min::<usize>::rmq(v);
    assert_eq!(st.query(0, 7).0, 1);
  }
  #[test]
  fn sample_update_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut st = Min::<usize>::rmq(v);
    st.update(0, 1, Min(2));
    assert_eq!(st.query(0, 5).0, 2);
  }
  #[test]
  fn sample_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let mut st = Min::<usize>::rmq(v);
    assert_eq!(st.query(0, 6).0, 3);
  }
  #[test]
  fn sample_update_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let mut st = Min::<usize>::rmq(v);
    st.update(0, 1, Min(2));
    assert_eq!(st.query(0, 5).0, 2);
  }
}
