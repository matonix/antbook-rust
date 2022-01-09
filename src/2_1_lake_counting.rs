use proconio::marker::Chars;
use proconio::{fastout, input};

// 一時的に添字の値が負になることがあるので isize で保持して、配列アクセス時に usize に変換する
// 全部 usize でやる場合はオーバーフローを許容する算術 wrapping_* を使えば良さそう https://moshg.github.io/rust-std-ja/std/primitive.usize.html#method.wrapping_add

#[fastout]
fn main() {
  input! {
    n: isize,
    m: isize,
    mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
  }
  println!("{}", solve(n, m, &mut a));
}

fn solve(n: isize, m: isize, a: &mut Vec<Vec<char>>) -> isize {
  let mut cnt = 0;
  for i in 0..n {
    for j in 0..m {
      if a[i as usize][j as usize] == 'W' {
        cnt += 1;
        dfs(i, j, n, m, a)
      }
    }
  }
  cnt
}

// 事前条件: i, j が添字の範囲内であること。 a[i][j] == 'W' を満たすこと
fn dfs(i: isize, j: isize, n: isize, m: isize, a: &mut Vec<Vec<char>>) {
  a[i as usize][j as usize] = 'X'; // visited
  for p in i - 1..=i + 1 {
    for q in j - 1..=j + 1 {
      // dfs 呼び出しのための事前条件チェック
      if p >= 0 && p < n && q >= 0 && q < m && a[p as usize][q as usize] == 'W' {
        dfs(p, q, n, m, a)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let mut a = vec![
      "W........WW.",
      ".WWW.....WWW",
      "....WW...WW.",
      ".........WW.",
      ".........W..",
      "..W......W..",
      ".W.W.....WW.",
      "W.W.W.....W.",
      ".W.W......W.",
      "..W.......W.",
    ]
    .iter()
    .map(|&s| s.chars().collect())
    .collect();
    assert_eq!(solve(10, 12, &mut a), 3);
  }
}
