use antbook::segtree::{Monoid, SegTree};
use itertools::izip;
use itertools::Itertools;
use num::Complex;
use proconio::input;
use std::f64::consts::PI;

fn main() {
  input! {
    n: usize,
    c: usize,
    l: [usize; n],
    s: [usize; c],
    a: [usize; n]
  }
  println!("{:?}", solve(n, c, l, s, a));
}

// クエリ a, b に対しては a..b のベクトル和を返し、 更新 i, x に対しては要素 i のベクトルを x にする（ベクトルの表現は複素数で持つ）
fn solve(_n: usize, _c: usize, l: Vec<usize>, s: Vec<usize>, a: Vec<usize>) -> Vec<(f64, f64)> {
  let z = l.iter().map(|&len| Node::new(0.0, Complex::new(0.0, len as f64))).collect_vec(); // ベクトルを表す複素数として初期化 (Im 軸沿い)
  let mut st = SegTree::new(z);
  let a = a.iter().map(|&deg| deg as f64 / 180.0 * PI - PI).collect_vec(); // 前のベクトルからどれだけ回転するか（前のベクトルと同じならゼロ）
  let mut ans = vec![];
  fn query(st: &mut SegTree<Node>, s: usize, a: f64) -> (f64, f64) {
    let n = st.query(s, s + 1); // 更新対象
    st.update(s, Node::new(a, Complex::from_polar(1.0, a) * n.z));
    let n = st.query(0, st.size());
    (n.z.re, n.z.im)
  }
  for (s, a) in izip!(s, a) {
    ans.push(query(&mut st, s, a));
  }
  ans
}

// モノイドを構成する解法: https://minus3theta.hatenablog.com/entry/2020/04/02/014949
// 和ベクトルの次のベクトルが刺さる向き θ と和ベクトル z の組
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
  theta: f64,
  z: Complex<f64>,
}

impl Monoid for Node {
  fn op(a: Self, b: Self) -> Self {
    Self {
      theta: a.theta + b.theta,
      z: a.z + Complex::from_polar(1.0, a.theta) * b.z,
    }
  }
  fn e() -> Self {
    Self {
      theta: 0.0,
      z: Complex::new(0.0, 0.0),
    }
  }
}

impl Node {
  fn new(theta: f64, z: Complex<f64>) -> Self {
    Self { theta, z }
  }
}

// ラジアン極座標系 (r, θ) を扱う
// 愚直実装 O(NC)
fn _solve(n: usize, _c: usize, l: Vec<usize>, s: Vec<usize>, a: Vec<usize>) -> Vec<(f64, f64)> {
  let r = l.iter().map(|&len| len as f64).collect_vec();
  let mut t = vec![PI / 2.0; n];
  let a = a.iter().map(|&deg| deg as f64 / 180.0 * PI).collect_vec(); // ラジアンへの変換
  let s = s.iter().map(|&idx| idx - 1).collect_vec(); // 0-indexed
  let mut ans = vec![];
  fn query(r: &Vec<f64>, t: &mut Vec<f64>, s: usize, a: f64) -> (f64, f64) {
    let rot = a + t[s] - PI - t[s + 1]; // 回転後を θ'_i+1 とすると、 a_i = π - θ_i + θ'_i+1 となる。ここで求めたいのは変化量 θ'_i+1 - θ_i+1
    t[s + 1..].iter_mut().for_each(|rad| *rad += rot);
    izip![r, t].fold((0.0, 0.0), |(x, y), (r, t)| {
      (x + r * t.cos(), y + r * t.sin())
    })
  }
  for (s, a) in izip!(s, a) {
    ans.push(query(&r, &mut t, s, a));
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;
  use assert_approx_eq::assert_approx_eq;
  use itertools::izip;

  #[test]
  fn example1() {
    let n = 2;
    let c = 1;
    let l = vec![10, 5];
    let s = vec![1];
    let a = vec![90];
    let expect = vec![(5.00, 10.00)];
    let actual = solve(n, c, l, s, a);
    for ((x_a, y_a), (x_e, y_e)) in izip!(actual, expect) {
      assert_approx_eq!(x_a, x_e);
      assert_approx_eq!(y_a, y_e);
    }
  }
  #[test]
  fn example2() {
    let n = 3;
    let c = 2;
    let l = vec![5, 5, 5];
    let s = vec![1, 2];
    let a = vec![270, 90];
    let expect = vec![(-10.00, 5.00), (-5.00, 10.00)];
    let actual = solve(n, c, l, s, a);
    for ((x_a, y_a), (x_e, y_e)) in izip!(actual, expect) {
      assert_approx_eq!(x_a, x_e);
      assert_approx_eq!(y_a, y_e);
    }
  }
}
