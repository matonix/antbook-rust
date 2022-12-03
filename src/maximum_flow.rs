use fixedbitset::FixedBitSet;
use num::{traits::NumAssign, Bounded, Zero};
use petgraph::{prelude::*, visit::{Visitable, VisitMap}};
use std::{
  collections::{HashMap, VecDeque},
  fmt::Debug,
};

pub fn ford_fulkerson<E>(s: NodeIndex, t: NodeIndex, graph: &DiGraph<(), E>) -> E
where
  E: Copy + NumAssign + Bounded + Ord + Debug,
{
  let g = UnGraph::from_edges(graph.edge_references().map(|e| (e.source(), e.target())));
  let mut cap = HashMap::new();
  for e in graph.edge_references() {
    if !cap.contains_key(&(e.source(), e.target())) {
      cap.insert((e.source(), e.target()), *e.weight());
    }
    if !cap.contains_key(&(e.target(), e.source())) {
      cap.insert((e.target(), e.source()), E::zero());
    }
  }

  fn dfs<E>(
    visit_map: &mut FixedBitSet,
    cap: &mut HashMap<(NodeIndex, NodeIndex), E>,
    g: &UnGraph<(), ()>,
    v: NodeIndex,
    t: NodeIndex,
    f: E,
  ) -> E
  where
    E: Copy + NumAssign + Bounded + Ord + Debug,
  {
    if v == t {
      return f;
    }
    visit_map.visit(v);
    for e in g.edges(v) {
      let u = e.target();
      if let Some(w) = cap.get(&(v, u)) {
        if !visit_map.is_visited(&u) && *w > E::zero() {
          let d = dfs(visit_map, cap, g, u, t, f.min(*w));
          if d > E::zero() {
            if let Some(w_mut) = cap.get_mut(&(v, u)) {
              *w_mut -= d;
            }
            if let Some(m_mut) = cap.get_mut(&(u, v)) {
              *m_mut += d;
            }
            return d;
          }
        }
      }
    }
    E::zero()
  }

  let mut flow = E::zero();
  loop {
    let mut visit_map = g.visit_map();
    let f = dfs(&mut visit_map, &mut cap, &g, s, t, E::max_value());
    if f == E::zero() {
      return flow;
    }
    flow += f;
  }
}

pub fn dinic<E>(s: NodeIndex, t: NodeIndex, graph: &DiGraph<(), E>) -> E
where
  E: Copy + NumAssign + Bounded + Ord + Debug,
{
  // グラフそのものをイミュータブル、辺の重みをミュータブルにするため、構造を分離
  let g = UnGraph::from_edges(graph.edge_references().map(|e| (e.source(), e.target())));
  let mut cap = HashMap::new();
  for e in graph.edge_references() {
    if !cap.contains_key(&(e.source(), e.target())) {
      cap.insert((e.source(), e.target()), *e.weight());
    }
    if !cap.contains_key(&(e.target(), e.source())) {
      cap.insert((e.target(), e.source()), E::zero());
    }
  }

  // s から各ノードへの最短距離の計算
  fn bfs<E>(
    cap: &HashMap<(NodeIndex, NodeIndex), E>,
    g: &UnGraph<(), ()>,
    s: NodeIndex,
  ) -> Vec<isize>
  where
    E: Copy + Zero + Ord,
  {
    let mut level = vec![-1; g.node_count()];
    let mut q = VecDeque::new();
    level[s.index()] = 0;
    q.push_back(s);
    while let Some(v) = q.pop_front() {
      for e in g.edges(v) {
        let u = e.target();
        if let Some(c) = cap.get(&(v, u)) {
          if *c > E::zero() && level[u.index()] < 0 {
            level[u.index()] = level[v.index()] + 1;
            q.push_back(u);
          }
        }
      }
    }
    level
  }

  // 増加パスの探索
  fn dfs<E>(
    level: &Vec<isize>,
    cap: &mut HashMap<(NodeIndex, NodeIndex), E>,
    g: &UnGraph<(), ()>,
    v: NodeIndex,
    t: NodeIndex,
    f: E,
  ) -> Option<E>
  where
    E: Copy + NumAssign + Bounded + Ord + Debug,
  {
    if v == t {
      return Some(f);
    }
    for e in g.edges(v) {
      let u = e.target();
      if let Some(w) = cap.get(&(v, u)) {
        if *w > E::zero() && level[v.index()] < level[u.index()] {
          if let Some(d) = dfs(level, cap, g, u, t, f.min(*w)) {
            if let Some(w_mut) = cap.get_mut(&(v, u)) {
              *w_mut -= d;
            }
            if let Some(m_mut) = cap.get_mut(&(u, v)) {
              *m_mut += d;
            }
            return Some(d);
          }
        }
      }
    }
    None
  }

  let mut flow = E::zero();
  loop {
    let level = bfs(&cap, &g, s);
    if level[t.index()] < 0 {
      return flow;
    }
    while let Some(f) = dfs(&level, &mut cap, &g, s, t, E::max_value()) {
      flow += f;
    }
  }
}

mod tests {
  #[test]
  fn test() {
    let mut graph = petgraph::Graph::new();
    let s = graph.add_node(());
    let n1 = graph.add_node(());
    let n2 = graph.add_node(());
    let n3 = graph.add_node(());
    let t = graph.add_node(());
    graph.extend_with_edges(&[
      (s, n1, 10),
      (s, n2, 2),
      (n1, n2, 6),
      (n1, n3, 6),
      (n3, n2, 3),
      (n2, t, 5),
      (n3, t, 8),
    ]);
    assert_eq!(super::ford_fulkerson(s, t, &graph), 11);
    assert_eq!(super::dinic(s, t, &graph), 11);
  }
}
