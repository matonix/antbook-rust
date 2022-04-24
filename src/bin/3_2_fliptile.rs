use proconio::input;
use itertools::Itertools;
use antbook::Walk2D;

fn main() {
  input! {
    m: usize,
    n: usize,
    a: [[usize; n]; m],
  }
  if let Some(ans) = solve(m, n, a) {
    println!("{}", ans.iter().map(|line| line.iter().join(" ")).join("\n"));
  } else {
    println!("IMPOSSIBLE");
  }
}

fn solve(m: usize, n: usize, a: Vec<Vec<usize>>) -> Option<Vec<Vec<usize>>> {
  for inits in powerset((0..n).collect_vec()) {
    let mut ans = vec![vec![0; n]; m];
    let mut walk = Walk2D::new(m, n, a.clone());
    // 一行目のひっくり返し
    for c in inits {
      stomp(&mut walk, 0, c);
      ans[0][c] = 1;
    }
    // 残行のひっくり返し
    for i in 1..m {
      for j in 0..n {
        if walk[i-1][j] == 1 {
          stomp(&mut walk, i, j);
          ans[i][j] = 1;
        }
      }
    }
    // 最終行のチェック
    if (0..n).all(|j| walk[m-1][j] == 0) {
      return Some(ans);
    }
  }
  None
}

fn stomp(walk: &mut Walk2D<usize>, i: usize, j: usize) {
  walk[i][j] ^= 1;
  for (k, l) in walk.next_4way(i, j) {
    walk[k][l] ^= 1;
  }
}

// https://stackoverflow.com/questions/40718975/how-to-get-every-subset-of-a-vector-in-rust
fn powerset<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
  (0..=items.len())
      .map(|count| items.clone().into_iter().combinations(count))
      .flatten()
      .collect()
}

#[cfg(test)]
mod tests {
  use super::solve;
  use proconio::input;
  use proconio::source::auto::AutoSource;

  #[test]
  fn example1() {
    let m: usize = 4;
    let n: usize = 4;
    // raw string を使った読み込み https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let source = AutoSource::from(
      r"
      1 0 0 1
      0 1 1 0
      0 1 1 0
      1 0 0 1
      ",
    );
    input! {
      from source,
      a: [[usize; n]; m]
    }
    let source2 = AutoSource::from(
      r"
      0 0 0 0
      1 0 0 1
      1 0 0 1
      0 0 0 0
      ",
    );
    input! {
      from source2,
      expect: [[usize; n]; m]
    }
    assert_eq!(solve(m, n, a), Some(expect));
  }
}
