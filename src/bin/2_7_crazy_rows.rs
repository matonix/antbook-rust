use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [[u32; n]; n],
  }
  dbg!(solve(n, a));
}

// 方針: 10進数に変換してバブルソート
// 10進数表現 x が下三角を満たす条件: x >= 2^{n - i - 1}
// n - 1 番目は d..ddd1
// n - 2 番目は d..dd10
// n - 3 番目は d..d100 という感じ
// この場合、ドントケアな桁も考慮して交換してしまうので、NG

// 解答より: 気にすべきは最後に１が出た桁数なので、これにエンコードする
// 下三角条件 i >= a[j] を満たす j を求めて、交換を繰り返す。（バブルソートでも選択ソートでもない・・・）
fn solve(n: usize, a: Vec<Vec<u32>>) -> u32 {
  let mut a = a
    .iter()
    .map(|v| v.iter().rposition(|&x| x == 1).unwrap_or(0))
    .collect_vec();
  let mut ans = 0;
  for i in 0..n {
    if let Some(j) = a[i..].iter().position(|&x| i >= x) {
      for k in (i..i + j).rev() {
        a.swap(k, k + 1);
        ans += 1;
      }
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;
  #[test]
  fn example1() {
    let n = 2;
    let a = vec![vec![1, 0], vec![1, 1]];
    assert_eq!(solve(n, a), 0)
  }
  #[test]
  fn example2() {
    let n = 3;
    let a = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 1, 0]];
    assert_eq!(solve(n, a), 2)
  }
  #[test]
  fn example3() {
    let n = 4;
    let a = vec![
      vec![1, 1, 1, 0],
      vec![1, 1, 0, 0],
      vec![1, 1, 0, 0],
      vec![1, 0, 0, 0],
    ];
    assert_eq!(solve(n, a), 4)
  }
}
