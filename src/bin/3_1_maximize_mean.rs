use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    wv: [(usize, usize); n],
  }
  println!("{}", solve(n, k, wv));
}

// maximize: 重さ当たりの価値 x = Σ_i v_i/w_i
// condition: 重さ当たりの価値 x 以上となるように k 個選ぶことができる (lb側が満たす)
// 解より： v_i - x*w_i の上位 k 個の和が 0 以上
fn solve(_n: usize, k: usize, wv: Vec<(usize, usize)>) -> f64 {
  let cond = |x: &f64| {
    wv.iter()
      .map(|wv| wv.1 as f64 - x * wv.0 as f64)
      .sorted_by(|a, b| b.partial_cmp(a).unwrap()) // 降順
      .take(k)
      .sum::<f64>()
      >= 0.0
  };
  // めぐる式 [ok, ng) (lb = ok)
  // https://twitter.com/meguru_comp/status/697008509376835584/photo/4
  let mut ok = 0 as f64;
  let mut ng = wv.iter().map(|wv| wv.1).sum::<usize>() as f64;
  let eps = 0.001;
  while ng - ok > eps {
    let mid = (ok + ng) / 2.0;
    if cond(&mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  ok
}

#[cfg(test)]
mod tests {
  use super::solve;
  use assert_approx_eq::assert_approx_eq;

  #[test]
  fn example1() {
    let n = 3;
    let k = 2;
    let wv = vec![(2, 2), (5, 3), (2, 1)];
    assert_approx_eq!(solve(n, k, wv), 0.75, 1.0e-2);
  }
}
