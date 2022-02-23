use itertools::Itertools;
use num::*;
use petgraph::algo::min_spanning_tree;
use petgraph::graph::*;
use petgraph::data::FromElements;
use proconio::input;
use proconio::source::{Readable, Source};
use std::cmp::Reverse;
use std::io::BufRead;
use std::marker::PhantomData;
use std::ops::Sub;

fn main() {
  input! {
    n: usize,
    m: usize,
    r: usize,
    es: [(NodeIndex0<u32>, NodeIndex0<u32>, u32); r]
  }
  println!("{}", solve(n, m, r, es));
}

fn solve(n: usize, m: usize, _r: usize, es: Vec<(NodeIndex<u32>, NodeIndex<u32>, u32)>) -> u32 {
  let rev_cost_edges = es
    .iter()
    .map(|(u, v, w)| (*u, NodeIndex::from((m + v.index()) as u32), Reverse(*w)))
    .collect_vec();
  let g = UnGraph::<(), Reverse<u32>>::from_edges(&rev_cost_edges);
  let mst = min_spanning_tree(&g);
  let ws: u32 = Graph::<(), Reverse<u32>>::from_elements(mst).edge_references().map(|e| e.weight().0).sum();
  10000 * (n as u32 + m as u32) - ws
}

#[cfg(test)]
mod tests {
  use super::solve;
  use petgraph::graph::NodeIndex;

  #[test]
  fn example1() {
    let n = 5;
    let m = 5;
    let r = 8;
    let es = vec![
      (4, 3, 6831),
      (1, 3, 4583),
      (0, 0, 6592),
      (0, 1, 3063),
      (3, 3, 4975),
      (1, 3, 2049),
      (4, 2, 2104),
      (2, 2, 781),
    ]
    .iter()
    .map(|(s, t, w)| (NodeIndex::from(*s), NodeIndex::from(*t), *w))
    .collect::<Vec<(NodeIndex<u32>, NodeIndex<u32>, u32)>>();
    assert_eq!(solve(n, m, r, es), 71071);
  }
}

// https://github.com/rust-lang-ja/atcoder-rust-base/blob/ja-all-enabled/examples/abc073-d.rs
struct NodeIndex0<Ix>(PhantomData<fn() -> Ix>);

impl<Ix: IndexType + Readable<Output = Ix> + One + Sub<Output = Ix>> Readable for NodeIndex0<Ix> {
  type Output = NodeIndex<Ix>;

  fn read<R: BufRead, S: Source<R>>(source: &mut S) -> NodeIndex<Ix> {
    NodeIndex::from(Ix::read(source))
  }
}
