use antbook::binary_search::BinarySearch;
use antbook::walk_2d::Walk2D;
use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    w: usize,
    h: usize,
    n: usize,
    x1: [usize; n],
    x2: [usize; n],
    y1: [usize; n],
    y2: [usize; n],
  }
  println!("{}", solve(w, h, n, x1, x2, y1, y2));
}

// 座標圧縮
// 1-indexed だけどそのまま受け取る（あとで -1 した座標も計算するので）
fn solve(w: usize, h: usize, n: usize, x1: Vec<usize>, x2: Vec<usize>, y1: Vec<usize>, y2: Vec<usize>) -> usize {
  // 座標圧縮
  let (x1, x2, w) = compress2d(x1, x2, w);
  let (y1, y2, h) = compress2d(y1, y2, h);
  // 座標構築
  let mut board = vec![vec![false; h]; w];
  for i in 0..n {
    for j in x1[i]..=x2[i] {
      for k in y1[i]..=y2[i] {
        board[j][k] = true;
      }
    }
  }
  let mut a = Walk2D::new(w, h, board);
  // 探索
  let mut cnt = 0;
  for i in 0..h {
    for j in 0..w {
      if !a[i][j] {
        cnt += 1;
        let mut q = a.next_4way(i, j);
        // bfs
        while !q.is_empty() {
          if let Some(p) = q.pop() {
            if !a[p] {
              a[p] = true; // visited
              q.append(&mut a.next_4way_p(p));
            }
          }
        }
      }
    }
  }
  cnt
}

fn compress2d(a: Vec<usize>, b: Vec<usize>, u: usize) -> (Vec<usize>, Vec<usize>, usize) {
  // 線分の終端の左右の空白を考慮する
  let d = |&x| vec![x - 1, x, x + 1];
  let coords = vec![a.iter().map(d).concat(), b.iter().map(d).concat()].concat().into_iter().sorted().dedup().filter(|&x| x > 0 && x <= u).collect_vec();
  let ac = a.iter().map(|x| coords.lower_bound(&x)).collect_vec();
  let bc = b.iter().map(|x| coords.lower_bound(&x)).collect_vec();
  (ac, bc, coords.len())
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let w = 10;
    let h = 10;
    let n = 5;
    let x1 = vec![1, 1, 4, 9, 10];
    let x2 = vec![6, 10, 4, 9, 10];
    let y1 = vec![4, 8, 1, 1, 6];
    let y2 = vec![4, 8, 10, 5, 10];
    assert_eq!(solve(w, h, n, x1, x2, y1, y2), 6);
  }
}
