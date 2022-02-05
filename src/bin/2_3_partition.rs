use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    d: usize,
  }
  println!("{}", solve(n, m, d));
}

// よくわからんかった
// ここが一番わかりやすい（なんで{a_i-1}について考えてるの？というところ）: http://techtipshoge.blogspot.com/2011/01/blog-post_28.html

// j の i 個以下への分割の総数（dpの計算において、参照が遠くなる方の添字を列成分（j）にしている）
// f(i, j) = f(i, j - i) + f(i - 1, j) if j >= i
//           f(i - 1, j)               otherwise
// f(0, 0) = 1 → なんで？ 
// f(_, _) = 0

fn solve(n: usize, m: usize, d: usize) -> usize {
  let mut dp = vec![vec![0; n + 1]; m + 1]; // n の m 個以下への分割
  dp[0][0] = 1;
  for i in 1..=m {
    for j in 0..=n {
      dp[i][j] = if j >= i {
        (dp[i][j - i] + dp[i - 1][j]) % d
      } else {
        dp[i - 1][j]
      };
    }
  }
  dp[m][n]
}

// 関連
// 分割数: https://ja.wikipedia.org/wiki/%E5%88%86%E5%89%B2%E6%95%B0 ←今回はこれ（ただし漸化式の構成は異なる）
// 自然数の分割（列挙）: https://ja.wikipedia.org/wiki/%E8%87%AA%E7%84%B6%E6%95%B0%E3%81%AE%E5%88%86%E5%89%B2
// 区別できる品物の場合→集合の分割: https://ja.wikipedia.org/wiki/%E9%9B%86%E5%90%88%E3%81%AE%E5%88%86%E5%89%B2

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let m = 3;
    let d = 10000;
    assert_eq!(solve(n, m, d), 4);
  }
}
