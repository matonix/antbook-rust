use petgraph::visit::*;
use petgraph::prelude::*;
use num::*;

#[allow(unused_variables)]
fn main() {
  let mut graph: Graph<_, usize, _, _> = Graph::new_undirected();
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

  let dists = warshall_floyd(&graph, 100000);
  println!("{:?}", dists);
  assert_eq!(dists[graph.to_index(a)][graph.to_index(d)], 7);
}

// 蟻本の実装
// 辺のコストをジェネリックにして、かつ、でかすぎない最大値をいい感じに見つける、というのが面倒なので、ユーザがinfを与えることにした
pub fn warshall_floyd<G>(graph: G, inf: G::EdgeWeight) -> Vec<Vec<G::EdgeWeight>>
where
  G: NodeCount + NodeIndexable + IntoEdgeReferences,
  G::EdgeWeight: Copy + Ord + Num
{
  let mut d = vec![vec![inf; graph.node_bound()]; graph.node_bound()];
  let ix = |i| graph.to_index(i);
  for i in 0..graph.node_count() {
    d[i][i] = <G::EdgeWeight>::zero();
  }
  for e in graph.edge_references() {
    d[ix(e.source())][ix(e.target())] = *e.weight();
  }
  for i in 0..graph.node_count() {
    for j in 0..graph.node_count() {
      for k in 0..graph.node_count() {
        d[i][j] = std::cmp::min(d[i][j], d[i][k] + d[k][j]);
      }
    }
  }
  d
}
