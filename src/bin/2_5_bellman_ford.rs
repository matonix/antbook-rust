// use petgraph::algo::bellman_ford; // これで一発
use petgraph::algo::*;
use petgraph::prelude::*;
use petgraph::visit::*;
use petgraph::Graph;

#[allow(unused_variables)]
fn main() {
  let mut g = Graph::new();
  let a = g.add_node(()); // node with no weight
  let b = g.add_node(());
  let c = g.add_node(());
  let d = g.add_node(());
  let e = g.add_node(());
  let f = g.add_node(());
  g.extend_with_edges(&[
    (0, 1, 2.0),
    (0, 3, 4.0),
    (1, 2, 1.0),
    (1, 5, 7.0),
    (2, 4, 5.0),
    (4, 5, 1.0),
    (3, 4, 1.0),
  ]);

  // Graph represented with the weight of each edge
  //
  //     2       1
  // a ----- b ----- c
  // | 4     | 7     |
  // d       f       | 5
  // | 1     | 1     |
  // \------ e ------/

  let path = bellman_ford(&g, a);
  assert_eq!(
    path,
    Ok((
      vec![0.0, 2.0, 3.0, 4.0, 5.0, 6.0],
      vec![None, Some(a), Some(b), Some(a), Some(d), Some(e)]
    ))
  );
  let graph_with_neg_cycle = Graph::<(), f32, Undirected>::from_edges(&[
    (0, 1, -2.0),
    (0, 3, -4.0),
    (1, 2, -1.0),
    (1, 5, -25.0),
    (2, 4, -5.0),
    (4, 5, -25.0),
    (3, 4, -1.0),
  ]);

  assert_eq!(find_negative_loop(&graph_with_neg_cycle), true);
}

// 蟻本の実装
pub fn bellman_ford<G>(
  g: G,
  source: G::NodeId,
) -> Result<(Vec<G::EdgeWeight>, Vec<Option<G::NodeId>>), NegativeCycle>
where
  G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
  G::EdgeWeight: FloatMeasure,
{
  let mut predecessor = vec![None; g.node_bound()];
  let mut distance = vec![<_>::infinite(); g.node_bound()];
  let ix = |i| g.to_index(i);
  distance[ix(source)] = <_>::zero();
  loop {
    let mut update = false;
    for e in g.edge_references() {
      let s = ix(e.source());
      let t = ix(e.target());
      let w = *e.weight();
      if distance[s] != <_>::infinite() && distance[t] > distance[s] + w {
        distance[t] = distance[s] + w;
        predecessor[t] = Some(e.source());
        update = true;
      }
    }
    if !update {
      break;
    }
  }
  Ok((distance, predecessor))
}

// 蟻本の実装（うまくいかない）
pub fn find_negative_loop<G>(
  g: G
) -> bool
where
  G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
  G::EdgeWeight: FloatMeasure,
{
  let mut distance = vec![<G::EdgeWeight>::zero(); g.node_bound()];
  let ix = |i| g.to_index(i);
  for i in 0..g.node_count() {
    for e in g.edge_references() {
      let s = ix(e.source());
      let t = ix(e.target());
      let w = *e.weight();
      if distance[t] > distance[s] + w {
        distance[t] = distance[s] + w;
        if i == g.node_count() - 1 {
          return true
        }
      }
    }
    println!("{:?}", distance);
  }
  false
}