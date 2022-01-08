use proconio::marker::Chars;
use proconio::{fastout, input};

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
  fn dfs(i: isize, j: isize, n: isize, m: isize, a: &mut Vec<Vec<char>>) {
    if i < 0 || i >= n || j < 0 || j >= m {
    } else if a[i as usize][j as usize] == 'W' {
      a[i as usize][j as usize] = 'X'; // visited
      for p in -1..=1 {
        for q in -1..=1 {
          dfs(i+p, j+q, n, m, a)
        }
      }
    }
  }
  cnt
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
    ].iter().map(|&s| s.chars().collect()).collect();
    assert_eq!(solve(10, 12, &mut a), 3);
  }
}
