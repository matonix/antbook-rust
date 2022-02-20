// use petgraph::algo::dijkstra; // これで一発
use num::*;
use petgraph::prelude::*;
use petgraph::visit::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[allow(unused_variables)]
fn main() {
  let mut graph = Graph::new_undirected();
  let a = graph.add_node(());
  let b = graph.add_node(());
  let c = graph.add_node(());
  let d = graph.add_node(());
  let e = graph.add_node(());
  let f = graph.add_node(());
  let g = graph.add_node(());

  // 蟻本の例
  graph.extend_with_edges(&[
    (a, b, 2),
    (a, c, 5),
    (b, c, 4),
    (b, d, 6),
    (b, e, 10),
    (c, d, 2),
    (d, f, 1),
    (e, f, 3),
    (e, g, 5),
    (f, g, 9),
  ]);

  let (dists, prev) = dijkstra(&graph, a);
  println!("{:?}", dists);
  println!("{:?}", get_path(&graph, prev, g));
  assert_eq!(dists[graph.to_index(d)], 7);
}

// 経路復元
pub fn get_path<G>(graph: G, prev: Vec<Option<G::NodeId>>, goal: G::NodeId) -> Vec<G::NodeId>
where
  G: NodeIndexable,
  G::NodeId: Eq,
{
  let mut curr = goal;
  let mut path = vec![goal];
  let ix = |x| graph.to_index(x);
  while let Some(prev) = prev[ix(curr)] {
    path.push(prev);
    curr = prev;
  }
  path.reverse();
  path
}

// 蟻本の実装
pub fn dijkstra<G>(
  graph: G,
  start: G::NodeId,
) -> (Vec<G::EdgeWeight>, Vec<Option<G::NodeId>>)
where
  G: IntoEdges + Visitable + NodeIndexable,
  G::NodeId: Eq,
  G::EdgeWeight: Copy + Ord + Num + Bounded,
{
  let mut visited = graph.visit_map();
  let mut scores = vec![<G::EdgeWeight>::max_value(); graph.node_bound()];
  let mut predecessor = vec![None; graph.node_bound()];
  let mut visit_next = BinaryHeap::new();
  let ix = |x| graph.to_index(x);
  scores[graph.to_index(start)] = <G::EdgeWeight>::zero();
  visit_next.push(MinScored(<G::EdgeWeight>::zero(), start));

  while let Some(MinScored(score, node)) = visit_next.pop() {
    if visited.is_visited(&node) {
      continue;
    }
    for edge in graph.edges(node) {
      let next = edge.target();
      if visited.is_visited(&next) {
        continue;
      }
      let next_score = score + *edge.weight();
      scores[ix(next)] = std::cmp::min(scores[ix(next)], next_score);
      visit_next.push(MinScored(scores[ix(next)], next));
      predecessor[ix(next)] = Some(node);
    }
    visited.visit(node);
  }
  (scores, predecessor)
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
