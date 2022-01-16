use proconio::marker::Chars;
use proconio::{fastout, input};

// TODO: 4, 8 方向に usize で探索できるデータ構造

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
  }
  println!("{}", solve(n, m, &mut a));
}

fn solve(n: usize, m: usize, a: &mut Vec<Vec<char>>) -> usize {
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
fn dfs(i: usize, j: usize, n: usize, m: usize, a: &mut Vec<Vec<char>>) {
  a[i][j] = 'X'; // visited
  for di in 0..=2 {
    for dj in 0..=2 {
      // 添字の調整 (usize なので負の値をとれない)
      let ni = i.wrapping_add(di).wrapping_sub(1);
      let nj = j.wrapping_add(dj).wrapping_sub(1);
      // dfs 呼び出しのための事前条件チェック
      if ni < n && nj < m && a[ni][nj] == 'W' {
        dfs(ni, nj, n, m, a)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::solve;
  use proconio::input;
  use proconio::marker::Chars;
  use proconio::source::auto::AutoSource;

  #[test]
  fn example1() {
    let n: usize = 10;
    let m: usize = 12;
    // raw string を使った読み込み https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let source = AutoSource::from(
      r"
      W........WW.
      .WWW.....WWW
      ....WW...WW.
      .........WW.
      .........W..
      ..W......W..
      .W.W.....WW.
      W.W.W.....W.
      .W.W......W.
      ..W.......W.
      ",
    );
    input! {
      from source,
      mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
    }
    assert_eq!(solve(n, m, &mut a), 3);
  }
}
