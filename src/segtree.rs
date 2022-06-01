use itertools::Itertools;
use num::{Bounded, Zero};
use std::cmp::min;

// Segment tree による Range Minimum Query (Min) (3-3 節)
// s, t が与えられたとき、[s, t] の最小値を求める O(log n)
// i, x が与えられたとき、[i] = x に変更する O(log n)

// 一般にはモノイド （op, e) 上の構造を持つ
pub trait Monoid {
  fn op(a: Self, b: Self) -> Self;
  fn e() -> Self;
}

#[derive(Debug)]
pub struct SegTree<T>(Vec<T>, usize);

impl<T> Monoid for T
where
  T: Zero,
{
  fn op(a: Self, b: Self) -> Self {
    a.add(b)
  }
  fn e() -> Self {
    T::zero()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Min<T>(T);

impl<T> Monoid for Min<T>
where
  T: Ord + Bounded + Clone,
{
  fn op(a: Self, b: Self) -> Self {
    Self(min(a.0, b.0))
  }
  fn e() -> Self {
    Self(T::max_value())
  }
}

impl<T> Min<T>
where
  T: Ord + Bounded + Clone,
{
  pub fn rmq(vec: Vec<T>) -> SegTree<Min<T>> {
    let vec = vec.into_iter().map(|v| Min(v)).collect_vec();
    SegTree::new(vec)
  }
}

impl<T> SegTree<T>
where
  T: Clone + Monoid,
{
  pub fn new(vec: Vec<T>) -> Self {
    fn go<T>(vec: Vec<T>, acc: &mut Vec<Vec<T>>)
    where
      T: Clone + Monoid,
    {
      if vec.len() <= 1 {
        acc.push(vec);
      } else {
        acc.push(vec.clone());
        let mut new_vec = vec![];
        for (l, r) in vec.iter().tuples() {
          new_vec.push(T::op(l.clone(), r.clone()));
        }
        if vec.len() % 2 != 0 {
          if let Some(last) = vec.last() {
            new_vec.push(last.clone());
          }
        }
        go(new_vec, acc);
      }
    }
    // padding to 2^n
    let len = vec.len();
    let mut n = 1;
    while len > n {
      n = n << 1;
    }
    let pad = n - len;
    let vec = vec![vec, vec![T::e(); pad]].concat();
    let mut acc = vec![];
    go(vec, &mut acc);
    acc.reverse();
    SegTree(acc.concat(), len)
  }

  // [s, t) の最小値を求める
  // 見つからない場合は MaxBound を返す
  pub fn query(&self, s: usize, t: usize) -> T {
    // k は接点の番号であり、 [l, r) に対応する
    fn go<T>(st: &SegTree<T>, s: usize, t: usize, k: usize, l: usize, r: usize) -> T
    where
      T: Clone + Monoid,
    {
      // [s, t), [l, r) に共通部分がない場合は終了
      if r <= s || t <= l {
        T::e()
        // [s, t) が [l, r) を包摂する場合はノードの値を返す
      } else if s <= l && r <= t {
        st.0[k].clone()
      } else {
        let vl = go(st, s, t, k * 2 + 1, l, (l + r) / 2);
        let vr = go(st, s, t, k * 2 + 2, (l + r) / 2, r);
        T::op(vl, vr)
      }
    }
    go(&self, s, t, 0, 0, (self.0.len() + 1) / 2)
  }

  pub fn update(&mut self, i: usize, x: T) {
    // 葉の接点
    let mut k = i + (self.0.len() + 1) / 2 - 1;
    self.0[k] = x;
    // 登りながら更新
    while k > 0 {
      k = (k - 1) / 2;
      self.0[k] = T::op(self.0[k * 2 + 1].clone(), self.0[k * 2 + 2].clone());
    }
  }

  pub fn size(&self) -> usize {
    self.1
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
