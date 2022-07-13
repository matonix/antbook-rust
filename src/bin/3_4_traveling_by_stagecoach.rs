use petgraph::{visit::NodeIndexable, Graph, Undirected};
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: usize,
    b: usize,
    t: [usize; n],
    es: [(usize, usize, usize); m],
  }
  println!("{}", solve(n, m, a, b, t, es));
}

fn solve(
  n: usize,
  m: usize,
  a: usize,
  b: usize,
  t: Vec<usize>,
  es: Vec<(usize, usize, usize)>,
) -> f64 {
  let g: Graph<(), usize, Undirected, usize> = Graph::from_edges(es);
  let mut dp = vec![vec![f64::INFINITY; 1 << n]; m]; // dp[tickets][pos] 全チケットを持った始点からの移動コスト
  let all_tickets = (1 << n) - 1;
  dp[all_tickets][a - 1] = 0.0;
  let mut res = f64::INFINITY;
  for s in (0..(1 << n)).rev() {
    res = res.min(dp[s][b - 1]);
    for v in 0..m {
      for i in 0..n {
        if (s >> i & 1) == 1 {
          for u in 0..m {
            // i を使って v -> u 移動
            if let Some(w) = g
              .find_edge(g.from_index(v), g.from_index(u))
              .map(|e| g[e] as f64)
            {
              dp[s & !(1 << i)][u] = dp[s & !(1 << i)][u].min(dp[s][v] + w / t[i] as f64);
            }
          }
        }
      }
    }
  }
  res
}

#[cfg(test)]
mod tests {
  use super::solve;
  use assert_approx_eq::assert_approx_eq;
  use itertools::Itertools;

  #[test]
  fn example1() {
    let n = 2;
    let m = 4;
    let a = 2;
    let b = 1;
    let t = vec![3, 1];
    let es = vec![(1, 3, 3), (1, 4, 2), (2, 3, 3), (2, 4, 5)]
      .into_iter()
      .map(|(u, v, c)| (u - 1, v - 1, c))
      .collect_vec();
    assert_approx_eq!(solve(n, m, a, b, t, es), 3.667, 1.0e-2);
  }
}
