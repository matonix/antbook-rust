use proconio::input;
use itertools;

fn main() {
  input! {
    n: usize,
    s: [usize; n],
    t: [usize; n]
  }
  println!("{}", solve(n, s, t));
}

// 貪欲法の証明について
// https://penguinshunya.hatenablog.com/entry/2020/01/21/093846
// https://scrapbox.io/nishio/%E8%B2%AA%E6%AC%B2%E6%B3%95%E3%81%AE%E8%A8%BC%E6%98%8E%E3%83%91%E3%82%BF%E3%83%BC%E3%83%B3

fn solve(_n: usize, s: Vec<usize>, t: Vec<usize>) -> usize {
  // 終了時間の早い順
  let mut r: Vec<(usize, usize)> = itertools::zip(s, t).collect();
  r.sort_by_key(|rng| rng.1);
  let mut cnt = 0;
  let mut time = 0;
  for rng in r {
    if rng.0 > time {
      cnt += 1;
      time = rng.1;
    }
  }
  cnt
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 5;
    let s = vec![1, 2, 4, 6, 8];
    let t = vec![3, 5, 7, 9, 10];
    assert_eq!(solve(n, s, t), 3);
  }
}
