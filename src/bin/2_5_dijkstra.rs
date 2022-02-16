// use petgraph::algo::dijkstra; // これで一発
use petgraph::algo::Measure;
use petgraph::visit::EdgeRef;
use petgraph::visit::VisitMap;
use petgraph::visit::{IntoEdges, Visitable};
use petgraph::Graph;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

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

  let dists = dijkstra(&graph, a, None, |e| *e.weight());
  println!("{:?}", dists);
  assert_eq!(dists[&d], 7);
}

// 蟻本の実装…とみせかけてほぼpetgraph実装
pub fn dijkstra<G, F, K>(
  graph: G,
  start: G::NodeId,
  _goal: Option<G::NodeId>,
  mut edge_cost: F,
) -> HashMap<G::NodeId, K>
where
  G: IntoEdges + Visitable,
  G::NodeId: Eq + Hash,
  F: FnMut(G::EdgeRef) -> K,
  K: Measure + Copy,
{
  let mut visited = graph.visit_map();
  let mut scores = HashMap::new(); // INF を使わない実装方法
  let mut visit_next = BinaryHeap::new();
  let zero_score = K::default();
  scores.insert(start, zero_score);
  visit_next.push(MinScored(zero_score, start));

  while let Some(MinScored(score, node)) = visit_next.pop() {
    if visited.is_visited(&node) {
      continue;
    }
    for edge in graph.edges(node) {
      let next = edge.target();
      if visited.is_visited(&next) {
        continue;
      }
      let next_score = score + edge_cost(edge); // edge.weight() でもよいかと
      match scores.entry(next) {
        Occupied(ent) => {
          if next_score < *ent.get() {
            *ent.into_mut() = next_score;
            visit_next.push(MinScored(next_score, next));
          }
        }
        Vacant(ent) => {
          ent.insert(next_score);
          visit_next.push(MinScored(next_score, next));
        }
      }
    }
    visited.visit(node);
  }
  scores
}

// ヒープに入れるコンテナ
struct MinScored<K, N>(K, N);
impl<K, N> Ord for MinScored<K, N>
where
  N: Eq + Hash,
  K: Measure + Copy,
{
  fn cmp(&self, other: &Self) -> Ordering {
    if let Some(res) = other.0.partial_cmp(&self.0) {
      res
    } else {
      Ordering::Equal
    }
  }
}

impl<K, N> PartialOrd for MinScored<K, N>
where
  N: Eq + Hash,
  K: Measure + Copy,
{
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    other.0.partial_cmp(&self.0)
  }
}

impl<K, N> PartialEq for MinScored<K, N>
where
  N: Eq + Hash,
  K: Measure + Copy,
{
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<K, N> Eq for MinScored<K, N>
where
  N: Eq + Hash,
  K: Measure + Copy,
{
}
