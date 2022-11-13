use num::*;
use num::traits::WrappingAdd;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;
use petgraph::prelude::*;
use petgraph::visit::*;

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
// use petgraph::algo::dijkstra; で一発だが、経路復元のための構造を返す。
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
      if scores[ix(next)] > next_score {
        scores[ix(next)] = next_score;
        visit_next.push(MinScored(next_score, next));
        predecessor[ix(next)] = Some(node);
      }
    }
    visited.visit(node);
  }
  (scores, predecessor)
}

pub fn dijkstra_with_potential<G, W>(
  graph: G,
  start: G::NodeId,
) -> (Vec<G::EdgeWeight>, Vec<Option<G::NodeId>>)
where
  W: Copy + Ord + Num + Bounded,
  G: IntoEdges + Visitable + NodeIndexable + IntoNodeReferences<NodeWeight = W, EdgeWeight = W>,
  G::NodeId: Eq + Hash,
{
  let mut visited = graph.visit_map();
  let mut scores = vec![<G::EdgeWeight>::max_value(); graph.node_bound()];
  let mut predecessor = vec![None; graph.node_bound()];
  let mut visit_next = BinaryHeap::new();
  let ix = |x| graph.to_index(x);
  scores[graph.to_index(start)] = <G::EdgeWeight>::zero();
  let node_weights: HashMap<G::NodeId, G::NodeWeight> = graph.node_references().map(|n| (n.id(), *n.weight())).collect();
  visit_next.push(MinScored(<G::EdgeWeight>::zero() + *node_weights.get(&start).unwrap(), start));

  while let Some(MinScored(score, node)) = visit_next.pop() {
    if visited.is_visited(&node) {
      continue;
    }
    for edge in graph.edges(node) {
      let next = edge.target();
      if visited.is_visited(&next) {
        continue;
      }
      let next_score = score + *edge.weight() + *node_weights.get(&node).unwrap() - *node_weights.get(&next).unwrap();
      if scores[ix(next)] > next_score {
        scores[ix(next)] = next_score;
        visit_next.push(MinScored(next_score, next));
        predecessor[ix(next)] = Some(node);
      }
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

// Num系トレイトを使った整数重みの辺に対する bellman ford
// 浮動小数点数の場合は petgraph::algo::bellman_ford
pub fn bellman_ford<G>(
  g: G,
  source: G::NodeId,
) -> Result<(Vec<G::EdgeWeight>, Vec<Option<G::NodeId>>), NegativeCycle>
where
  G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
  G::EdgeWeight: Copy + PartialOrd + Num + Bounded + WrappingAdd,
{
  let mut predecessor = vec![None; g.node_bound()];
  let mut distance = vec![<G::EdgeWeight>::max_value(); g.node_bound()];

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
        if distance[ix(i)].wrapping_add(&w) < distance[ix(j)] {
          distance[ix(j)] = distance[ix(i)].wrapping_add(&w);
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
      if distance[ix(i)].wrapping_add(&w) < distance[ix(j)] {
        return Err(NegativeCycle(()));
      }
    }
  }

  Ok((distance, predecessor))
}

#[derive(Clone, Debug, PartialEq)]
pub struct NegativeCycle(());


mod tests {

  #[allow(unused_variables)]
  #[test]
  fn test_dijkstra() {
    let mut graph = super::Graph::new_undirected();
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
  
    let (dists, prev) = super::dijkstra(&graph, a);
    println!("{:?}", dists);
    println!("{:?}", super::get_path(&graph, prev, g));
    assert_eq!(dists[petgraph::visit::NodeIndexable::to_index(&graph, d)], 7);
  }

  #[allow(unused_variables)]
  #[test]
  fn test_bellman_ford() {
    let mut g = super::Graph::new();
    let a = g.add_node(()); // node with no weight
    let b = g.add_node(());
    let c = g.add_node(());
    let d = g.add_node(());
    let e = g.add_node(());
    let f = g.add_node(());
    g.extend_with_edges(&[
      (0, 1, 2),
      (0, 3, 4),
      (1, 2, 1),
      (1, 5, 7),
      (2, 4, 5),
      (4, 5, 1),
      (3, 4, 1),
    ]);

    // Graph represented with the weight of each edge
    //
    //     2       1
    // a ----- b ----- c
    // | 4     | 7     |
    // d       f       | 5
    // | 1     | 1     |
    // \------ e ------/

    let path = super::bellman_ford(&g, a);
    assert_eq!(
      path,
      Ok((
        vec![0, 2, 3, 4, 5, 6],
        vec![None, Some(a), Some(b), Some(a), Some(d), Some(e)]
      ))
    );
    let graph_with_neg_cycle = super::UnGraph::<(), i32>::from_edges(&[
      (0, 1, -2),
      (0, 3, -4),
      (1, 2, -1),
      (1, 5, -25),
      (2, 4, -5),
      (4, 5, -25),
      (3, 4, -1),
    ]);

    assert!(super::bellman_ford(&graph_with_neg_cycle, super::NodeIndex::new(0)).is_err());
  }
}
