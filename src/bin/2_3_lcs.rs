use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    m: usize,
    s: Chars,
    t: Chars,
  }
  println!("{}", solve(n, m, s, t));
}

// lcs f(n, m)
// f(i+1, j+1) = | f(i, j) + 1 `max` f(i+1, j) `max` f(i, j+1) if s[i+1] == t[j+1] ※1-indexed
//               | f(i+1, j) `max` f(i, j+1) otherwise ※ f(i, j) <= f(i+1, j) かつ f(i, j) <= f(i, j+1) が常に成立するので

// 解： f(i, j) + 1 >= f(i+1, j) かつ f(i, j) + 1 >= f(i, j+1) が常に成立するので f(i, j) + 1 if s[i+1] == t[j+1] でいいらしい

fn solve(n: usize, m: usize, s: Vec<char>, t: Vec<char>) -> usize {
  let mut dp = vec![vec![0; m+1]; n+1];
  for (i, &sc) in s.iter().enumerate() {
    for (j, &tc) in t.iter().enumerate() {
      let x = max(dp[i+1][j], dp[i][j+1]);
      dp[i+1][j+1] = if sc == tc { max(dp[i][j] + 1, x) } else { x };
    }
  }
  dp[n][m]
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let m = 4;
    let s = "abcd".chars().collect();
    let t = "becd".chars().collect();
    assert_eq!(solve(n, m, s, t), 3);
  }
}
