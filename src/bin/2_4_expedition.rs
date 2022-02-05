use proconio::input;
use std::collections::BinaryHeap;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    l: usize,
    p: usize,
    a: [usize; n],
    b: [usize; n],
  }
  println!("{}", solve(n, l, p, a, b));
}

// プライオリティキュー
// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
// デフォルトは max-heap
// min-heap の使い方は: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap

fn solve(n: usize, l: usize, p: usize, a: Vec<usize>, b: Vec<usize>) -> isize {
  let mut remain = p;
  let mut pq = BinaryHeap::new();
  let dists = vec![vec![0], a, vec![l]].concat().iter().tuple_windows().map(|(s, t)| t - s).collect_vec();
  for (dist, gas) in itertools::zip(dists, b) {
    pq.push(gas);
    while remain < dist { // 足りない
      if let Some(ene) = pq.pop() { // 補給する
        remain += ene;
      } else { // 補給不可
        return -1;
      }
    } 
    remain -= dist // 消費
  }
  // 訪問数 = スタンドの数 - 未訪問のスタンドの数
  (n - pq.len()).try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let l = 25;
    let p = 10;
    let a = vec![10, 14, 20, 21];
    let b = vec![10, 5, 2, 4];
    assert_eq!(solve(n, l, p, a, b), 2);
  }
}
