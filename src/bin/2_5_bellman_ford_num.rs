// Num系トレイトを使った辺の重さの制約
use num::*;
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

  assert!(bellman_ford(&graph_with_neg_cycle, NodeIndex::new(0)).is_err());
}

pub fn bellman_ford<G>(
  g: G,
  source: G::NodeId,
) -> Result<(Vec<G::EdgeWeight>, Vec<Option<G::NodeId>>), NegativeCycle>
where
  G: NodeCount + IntoNodeIdentifiers + IntoEdges + NodeIndexable,
  G::EdgeWeight: Copy + PartialOrd + Num + Bounded,
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
