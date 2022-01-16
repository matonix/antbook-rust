use proconio::input;
use antbook::BinarySearch;
use std::collections::VecDeque;

fn main() {
  input! {
    n: usize,
    mut l: [usize; n]
  }
  println!("{}", solve(n, l));
}

fn solve(_n: usize, mut l: Vec<usize>) -> usize {
  // 小さい順にカタマリをつくる
  let mut cost = 0;
  l.sort();
  let mut l = VecDeque::from(l);
  while l.len() > 1 {
    // 先頭2つを取り出す
    let a = l.pop_front().unwrap();
    let b = l.pop_front().unwrap();
    let c = a + b;
    // 作った板の挿入位置を探して、挿入する 
    // (make_contiguous は VecDeque の内部表現を整理してスライスとして扱えるようにするおまじない)
    let i = l.make_contiguous().lower_bound(&c);
    l.insert(i, c);
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
