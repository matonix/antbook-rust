// use petgraph::algo::min_spanning_tree; // これで一発
use petgraph::prelude::*;
use petgraph::unionfind::UnionFind;
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

  // 蟻本の例（記号はないので、上から順番に
  graph.extend_with_edges(&[
    (a, b, 1),
    (b, c, 2),
    (b, e, 10),
    (c, d, 3),
    (c, f, 7),
    (d, f, 1),
    (d, g, 5),
    (e, f, 5),
    (f, g, 8),
  ]);

  // let mst = min_spanning_tree(&graph);
  let mst = kruskal(&graph);
  // let mst = prim(&graph);
  for e in mst {
    println!("{:?}", e);
  }
}

pub fn prim<G>(g: G) -> Vec<G::EdgeRef>
where
  G::EdgeWeight: Copy + Ord,
  G: IntoNodeIdentifiers + Visitable + IntoEdges + GraphProp,
{
  let mut mst = vec![];
  let mut visited = g.visit_map();
  let mut visit_next = BinaryHeap::new();
  let start = g.node_identifiers().next().unwrap();
  if g.is_directed() {
    panic!("ensure input graph is undirected")
  }
  visited.visit(start);
  for edge in g.edges(start) {
    visit_next.push(MinScored(*edge.weight(), edge));
  }
  while let Some(MinScored(_score, edge)) = visit_next.pop() {
    if visited.is_visited(&edge.source()) && !visited.is_visited(&edge.target()) {
      visited.visit(edge.target());
      mst.push(edge);
      for next_edge in g.edges(edge.target()) {
        if !visited.is_visited(&next_edge.target()) {
          visit_next.push(MinScored(*next_edge.weight(), next_edge));
        }
      }
    }
  }
  mst
}

pub fn kruskal<G>(g: G) -> Vec<G::EdgeRef>
where
  G::EdgeWeight: Clone + Ord,
  G: IntoEdgeReferences + NodeIndexable,
{
  let edges = g.edge_references();
  let mut sorted_edges = BinaryHeap::with_capacity(edges.size_hint().0);
  for edge in edges {
    sorted_edges.push(MinScored(edge.weight().clone(), edge));
  }
  let mut uf = UnionFind::new(g.node_bound());
  let mut mst = vec![];
  while let Some(MinScored(_score, edge)) = sorted_edges.pop() {
    let u = g.to_index(edge.source());
    let v = g.to_index(edge.target());
    if !uf.equiv(u, v) {
      uf.union(u, v);
      mst.push(edge);
    }
  }
  mst
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
