use std::ops::{Index, IndexMut};

use itertools::Itertools;

pub type Point = (usize, usize);

#[derive(Debug)]
pub struct Walk2D<T> {
  n: usize,
  m: usize,
  arr: Vec<Vec<T>>, // (n, m)-matrix.
}

impl<T> Walk2D<T> {
  pub fn new(n: usize, m: usize, arr: Vec<Vec<T>>) -> Walk2D<T> {
    Walk2D { n, m, arr }
  }

  pub fn find<P>(&self, predicate: P) -> Option<Point>
  where
    P: FnMut(&T) -> bool,
    T: Clone,
  {
    self
      .arr
      .concat()
      .iter()
      .position(predicate)
      .map(|idx| (idx / self.m, idx % self.m))
  }

  pub fn find_all<P>(&self, predicate: P) -> Vec<Point>
  where
    P: FnMut(&T) -> bool,
    T: Clone,
  {
    self
      .arr
      .concat()
      .iter()
      .positions(predicate)
      .map(|idx| (idx / self.m, idx % self.m))
      .collect_vec()
  }

  pub fn next_4way_p(&self, (i, j): Point) -> Vec<Point> {
    self.next_4way(i, j)
  }

  pub fn next_4way(&self, i: usize, j: usize) -> Vec<Point> {
    self.filter_in_bound(vec![
      (i.wrapping_sub(1), j),
      (i.wrapping_add(1), j),
      (i, j.wrapping_sub(1)),
      (i, j.wrapping_add(1)),
    ])
  }

  pub fn next_8way_p(&self, (i, j): Point) -> Vec<Point> {
    self.next_8way(i, j)
  }

  pub fn next_8way(&self, i: usize, j: usize) -> Vec<Point> {
    self.filter_in_bound(vec![
      (i.wrapping_sub(1), j.wrapping_sub(1)),
      (i.wrapping_sub(1), j),
      (i.wrapping_sub(1), j.wrapping_add(1)),
      (i, j.wrapping_sub(1)),
      (i, j.wrapping_add(1)),
      (i.wrapping_add(1), j.wrapping_sub(1)),
      (i.wrapping_add(1), j),
      (i.wrapping_add(1), j.wrapping_add(1)),
    ])
  }

  fn filter_in_bound(&self, points: Vec<Point>) -> Vec<Point> {
    points
      .into_iter()
      .filter(|(i, j)| *i < self.n && *j < self.m)
      .collect()
  }

  pub fn clone_as_vec(&self) -> Vec<Vec<T>>
  where
    T: Clone,
  {
    self.arr.clone()
  }
}

// array と同じ操作感にする
impl<T> Index<usize> for Walk2D<T> {
  type Output = Vec<T>;

  fn index(&self, i: usize) -> &Self::Output {
    &self.arr[i]
  }
}

impl<T> IndexMut<usize> for Walk2D<T> {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.arr[i]
  }
}

// Point でのアクセス
impl<T> Index<Point> for Walk2D<T> {
  type Output = T;

  fn index(&self, (i, j): Point) -> &Self::Output {
    &self.arr[i][j]
  }
}

impl<T> IndexMut<Point> for Walk2D<T> {
  fn index_mut(&mut self, (i, j): Point) -> &mut Self::Output {
    &mut self.arr[i][j]
  }
}

// もうちょっと考えてやる
// 参考: https://ordovicia.github.io/post/implementingvecinrust/into_iter/

// impl<T: Clone> IntoIterator for Walk2D<T> {
//   type Item = T;
//   type IntoIter = std::vec::IntoIter<Self::Item>;

//   fn into_iter(self) -> Self::IntoIter {
//     self.arr.concat().into_iter()
//   }
// }
