use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; n],
    d: usize,
  }
  println!("{}", solve(n, m, a, d));
}

// f(i + 1, j) = 0..i番目の荷物からちょうど j 個抽出する場合の総和
// 0..i-1 番目の荷物から 0..j 個それぞれ抽出した場合の総和を考える
// （例えば k 個抽出した場合は、残りの j - k 個を i 番目の荷物で占めれば合計 j 個になる）
// f(i + 1, j) = Σ_k=0..min(j, a_i) f(i, j - k) 3重ループ
// O(nm^2)
fn _solve(n: usize, m: usize, a: Vec<usize>, d: usize) -> usize {
  let mut dp = vec![vec![0; m + 1]; n + 1];
  for i in 0..=n { // 0 個選ぶ場合は常に 1
    dp[i][0] = 1;
  }
  for i in 0..n {
    for j in 1..=m {
      for k in 0..=std::cmp::min(j, a[i]) {
        dp[i + 1][j] += dp[i][j - k] % d;
      }
    }
  }
  dp[n][m]
}

// f(i + 1, j) = Σ_k=0..min(j, a_i) f(i, j - k) 3重ループだが
// f(i + 1, j - 1) = Σ_k=0..min(j - 1, a_i) f(i, j - k - 1) を考えると、
// f(i + 1, j) - f(i + 1, j - 1) = f(i, j) ※一番最後 - f(i, j - a_i - 1) ※一番最初 となり
// f(i + 1, j) = f(i + 1, j - 1) + f(i, j) - f(i, j - a_i - 1) と書ける (j - a_i - 1 >= 0 のとき)
// O(nm)
fn solve(n: usize, m: usize, a: Vec<usize>, d: usize) -> usize {
  let mut dp = vec![vec![0; m + 1]; n + 1]; // 0-indexed
  for i in 0..=n { // 0 個選ぶ場合は常に 1
    dp[i][0] = 1;
  }
  for i in 0..n {
    for j in 1..=m {
      if j >= a[i] + 1 { 
        dp[i + 1][j] = (dp[i + 1][j - 1] + dp[i][j] - dp[i][j - a[i] - 1] + d) % d;
      } else { 
        dp[i + 1][j] = (dp[i + 1][j - 1] + dp[i][j]) % d;
      }
    }
  }
  dp[n][m]
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let m = 3;
    let a = vec![1, 2, 3];
    let d = 10000;
    assert_eq!(solve(n, m, a, d), 6);
  }
}
