use antbook::maximum_flow::dinic;
use petgraph::Graph;

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
}