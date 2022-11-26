use antbook::maximum_flow::dinic;
use fixedbitset::FixedBitSet;
use petgraph::{prelude::*, visit::*};
use std::collections::HashMap;

fn main() {
  let mut graph = Graph::new();
  let s = graph.add_node(());
  let u1 = graph.add_node(());
  let u2 = graph.add_node(());
  let u3 = graph.add_node(());
  let v1 = graph.add_node(());
  let v2 = graph.add_node(());
  let v3 = graph.add_node(());
  let t = graph.add_node(());
  graph.extend_with_edges(&[
    (s, u1, 1),
    (s, u2, 1),
    (s, u3, 1),
    (u1, v1, 1),
    (u1, v2, 1),
    (u2, v1, 1),
    (u2, v3, 1),
    (u3, v2, 1),
    (v1, t, 1),
    (v2, t, 1),
    (v3, t, 1),
  ]);
  dbg!(&dinic(s, t, &graph));
  another();
}

fn another() {
  let mut graph = Graph::new_undirected();
  let u1 = graph.add_node(());
  let u2 = graph.add_node(());
  let u3 = graph.add_node(());
  let v1 = graph.add_node(());
  let v2 = graph.add_node(());
  let v3 = graph.add_node(());
  graph.extend_with_edges(&[(u1, v1), (u1, v2), (u2, v1), (u2, v3), (u3, v2)]);
  dbg!(&hopcroft_karp(&graph));
}

// p.197の容量1二部グラフのアルゴリズム
// https://en.wikipedia.org/wiki/Hopcroft%E2%80%93Karp_algorithm
fn hopcroft_karp(g: &UnGraph<(), ()>) -> usize {
  fn dfs(
    v: NodeIndex,
    g: &UnGraph<(), ()>,
    visit_map: &mut FixedBitSet,
    matching: &mut HashMap<NodeIndex, NodeIndex>,
  ) -> bool {
    visit_map.visit(v);
    for u in g.neighbors_undirected(v) {
      if !matching.contains_key(&u)
        || !visit_map.is_visited(matching.get(&u).unwrap())
          && dfs(*matching.get(&u).unwrap(), g, visit_map, matching)
      {
        matching.insert(v, u);
        matching.insert(u, v);
        return true;
      }
    }
    false
  }

  let mut res = 0;
  let mut matching: HashMap<NodeIndex, NodeIndex> = HashMap::new();
  for (v, _) in g.node_references() {
    if !matching.contains_key(&v) {
      let mut visit_map = g.visit_map();
      if dfs(v, &g, &mut visit_map, &mut matching) {
        res += 1;
      }
    }
  }
  res
}
