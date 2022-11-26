use proconio::input;
use antbook::maximum_flow::dinic;
use petgraph::graph::Graph;

fn main() {
  input! {
    n: usize,
    f: usize,
    d: usize,
    fs: [[usize]; n],
    ds: [[usize]; n],
  }
  println!("{}", solve(n, f, d, fs, ds));
}
fn solve(n: usize, f: usize, d: usize, fns: Vec<Vec<usize>>, nds: Vec<Vec<usize>>) -> usize {
  let mut g = Graph::<(), usize>::new();
  let fs: Vec<_> = (0..f).map(|_| g.add_node(())).collect();
  let ns1: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
  let ns2: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
  let ds: Vec<_> = (0..d).map(|_| g.add_node(())).collect();
  let s = g.add_node(());
  let t = g.add_node(());
  g.extend_with_edges(fs.iter().map(|&f| (s, f, 1)));
  fns.iter().enumerate().for_each(|(n, v)| g.extend_with_edges(v.iter().map(|f| (fs[f-1], ns1[n], 1))));
  g.extend_with_edges(ns1.iter().zip(ns2.iter()).map(|(&n1, &n2)| (n1, n2, 1)));
  nds.iter().enumerate().for_each(|(n, v)| g.extend_with_edges(v.iter().map(|d| (ns2[n], ds[d-1], 1))));
  g.extend_with_edges(ds.iter().map(|&d| (d, t, 1)));
  dinic(s, t, &g)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let f = 3;
    let d = 3;
    let fs = vec![vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 3]];
    let ds = vec![vec![1, 3], vec![1, 2], vec![1, 2], vec![3]];
    assert_eq!(solve(n, f, d, fs, ds), 3);
  }
}
