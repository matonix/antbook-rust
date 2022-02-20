use num::*;
use petgraph::graph::{IndexType, NodeIndex, UnGraph};
use petgraph::visit::*;
use proconio::input;
use proconio::source::{Readable, Source};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::BufRead;
use std::marker::PhantomData;
use std::ops::Sub;

fn main() {
  input! {
    n: usize,
    r: usize,
    es: [(NodeIndex1<u32>, NodeIndex1<u32>, u32); r]
  }
  println!("{}", solve(n, r, es));
}

fn solve(n: usize, _r: usize, es: Vec<(NodeIndex<u32>, NodeIndex<u32>, u32)>) -> u32 {
  let g = UnGraph::<(), u32>::from_edges(&es);
  let dist = dijkstra2(&g, NodeIndex::from(0));
  dist[n - 1]
}

#[cfg(test)]
mod tests {
  use super::solve;
  use petgraph::graph::NodeIndex;

  #[test]
  fn example1() {
    let n = 4;
    let r = 4;
    let es = vec![(1, 2, 100), (2, 3, 250), (2, 4, 200), (3, 4, 100)]
      .iter()
      .map(|(s, t, w)| (NodeIndex::from(s - 1), NodeIndex::from(t - 1), *w))
      .collect::<Vec<(NodeIndex<u32>, NodeIndex<u32>, u32)>>();
    assert_eq!(solve(n, r, es), 450);
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

// よくわからん
// 読んだ: https://kyo-pro.hatenablog.com/entry/ari-hon-2-5
pub fn dijkstra2<G>(graph: G, start: G::NodeId) -> Vec<G::EdgeWeight>
where
  G: IntoEdges + NodeIndexable,
  G::NodeId: Eq,
  G::EdgeWeight: Copy + Ord + Num + Bounded,
{
  let mut scores = vec![<G::EdgeWeight>::max_value(); graph.node_bound()];
  let mut scores2 = vec![<G::EdgeWeight>::max_value(); graph.node_bound()];
  let mut visit_next = BinaryHeap::new();
  let ix = |x| graph.to_index(x);
  scores[graph.to_index(start)] = <G::EdgeWeight>::zero();
  visit_next.push(MinScored(<G::EdgeWeight>::zero(), start));

  while let Some(MinScored(score, node)) = visit_next.pop() {
    if score > scores2[ix(node)] {
      continue;
    }
    for edge in graph.edges(node) {
      let next = edge.target();
      let mut next_score = score + *edge.weight();
      if scores[ix(next)] > next_score {
        std::mem::swap(&mut scores[ix(next)], &mut next_score);
        visit_next.push(MinScored(scores[ix(next)], next));
      }
      if scores2[ix(next)] > next_score && scores[ix(next)] < next_score {
        scores2[ix(next)] = next_score;
        visit_next.push(MinScored(scores2[ix(next)], next));
      }
    }
  }
  scores2
}

// ヒープに入れるコンテナ
// https://github.com/petgraph/petgraph/blob/944057cdc57824f90a36f838f3ee7f148157c801/src/scored.rs
#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
  #[inline]
  fn eq(&self, other: &MinScored<K, T>) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
  #[inline]
  fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
  #[inline]
  fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
    let a = &self.0;
    let b = &other.0;
    if a == b {
      Ordering::Equal
    } else if a < b {
      Ordering::Greater
    } else if a > b {
      Ordering::Less
    } else if a.ne(a) && b.ne(b) {
      // these are the NaN cases
      Ordering::Equal
    } else if a.ne(a) {
      // Order NaN less, so that it is last in the MinScore order
      Ordering::Less
    } else {
      Ordering::Greater
    }
  }
}
