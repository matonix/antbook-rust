use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    l: [usize; n]
  }
  println!("{}", solve(n, l));
}

fn solve(_n: usize, l: Vec<usize>) -> usize {
  // 小さい順にカタマリをつくる
  let mut cost = 0;
  let mut l = BinaryHeap::from(l.into_iter().map(|x| Reverse(x)).collect_vec()); // min-heap なので Reverse で包む
  while l.len() >= 2 {
    // 先頭2つを取り出す
    let a = l.pop().unwrap();
    let b = l.pop().unwrap();
    let c = a.0 + b.0; // Reverse なので
    // 作った板を優先度付きキューに挿入
    l.push(Reverse(c));
    cost += c;
  }
  cost
} 

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let l = vec![8, 5, 8];
    assert_eq!(solve(n, l), 34);
  }
}
