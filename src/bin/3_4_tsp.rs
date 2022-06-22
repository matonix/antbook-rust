use petgraph::matrix_graph::{MatrixGraph, node_index};
use proconio::input;

fn main() {
  input! {
    n: usize,
    e: usize,
    es: [(usize, usize, usize); e]
  }
  println!("{}", solve(n, es));
}

fn solve(n: usize, es: Vec<(usize, usize, usize)>) -> usize {
  let inf = usize::MAX / 2;
  let mut g = vec![vec![inf; n]; n];
  for (v, u, c) in es {
    g[v][u] = c;
  }

  let mut dp = vec![vec![inf; n]; 1 << n];
  dp[(1 << n) - 1][0] = 0;

  for s in (0..(1 << n) - 1).rev() {
    for v in 0..n {
      for u in 0..n {
        if (s >> u & 1) == 0 {
          dp[s][v] = dp[s][v].min(dp[s | 1 << u][u] + g[v][u])
        }
      }
    }
  }
  dp[0][0]
}

// メモ化再帰版（stack overflow する）
fn _solve(n: usize, es: Vec<(u16, u16, usize)>) -> usize {
  let g = MatrixGraph::<(), usize>::from_edges(&es);
  let mut dp = vec![vec![-1; n]; 1 << n];
  fn rec(n: &usize, g: &MatrixGraph<(), usize>, dp: &mut Vec<Vec<isize>>, s: usize, v: usize) -> usize {
    if dp[s][v] >= 0 {
      return dp[s][v] as usize
    }

    if s == (1 << n) - 1 && v == 0 {
      dp[s][v] = 0;
      return 0
    }
    let mut res = usize::MAX;
    for u in 0..*n {
      if (s >> n & 1) == 0 {
        res = res.min(rec(n, g, dp, s | 1 << u, u) + g[(node_index(v), node_index(u))])
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
