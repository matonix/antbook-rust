use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    wvs: [(usize, usize); n],
    w: usize,
  }
  println!("{}", solve(n, wvs, w));
}

// 価値の総和の最大値 f(n, w+1)
// f(i+1, j) = | max(f(i, j - w[i+1]) + v[i+1], f(i, j)) if j >= w[i+1] ※1-indexed
//             | f(i, j)                                 otherwise

// ベクタの使い方
// dp[0] は 0 で初期化
// 0..n 番目のアイテムは 1..n+1 番目で計算する。
// dp[i][j] の範囲は 0..=w_max で、回答は dp[n][w_max] を参照する。

fn solve(n: usize, wvs: Vec<(usize, usize)>, w_max: usize) -> usize {
  let mut dp = vec![vec![0; w_max+1]; n+1];
  for (i, &(w, v)) in wvs.iter().enumerate() {
    for j in 0..=w_max {
      dp[i+1][j] = if j >= w { max(dp[i][j - w] + v, dp[i][j]) } else { dp[i][j] };
    }
  }
  dp[n][w_max]
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let wvs = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let w = 5;
    assert_eq!(solve(n, wvs, w), 7);
  }
}
