use proconio::input;
use antbook::maximum_flow::dinic;
use petgraph::graph::Graph;

fn main() {
  input! {
    n: usize,
    k: usize,
    rcs: [(usize, usize); k]
  }
  println!("{}", solve(n, k, rcs));
}
fn solve(n: usize, _k: usize, rcs: Vec<(usize, usize)>) -> usize {
  let mut g = Graph::<(), usize>::new();
  let rs: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
  let cs: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
  let s = g.add_node(());
  let t = g.add_node(());
  g.extend_with_edges(rs.iter().map(|&r| (s, r, 1)));
  g.extend_with_edges(rcs.iter().map(|&(r, c)| (rs[r-1], cs[c-1], 1)));
  g.extend_with_edges(cs.iter().map(|&c| (c, t, 1)));
  dinic(s, t, &g)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let k = 4;
    let rcs = vec![(1, 1), (1, 3), (2, 2), (3, 2)];
    assert_eq!(solve(n, k, rcs), 2);
  }
}
