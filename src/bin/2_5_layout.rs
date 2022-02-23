use num::*;
use petgraph::graph::*;
use petgraph::visit::*;
use proconio::input;
use proconio::source::{Readable, Source};
use std::io::BufRead;
use std::marker::PhantomData;
use std::ops::Sub;
use itertools::Itertools;

type Edges = Vec<(NodeIndex<u32>, NodeIndex<u32>, i32)>;

fn main() {
  input! {
    n: usize,
    ml: usize,
    md: usize,
    ls: [(NodeIndex1<u32>, NodeIndex1<u32>, i32); ml], // 最大距離制約
    ds: [(NodeIndex1<u32>, NodeIndex1<u32>, i32); md]  // 最小距離制約
  }
  println!("{}", solve(n, ls, ds));
}

fn solve(n: usize, ls: Edges, ds: Edges) -> i32 {
  // 順序制約をエンコード
  let es: Edges = (0..n).map(|i| (NodeIndex::from(i as u32 +1), NodeIndex::from(i as u32), 0)).collect_vec();
  let ds: Edges = ds.iter().map(|(u, v, w)| (*v, *u, 0-*w)).collect_vec();
  let g = DiGraph::<(), i32>::from_edges(itertools::concat(vec![es, ls, ds]));
  let inf = 1000000000;
  let res = bellman_ford(&g, NodeIndex::from(0), inf);
  match res {
    Ok((costs, _)) => if costs[n-1] == inf { -2 } else { costs[n-1] },
    Err(_) => -1
  }
}

#[cfg(test)]
mod tests {
  use super::solve;
  use petgraph::graph::NodeIndex;
  use itertools::Itertools;

  #[test]
  fn example1() {
    let n = 4;
    let _ml = 2;
    let _md = 1;
    let ls = vec![(1, 3, 10), (2, 4, 20)]
      .iter()
      .map(|(s, t, w)| (NodeIndex::from(s - 1), NodeIndex::from(t - 1), *w))
      .collect_vec();
    let ds = vec![(2, 3, 3)]
      .iter()
      .map(|(s, t, w)| (NodeIndex::from(s - 1), NodeIndex::from(t - 1), *w))
      .collect_vec();
    assert_eq!(solve(n, ls, ds), 27);
  }
}

// https://github.com/rust-lang-ja/atcoder-rust-base/blob/ja-all-enabled/examples/abc073-d.rs
struct NodeIndex1<Ix>(PhantomData<fn() -> Ix>);

impl<Ix: IndexType + Readable<Output = Ix> + One + Sub<Output = Ix>> Readable for NodeIndex1<Ix> {
  type Output = NodeIndex<Ix>;

  fn read<R: BufRead, S: Source<R>>(source: &mut S) -> NodeIndex<Ix> {
    NodeIndex::from(Ix::read(source) - Ix::one())
  }
}

pub fn bellman_ford<G>(
  g: G,
  source: G::NodeId,
  inf: G::EdgeWeight
) -> Result<(Vec<G::EdgeWeight>, Vec<Option<G::NodeId>>), NegativeCycle>
where
  G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
  G::EdgeWeight: Copy + PartialOrd + Num,
{
  let mut predecessor = vec![None; g.node_bound()];
  let mut distance = vec![inf; g.node_bound()];

  let ix = |i| g.to_index(i);

  distance[ix(source)] = <G::EdgeWeight>::zero();
  // scan up to |V| - 1 times.
  for _ in 1..g.node_count() {
    let mut did_update = false;
    for i in g.node_identifiers() {
      for edge in g.edges(i) {
        let i = edge.source();
        let j = edge.target();
        let w = *edge.weight();
        if distance[ix(i)] + w < distance[ix(j)] {
          distance[ix(j)] = distance[ix(i)] + w;
          predecessor[ix(j)] = Some(i);
          did_update = true;
        }
      }
    }
    if !did_update {
      break;
    }
  }

  // check for negative weight cycle
  for i in g.node_identifiers() {
    for edge in g.edges(i) {
      let j = edge.target();
      let w = *edge.weight();
      if distance[ix(i)] + w < distance[ix(j)] {
        return Err(NegativeCycle(()));
      }
    }
  }

  Ok((distance, predecessor))
}

#[derive(Clone, Debug, PartialEq)]
pub struct NegativeCycle(());
