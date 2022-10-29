use antbook::minimum_cost_flow::ssp_bf;
use petgraph::Graph;

fn main() {
  let mut graph = Graph::new();
  let s = graph.add_node(());
  let v1 = graph.add_node(());
  let v2 = graph.add_node(());
  let v3 = graph.add_node(());
  let t = graph.add_node(());
  graph.extend_with_edges(&[
    (s, v1, (10, 2)),
    (s, v2, (2, 4)),
    (v1, v2, (6, 6)),
    (v1, v3, (6, 2)),
    (v2, t, (5, 2)),
    (v3, v2, (3, 3)),
    (v3, t, (8, 6)),
  ]);
  dbg!(&ssp_bf(s, t, &graph, 11));
}