use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    x: [usize; n],
  }
  println!("{}", solve(n, m, x));
}

// maximize: 牛間の最小距離 d
// condition: d 以上の間隔を開けてすべての牛を収納できること (lb側が満たす)
fn solve(n: usize, m: usize, x: Vec<usize>) -> usize {
  let mut x = x;
  x.sort();
  let cond = |d: &usize| {
    let mut cnt = 0;
    let mut next = x[0];
    for y in &x {
      if y >= &next {
        cnt += 1;
        next = y + d;
      }
    }
    cnt >= m
  };
  // めぐる式 [ok, ng)
  // https://twitter.com/meguru_comp/status/697008509376835584/photo/4
  let mut ok = 0;
  let mut ng = x[n - 1];
  while ng - ok > 1 {
    let mid = (ok + ng) / 2;
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

  #[test]
  fn example1() {
    let n = 5;
    let m = 3;
    let x = vec![1, 2, 8, 4, 9];
    assert_eq!(solve(n, m, x), 3);
  }
}
