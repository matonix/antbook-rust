use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    l: [f64; n]
  }
  println!("{}", solve(n, k, l));
}

// maximize: 紐の長さ d (ub - lb < 0.01)
// condition: d ずつ切って切り捨てた後、捨てられてない紐の本数が k 以上であること (lb側が満たす)
fn solve(_n: usize, k: usize, l: Vec<f64>) -> f64 {
  let mut lb = 0.0;
  let mut ub = *l.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap();
  let eps = 0.01;
  let cond = |d: f64| l.iter().map(|s| (s / d).floor() as usize).sum::<usize>() >= k;
  while ub - lb > eps {
    let mid = (ub + lb) / 2.0;
    if cond(mid) {
      lb = mid;
    } else {
      ub = mid;
    }
  }
  lb
}

#[cfg(test)]
mod tests {
  use super::solve;
  use assert_approx_eq::assert_approx_eq;

  #[test]
  fn example1() {
    let n = 4;
    let k = 11;
    let l = vec![8.02, 7.43, 4.57, 5.39];
    assert_approx_eq!(solve(n, k, l), 2.00, 1.0e-2);
  }
}
