use proconio::input;

fn main() {
  input! {
    n: usize,
    e: usize,
    es: [(u16, u16, usize); e]
  }
  println!("{}", solve(n, es));
}

fn _solve(n: usize, es: Vec<(u16, u16, usize)>) -> usize {
  let inf = usize::MAX;
  let mut g = vec![vec![inf; n]; n];
  for (v, u, c) in es {
    g[v as usize][u as usize] = c;
  }

  let mut dp = vec![vec![inf; n]; 1 << n];
  dp[(1 << n) - 1][0] = 0;

  for s in (0..(1 << n) - 1).rev() {
    for v in 0..n {
      for u in 0..n {
        if (s >> u & 1) == 0 {
          dp[s][v] = dp[s][v].min(dp[s | 1 << u][u].saturating_add(g[v][u]))
        }
      }
    }
  }
  dp[0][0]
}

// メモ化再帰版
fn solve(n: usize, es: Vec<(u16, u16, usize)>) -> usize {
  let inf = usize::MAX;
  let mut g = vec![vec![inf; n]; n];
  for (v, u, c) in es {
    g[v as usize][u as usize] = c;
  }
  let mut dp = vec![vec![-1; n]; 1 << n];
  fn rec(n: &usize, g: &Vec<Vec<usize>>, dp: &mut Vec<Vec<isize>>, s: usize, v: usize) -> usize {
    let inf = usize::MAX;
    if dp[s][v] >= 0 {
      return dp[s][v] as usize
    }

    if s == (1 << n) - 1 && v == 0 {
      dp[s][v] = 0;
      return 0
    }
    let mut res = inf;
    for u in 0..*n {
      if (s >> u & 1) == 0 {
        res = res.min(rec(n, g, dp, s | 1 << u, u).saturating_add(g[v][u]))
      }
    }
    dp[s][v] = res as isize;
    res
  }
  rec(&n, &g, &mut dp, 0, 0)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 5;
    let es = vec![(0, 1, 3), (0, 3, 4), (1, 2, 5), (2, 0, 4), (2, 3, 5), (3, 4, 3), (4, 0, 7), (4, 1, 6)];
    assert_eq!(solve(n, es), 22);
  }
}
