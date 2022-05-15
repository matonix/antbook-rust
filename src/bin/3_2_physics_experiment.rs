use proconio::input;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    h: usize,
    r: usize,
    t: usize,
  }
  println!("{}", solve(n, h, r, t).iter().format_with(" ", |elt, f| f(&format_args!("{:.2}", elt))));
}

// https://www.desmos.com/calculator/2ff5k0gzkj
// 3_2_physics_experiment.png
fn solve(n: usize, h: usize, r: usize, t: usize) -> Vec<f64> {
  let h = h as f64;
  let r = r as f64;
  let g = 10.0;
  let p = f64::sqrt(2.0 * h / g);
  let f = |t: f64| t - 2.0 * f64::floor((t + 1.0) / 2.0);
  let k = |t: f64| p * f(t/p);
  let pos = |t: f64| h - 1.0 / 2.0 * g * k(t).powf(2.0);
  (0..n).map(|i| {
    let i = i as f64;
    let t = t as f64 - i;
    pos(t) + 2.0 * r * i / 100.0
  }).collect_vec()
}

#[cfg(test)]
mod tests {
  use super::solve;
  use assert_approx_eq::assert_approx_eq;

  #[test]
  fn example1() {
    let n = 1;
    let h = 10;
    let r = 10;
    let t = 100;
    vec![4.95].iter().zip(solve(n, h, r, t)).for_each(|(expect, actual)| assert_approx_eq!(expect, actual, 1.0e-2));
  }
  #[test]
  fn example2() {
    let n = 2;
    let h = 10;
    let r = 10;
    let t = 100;
    vec![4.95, 10.20].iter().zip(solve(n, h, r, t)).for_each(|(expect, actual)| assert_approx_eq!(expect, actual, 1.0e-2));
  }
}
