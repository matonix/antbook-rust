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

  let (dists, prev) = dijkstra(&graph, a, None, |e| *e.weight());
  println!("{:?}", dists);
  println!("{:?}", get_path(prev, g));
  assert_eq!(dists[&d], 7);
}

// 経路復元
pub fn get_path<N>(prev: HashMap<N, N>, goal: N) -> Vec<N>
where
  N: Eq + Hash + Copy,
{
  let mut curr = goal;
  let mut path = vec![goal];
  while let Some(prev) = prev.get(&curr) {
    path.push(*prev);
    curr = *prev;
  }
  path.reverse();
  path
}

// 蟻本の実装…とみせかけてほぼpetgraph実装
// 辺のコスト周りはnum-traitを使っていった方が良いと思う
// TODO: INFを用いた実装
pub fn dijkstra<G, F, K>(
  graph: G,
  start: G::NodeId,
  _goal: Option<G::NodeId>,
  mut edge_cost: F,
) -> (HashMap<G::NodeId, K>, HashMap<G::NodeId, G::NodeId>)
where
  G: IntoEdges + Visitable,
  G::NodeId: Eq + Hash,
  F: FnMut(G::EdgeRef) -> K,
  K: Measure + Copy,
{
  let mut visited = graph.visit_map();
  let mut scores = HashMap::new(); // INF を使わない実装方法
  let mut predecessor = HashMap::new();
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
            predecessor.insert(next.clone(), node.clone());
          }
        }
        Vacant(ent) => {
          ent.insert(next_score);
          visit_next.push(MinScored(next_score, next));
          predecessor.insert(next.clone(), node.clone());
        }
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