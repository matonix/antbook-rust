use std::mem::swap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [Chars; n],
  }
  // println!("{}", G::_solve(n, m, a));
  println!("{}", solve(n, m, a));
}

// https://qiita.com/qnighy/items/46dbf8d2aff7c2531f4e#%E4%BB%A3%E6%9B%BF%E7%AD%96-%E7%AB%B6%E3%83%97%E3%83%AD%E4%BB%A5%E5%A4%96
struct _G {
  n: usize,
  m: usize,
  color: Vec<Vec<bool>>,
  used: Vec<Vec<bool>>,
}

fn solve(n: usize, m: usize, a: Vec<Vec<char>>) -> usize {
  let mut crt = vec![0; 1 << m];
  let mut next = vec![0; 1 << m];
  crt[0] = 1;
  for i in (0..n).rev() {
    for j in (0..m).rev() {
      for used in 0..1 << m {
        if (used >> j & 1 == 1) || a[i][j] == 'x' {
          next[used] = crt[used & !(1 << j)]; // j番目の used を false に
        } else {
          let mut res = 0;
          for (k, l) in [(0, 1), (1, 0)] {
            if i + k < n && j + l < m && (used >> j + l & 1 != 1) && a[i + k][j + l] != 'x'  {
              res += crt[used | 1 << j + l];
            }
          }
          next[used] = res;
        }
      }
      swap(&mut crt, &mut next);
    }
  }
  crt[0]
}

impl _G {

  fn _solve(n: usize, m: usize, a: Vec<Vec<char>>) -> usize {
    let used = vec![vec![false; m]; n];
    let color = a.into_iter().map(|v| v.into_iter().map(|c| c == 'x').collect_vec()).collect_vec(); 
    Self{ n, m, color, used }._go(0, 0)
  }

  // 愚直
  fn _go(&mut self, i: usize, j: usize) -> usize {
    // dbg!(i, j);
    // println!("{}", _disp(&self.used));
    let n = self.n;
    let m = self.m;
    if j == m {
      return self._go(i + 1, 0);
    }
    if i == n {
      return 1;
    }
    if self.used[i][j] || self.color[i][j] {
      return self._go(i, j + 1);
    }
    let mut res = 0;
    self.used[i][j] = true;
    for (k, l) in [(0, 1), (1, 0)] {
      if i + k < n && j + l < m && !self.used[i + k][j + l] && !self.color[i + k][j + l] {
        self.used[i + k][j + l] = true;
        res += self._go(i, j + 1); // 再帰から戻った時に探索で一時的に true にした盤面を false に戻す（バックトラック）
        self.used[i + k][j + l] = false;
      }
    }
    self.used[i][j] = false;
    res
  }
}

fn _disp(a: &Vec<Vec<bool>>) -> String {
  a.into_iter().map(|s| s.into_iter().map(|b| if *b { 'x' } else { '.' }).collect::<String>()).join("\n")
}

#[cfg(test)]
mod tests {
  use super::solve;
  // use super::G;
  use proconio::{source::auto::AutoSource, input, marker::Chars};

  #[test]
  fn example1() {
    let n = 3;
    let m = 3;
    let source = AutoSource::from(
      r"
      ...
      .x.
      ...
      ",
    );
    input! {
      from source,
      mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
    }
    assert_eq!(solve(n, m, a), 2);
    // assert_eq!(G::_solve(n, m, a), 2);
  }
}