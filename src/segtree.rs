use itertools::Itertools;
use num::Bounded;
use std::cmp::min;

// Segment tree による Range Minimum Query (RMQ) (3-3 節)
// s, t が与えられたとき、[s, t] の最小値を求める O(log n)
// i, x が与えられたとき、[i] = x に変更する O(log n)

#[derive(Debug)]
pub struct SegTree<T>(Vec<T>);

impl<T: Ord + Clone + Bounded> SegTree<T> {
  pub fn rmq(vec: Vec<T>) -> Self {
    fn go<T: Ord + Clone + Bounded>(vec: Vec<T>, acc: &mut Vec<Vec<T>>) {
      if vec.len() <= 1 {
        acc.push(vec);
      } else {
        acc.push(vec.clone());
        let mut new_vec = vec![];
        for (l, r) in vec.iter().tuples() {
          new_vec.push(min(l, r).clone());
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
    let mut n = 1;
    while vec.len() > n {
      n = n << 1;
    }
    let pad = n - vec.len();
    let vec = vec![vec, vec![T::max_value(); pad]].concat();
    let mut acc = vec![];
    go(vec, &mut acc);
    acc.reverse();
    SegTree(acc.concat())
  }

  // [s, t) の最小値を求める
  // 見つからない場合は MaxBound を返す
  pub fn query(&self, s: usize, t: usize) -> T {
    // k は接点の番号であり、 [l, r) に対応する
    fn go<T: Ord + Clone + Bounded>(
      v: &Vec<T>,
      s: usize,
      t: usize,
      k: usize,
      l: usize,
      r: usize,
    ) -> T {
      // [s, t), [l, r) に共通部分がない場合は終了
      if r <= s || t <= l {
        T::max_value()
        // [s, t) が [l, r) を包摂する場合はノードの値を返す
      } else if s <= l && r <= t {
        v[k].clone()
      } else {
        let vl = go(v, s, t, k * 2 + 1, l, (l + r) / 2);
        let vr = go(v, s, t, k * 2 + 2, (l + r) / 2, r);
        min(vl, vr)
      }
    }
    go(&self.0, s, t, 0, 0, (self.0.len() + 1) / 2)
  }

  pub fn update(&mut self, i: usize, x: T) {
    // 葉の接点
    let mut k = i + (self.0.len() + 1) / 2 - 1;
    self.0[k] = x;
    // 登りながら更新
    while k > 0 {
      k = (k - 1) / 2;
      self.0[k] = min(self.0[k * 2 + 1].clone(), self.0[k * 2 + 2].clone());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let st = SegTree::rmq(v);
    assert_eq!(st.query(0, 7), 1);
  }
  #[test]
  fn sample_update_query() {
    let v = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut st = SegTree::rmq(v);
    st.update(0, 2);
    assert_eq!(st.query(0, 5), 2);
  }
  #[test]
  fn sample_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let st = SegTree::rmq(v);
    assert_eq!(st.query(0, 6), 3);
  }
  #[test]
  fn sample_update_query_odd() {
    let v = vec![5, 3, 7, 9, 6, 4, 1];
    let mut st = SegTree::rmq(v);
    st.update(0, 2);
    assert_eq!(st.query(0, 5), 2);
  }
}
