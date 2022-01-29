use proconio::input;
use antbook::binary_search::BinarySearch;

fn main() {
  input! {
    n: usize,
    a: [usize; n]
  }
  println!("{}", solve(n, a));
}

// lis f(n)
// f(i) = Max_j=0..i ( if a[i] > a[j] then f(j) + 1 else 1 ) ※ a[j] を最後にもつ lis を増加させるなら +1 し、そうでなければ単独の列（長さ1）
// O(n^2)
fn _solve(n: usize, a: Vec<usize>) -> usize {
  let mut dp = vec![1; n]; // 0-indexed
  for i in 0..n {
    for j in 0..i {
      if a[i] > a[j] {
        dp[i] = std::cmp::max(dp[i], dp[j] + 1);
      }
    }
  }
  dp[n-1]
}

// O(n log n)
// 蟻本の説明は意味不明だったのでここのデモを見て挙動を理解した： https://en.wikipedia.org/wiki/Longest_increasing_subsequence
fn solve(n: usize, a: Vec<usize>) -> usize {
  let mut dp = vec![usize::MAX; n]; // 0-indexed
  for i in 0..n {
    let j = dp.lower_bound(&a[i]);
    dp[j] = a[i];
  }
  dp.lower_bound(&usize::MAX)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 5;
    let a = vec![4, 2, 3, 1, 5];
    assert_eq!(solve(n, a), 3);
  }
  #[test]
  fn example2() {
    let n = 4;
    let a = vec![1, 2, 1, 2];
    assert_eq!(solve(n, a), 2);
  }
}
