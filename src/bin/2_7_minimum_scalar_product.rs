use proconio::input;
use itertools::Itertools;
use std::collections::VecDeque;
use std::cmp::Reverse;

fn main() {
  input! {
    n: usize,
    u: [i64; n],
    v: [i64; n],
  }
  dbg!(solve(n, u, v));
}

// 解答より：そもそも第二象限VS大四象限で比較しなくても、ペアが成立してるなら単にSumを取ってよい
fn solve(_n: usize, u: Vec<i64>, v: Vec<i64>) -> i64 {
  let mut u = u;
  let mut v = v;
  u.sort();
  v.sort_by_key(|x| Reverse(*x));
  u.iter().zip(v).map(|(x, y)| x * y).sum()
}

// z = xy https://ja.wolframalpha.com/input/?i=z%3Dxy を参考に
// 積が小さいペア（第二象限VS大四象限）から順に採用
fn _solve(_n: usize, u: Vec<i64>, v: Vec<i64>) -> i64 {
  let mut u = u;
  let mut v = v;
  u.sort();
  v.sort();
  let mut u = VecDeque::from(u);
  let mut v = VecDeque::from(v);
  let mut ans = 0;
  while !u.is_empty() && !v.is_empty() {
    let uf = u.front().unwrap().clone();
    let ub = u.back().unwrap().clone();
    let vf = v.front().unwrap().clone();
    let vb = v.back().unwrap().clone();
    if uf * vb < ub * vf {
      u.pop_front();
      v.pop_back();
      ans += uf * vb;
    } else {
      u.pop_back();
      v.pop_front();
      ans += ub * vf;
    }
  }
  ans
}

// 網羅的 O(N! N)
fn __solve(n: usize, u: Vec<i64>, v: Vec<i64>) -> i64 {
  v.iter().permutations(n).map(|w| w.iter().zip(&u).map(|(&x, y)| x * y).sum()).min().unwrap()
}

#[cfg(test)]
mod tests {
  use super::solve;
  #[test]
  fn example1() {
    let n = 3;
    let u = vec![1, 3, -5];
    let v = vec![-2, 4, 1];
    assert_eq!(solve(n, u, v), -25)
  }
  #[test]
  fn example2() {
    let n = 5;
    let u = vec![1, 2, 3, 4, 5];
    let v = vec![1, 0, 1, 0, 1];
    assert_eq!(solve(n, u, v), 6)
  }
}
