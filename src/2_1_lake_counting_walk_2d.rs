use proconio::marker::Chars;
use proconio::{fastout, input};
use walk_2d::Walk2D;

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
  }
  println!("{}", solve(n, m, a));
}

fn solve(n: usize, m: usize, a: Vec<Vec<char>>) -> usize {
  let mut cnt = 0;
  let mut a = Walk2D::new(n, m, a);
  for i in 0..n {
    for j in 0..m {
      if a[i][j] == 'W' {
        cnt += 1;
        dfs(i, j, n, m, &mut a)
      }
    }
  }
  cnt
}

// 事前条件: a[i][j] == 'W' を満たすこと
fn dfs(i: usize, j: usize, n: usize, m: usize, a: &mut Walk2D<char>) {
  a[i][j] = 'X'; // visited
  for (ni, nj) in a.next_8way(i, j) {
    // dfs 呼び出しのための事前条件チェック
    if a[ni][nj] == 'W' {
      dfs(ni, nj, n, m, a)
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
    assert_eq!(solve(n, m, a), 3);
  }
}
