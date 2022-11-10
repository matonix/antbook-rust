use crate::{shortest_path::bellman_ford, dijkstra, dijkstra_with_potential, dot_log};
use num::{traits::{NumAssign, WrappingAdd}, Bounded};
use petgraph::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

// paper: https://arxiv.org/pdf/1207.6381.pdf

// successive shortest path with bellman-ford                    cap cost
pub fn ssp_bf<W>(s: NodeIndex, t: NodeIndex, graph: &DiGraph<(), (W, W)>, flow_obj: W) -> Option<W>
where
  W: Copy + NumAssign + Bounded + Ord + Debug + WrappingAdd,
{
  // 初期残余グラフ
  let mut res = graph.clone();
  let mut revs = HashMap::new();
  for e in graph.edge_references() {
    let rev_e = res.add_edge(
      e.target(),
      e.source(),
      (W::zero(), W::zero() - e.weight().1),
    );
    revs.insert(e.id(), rev_e);
    revs.insert(rev_e, e.id());
  }
  let mut cost = W::zero();
  let mut flow_tot = W::zero();
  while flow_tot < flow_obj {
    // Bellman-ford で残余グラフ上の s-t 最短路を求める
    let g = res.filter_map(
      |_, n| Some(n),
      |_, e| if e.0.is_zero() { None } else { Some(e.1) },
    ); // 探索用コストグラフ
    if let Ok(paths) = bellman_ford(&g, s) {
      // s-t 最短路に沿って流量を決定
      let mut f = W::max_value();
      let mut curr = t;
      let mut path = vec![];
      while let Some(pred) = paths.1[curr.index()] {
        let cap = res
          .find_edge(pred, curr)
          .and_then(|e| {
            path.push(e);
            res.edge_weight(e)
          })
          .map_or(f, |w| w.0);
        f = f.min(cap);
        curr = pred;
      }
      if curr != s {
        // s-t パスが存在しない
        return None;
      }
      f = f.min(flow_obj - flow_tot); // 流しすぎないように
      flow_tot += f;
      cost += paths.0[t.index()] * f;
      // 残余グラフを更新
      for e in path.into_iter() {
        res.edge_weight_mut(e).unwrap().0 -= f;
        res.edge_weight_mut(revs[&e]).unwrap().0 += f;
      }
    } else {
      // Negative Cycle
      return None;
    }
  }
  Some(cost)
}

// successive shortest path with dijkstra                       cap cost
pub fn ssp_di<W>(s: NodeIndex, t: NodeIndex, graph: &DiGraph<(), (W, W)>, flow_obj: W) -> Option<W>
where
  W: Copy + NumAssign + Bounded + Ord + Debug + WrappingAdd,
{
  // // 初期ポテンシャル w/ dijkstra
  // let g = graph.filter_map(
  //   |_, n| Some(n),
  //   |_, e| if e.0.is_zero() { None } else { Some(e.1) },
  // ); // 探索用コストグラフ
  // let potentials = dijkstra(&g, s).0;

  // 初期残余グラフ
  let mut res = graph.map(|_, _| W::zero(), |_, e| *e);
  let mut revs = HashMap::new();
  for e in graph.edge_references() {
    let rev_e = res.add_edge(
      e.target(),
      e.source(),
      (W::zero(), W::zero() - e.weight().1),
    );
    revs.insert(e.id(), rev_e);
    revs.insert(rev_e, e.id());
  }

  dot_log::log(&res, 0);

  let mut cost = W::zero();
  let mut flow_tot = W::zero();
  while flow_tot < flow_obj {
    dot_log::log(&res, 1);
    // dijkstra で残余グラフ上の s-t 最短路を求める
    let g = res.filter_map(
      |_, n| Some(*n),
      |_, e| if e.0.is_zero() { None } else { Some(e.1) },
    ); // 探索用コストグラフ
    let paths = dijkstra_with_potential(&g, s);
    // ポテンシャルの更新
    for (i, w) in res.node_weights_mut().enumerate() {
      *w = paths.0[i]
    }
    // s-t 最短路に沿って流量を決定
    let mut f = W::max_value();
    let mut curr = t;
    let mut path = vec![];
    while let Some(pred) = paths.1[curr.index()] {
      let cap = res
        .find_edge(pred, curr)
        .and_then(|e| {
          path.push(e);
          res.edge_weight(e)
        })
        .map_or(f, |w| w.0);
      f = f.min(cap);
      curr = pred;
    }
    if curr != s {
      // s-t パスが存在しない
      return None;
    }
    f = f.min(flow_obj - flow_tot); // 流しすぎないように
    flow_tot += f;
    cost += paths.0[t.index()] * f;
    // 残余グラフを更新
    for e in path.into_iter() {
      res.edge_weight_mut(e).unwrap().0 -= f;
      res.edge_weight_mut(revs[&e]).unwrap().0 += f;
      dot_log::log(&res, 2);
    }
    dot_log::log(&res, 1);
  }
  dot_log::log(&res, 0);
  dot_log::write_dot();
  Some(cost)
}


mod tests {
  #[test]
  fn test() {
    let mut graph = super::Graph::new();
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
    assert_eq!(super::ssp_bf(s, t, &graph, 0), Some(0));
    assert_eq!(super::ssp_bf(s, t, &graph, 5), Some(39));
    assert_eq!(super::ssp_bf(s, t, &graph, 11), Some(102));
    assert_eq!(super::ssp_bf(s, t, &graph, 12), None);
    // assert_eq!(super::ssp_di(s, t, &graph, 0), Some(0));
    assert_eq!(super::ssp_di(s, t, &graph, 5), Some(39));
    // assert_eq!(super::ssp_di(s, t, &graph, 11), Some(102));
    // assert_eq!(super::ssp_di(s, t, &graph, 12), None);
  }
}
