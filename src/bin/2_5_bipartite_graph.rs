use petgraph::visit::Dfs;
use petgraph::Graph;
use proconio::input;

fn main() {
  input! {
    n: usize,
    e: usize,
    es: [(u32, u32); e]
  }
  println!("{}", if solve(n, e, es) { "Yes" } else { "No" });
}

// 適当な点からDFSしながら着色して矛盾なく辿れれればYes、そうでなければNo
// https://docs.rs/petgraph/0.5.1/petgraph/graph/struct.WalkNeighbors.html

fn solve(n: usize, _e: usize, es: Vec<(u32, u32)>) -> bool {
  let gr = Graph::<usize, u32>::from_edges(es);
  let mut cs = vec![0; n];
  // 0: not visited
  // 1: color 1
  // -1: color 2
  let start = gr.node_indices().find(|i| gr[*i] == 0).unwrap();
  let mut color = 1;
  cs[start.index()] = color;

  let mut dfs = Dfs::new(&gr, start);
  while let Some(node) = dfs.next(&gr) {
    // 全頂点訪問
    color *= -1; // 色の反転
    let mut neighbors = gr.neighbors_undirected(node).detach();
    while let Some(next) = neighbors.next_node(&gr) {
      // 隣接ノードの矛盾を確認
      if cs[next.index()] == 0 {
        cs[next.index()] = color;
      } else if cs[next.index()] != color {
        return false;
      }
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let e = 3;
    let es = vec![(0, 1), (1, 2), (2, 0)];
    assert_eq!(solve(n, e, es), false);
  }

  #[test]
  fn example2() {
    let n = 4;
    let e = 4;
    let es = vec![(0, 1), (1, 2), (2, 3), (3, 0)];
    assert_eq!(solve(n, e, es), true);
  }
}
