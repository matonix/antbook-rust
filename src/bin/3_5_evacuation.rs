use fixedbitset::FixedBitSet;
use itertools::Itertools;
use petgraph::{prelude::*, visit::*};
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::iter;
use antbook::walk_2d::{Point, Walk2D};

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
  }
  println!("{:?}", solve(n, m, a));
}

fn solve(n: usize, m: usize, a: Vec<Vec<char>>) -> Option<usize> {
  let field = Walk2D::new(n, m, a.clone());
  let ds: Vec<Point> = field.find_all(|e| *e == 'D');
  let ps: Vec<Point> = field.find_all(|e| *e == '.');
  let mut dist: HashMap<(Point, Point), usize> = HashMap::new();
  
  // 各ドアからの最短距離
  fn bfs(s: Point, t: Point, mut a: Walk2D<char>, mut step: usize) -> Option<usize> {
    let mut q: Vec<(Point, usize)> = a.next_4way_p(s).into_iter().zip(iter::repeat(1)).collect();
    while !q.is_empty() && step > 0 {
      if let Some((next, dist)) = q.pop() {
        if next == t {
          return Some(dist);
        } else if a[next] == '.' {
          a[next] = '#'; // visited
          q.append(
            &mut a
              .next_4way_p(next)
              .into_iter()
              .zip(iter::repeat(dist + 1))
              .collect(),
          )
        }
      }
      step -= 1;
    }
    None
  }

  for d in ds.iter() {
    for p in ps.iter() {
      if let Some(di) = bfs(*d, *p, Walk2D::new(n, m, a.clone()), n*m) {
        dist.insert((*d, *p), di);
      }
    }
  }

  // グラフを作成
  let mut g = Graph::new_undirected();
  let p_nodes: HashMap<Point, NodeIndex> = ps.iter().map(|&p| (p, g.add_node(()))).collect();
  let dt_nodes: HashMap<(Point, usize), NodeIndex> = ds.iter().cartesian_product(0..n*m).map(|(&d, t)| ((d, t), g.add_node(()))).collect();
  for ((d, p), di) in dist.into_iter() {
    for t in di..n*m {
      g.add_edge(dt_nodes[&(d, t)], p_nodes[&p], ());
    }
  }

  // fn hopcroft_karp(g: &UnGraph<(), ()>) -> usize {
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

  let num_p = ps.len();
  let num_d = ds.len();
  let mut num = 0;
  let mut matching: HashMap<NodeIndex, NodeIndex> = HashMap::new();
  for (i, (v, _)) in g.node_references().enumerate() {
    if !matching.contains_key(&v) {
      let mut visit_map = g.visit_map();
      if dfs(v, &g, &mut visit_map, &mut matching) {
        num += 1;
        if num == num_p {
          return Some(i / num_d + 1);
        }
      }
    }
  }

  None
}

#[cfg(test)]
mod tests {
  use super::solve;
  use proconio::input;
  use proconio::marker::Chars;
  use proconio::source::auto::AutoSource;

  #[test]
  fn example1() {
    let n: usize = 5;
    let m: usize = 5;
    let source = AutoSource::from(
      r"
      XXDXX
      X...X
      D...X
      X...D
      XXXXX
      ",
    );
    input! {
      from source,
      mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
    }
    assert_eq!(solve(n, m, a), Some(3));
  }
  #[test]
  fn example2() {
    let n: usize = 5;
    let m: usize = 12;
    let source = AutoSource::from(
      r"
      XXXXXXXXXXXX
      X..........D
      X.XXXXXXXXXX
      X..........X
      XXXXXXXXXXXX
      ",
    );
    input! {
      from source,
      mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
    }
    assert_eq!(solve(n, m, a), Some(21));
  }
  #[test]
  fn example3() {
    let n: usize = 5;
    let m: usize = 5;
    let source = AutoSource::from(
      r"
      XDXXX
      X.X.D
      XX.XX
      D.X.X
      XXXDX
      ",
    );
    input! {
      from source,
      mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
    }
    assert_eq!(solve(n, m, a), None);
  }
}
