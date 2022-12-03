use proconio::input;
use antbook::maximum_flow::dinic;
use petgraph::graph::DiGraph;

fn main() {
  input! {
    n: usize,
    m: usize,
    abs: [(i32, i32); n],
    ws: [(usize, usize, i32); m],
  }
  println!("{}", solve(n, m, abs, ws));
}
fn solve(n: usize, _m: usize, abs: Vec<(i32, i32)>, ws: Vec<(usize, usize, i32)>) -> i32 {
  let mut g = DiGraph::<(), i32>::new();
  let s = g.add_node(());
  let t = g.add_node(());
  let ns: Vec<_> = (0..n).map(|_| g.add_node(())).collect();
  g.extend_with_edges(abs.iter().enumerate().map(|(i, &(a, _))| (ns[i], t, a)));
  g.extend_with_edges(abs.iter().enumerate().map(|(i, &(_, b))| (s, ns[i], b)));
  g.extend_with_edges(ws.iter().map(|&(a, b, w)| (ns[a-1], ns[b-1], w)));
  g.extend_with_edges(ws.iter().map(|&(a, b, w)| (ns[b-1], ns[a-1], w)));
  dinic(s, t, &g)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let m = 1;
    let abs = vec![(1, 10), (2, 10), (10, 3)];
    let ws = vec![(2, 3, 1000)];
    assert_eq!(solve(n, m, abs, ws), 13);
  }
}
