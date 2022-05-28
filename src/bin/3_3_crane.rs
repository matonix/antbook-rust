use proconio::input;
use itertools::Itertools;
use itertools::izip;
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

// ラジアン極座標系 (r, θ) を扱う
// 愚直実装 O(NC)
fn solve(n: usize, _c: usize, l: Vec<usize>, s: Vec<usize>, a: Vec<usize>) -> Vec<(f64, f64)> {
  let r = l.iter().map(|&len| len as f64).collect_vec();
  let mut t = vec![PI/2.0; n];
  let a = a.iter().map(|&deg| deg as f64 / 180.0 * PI).collect_vec();
  let s = s.iter().map(|&idx| idx - 1).collect_vec();
  let mut ans = vec![];
  fn query(r: &Vec<f64>, t: &mut Vec<f64>, s: usize, a: f64) -> (f64, f64) {
    let rot = a + t[s] - PI - t[s+1]; // 回転後を θ'_i+1 とすると、 a_i = π - θ_i + θ'_i+1 となる。ここで求めたいのは変化量 θ'_i+1 - θ_i+1 
    t[s+1..].iter_mut().for_each(|rad| *rad += rot);
    izip![r, t].fold((0.0, 0.0), |(x, y), (r, t)| (x + r * t.cos(), y + r * t.sin()))
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
    for ((x_a, y_a), (x_e, y_e)) in izip!(solve(n, c, l, s, a), vec![(5.00, 10.00)]) {
      assert_approx_eq!(x_a, x_e);
      assert_approx_eq!(y_a, y_e);
    }
  }
}
