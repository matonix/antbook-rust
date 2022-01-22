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

// 各アイテムごとに・・・の i の部分で工夫が居る
// 0, 1, 2, ..., j / w[i+1] 個入れる場合、という場合分け（というかその中の最大値）。

// 価値の総和の最大値 f(n, w+1)
// f(i+1, j) = max_k (f(i, j - k * w[i+1]) + k * v[i+1]) for k in 0..j / w[i+1]


// O(nw^2)
fn solve_(n: usize, wvs: Vec<(usize, usize)>, w_max: usize) -> usize {
  let mut dp = vec![vec![0; w_max+1]; n+1];
  for (i, &(w, v)) in wvs.iter().enumerate() { // O(n)
    for j in 0..=w_max { // O(w)
      for k in 0..=j / w { // O(w)
        dp[i+1][j] = max(dp[i+1][j], dp[i][j - k * w] + k * v)
      }
    }
  }
  dp[n][w_max]
}

// 考察: 1 ~ j / w[i+1] 個同じアイテムを入れる場合、 f(i+1, j - w[i+1]) の解を使い回せる。
// f(i+1, j) = max(f(i+1, j - w[i+1]) + v[i+1], f(i, j)) if j >= w[i+1]
//           | f(i, j)                                   otherwise

// O(nw)
fn solve(n: usize, wvs: Vec<(usize, usize)>, w_max: usize) -> usize {
  let mut dp = vec![vec![0; w_max+1]; n+1];
  for (i, &(w, v)) in wvs.iter().enumerate() { // O(n)
    for j in 0..=w_max { // O(w)
      dp[i+1][j] = if j >= w { max(dp[i+1][j - w] + v, dp[i][j]) } else { dp[i][j] };
    }
  }
  dp[n][w_max]
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let wvs = vec![(3, 4), (4, 5), (2, 3)];
    let w = 7;
    assert_eq!(solve(n, wvs, w), 10);
  }
}
