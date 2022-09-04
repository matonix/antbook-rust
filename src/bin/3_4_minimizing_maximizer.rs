use proconio::input;
use antbook::{RMQ, SegTree};

fn main() {
  input! {
    n: usize,
    m: usize,
    sts: [(usize, usize); m],
  }
  println!("{}", solve(n, m, sts));
}
fn solve(n: usize, _m: usize, sts: Vec<(usize, usize)>) -> usize {
  let mut v = vec![usize::MAX/2; n];
  v[0] = 0;
  let mut st = SegTree::<RMQ<usize>>::new(v);
  for (s, t) in sts {
    let s = s - 1;
    let t = t - 1;
    let orig = st.query(t, t+1);
    st.update(t, orig.min(st.query(s, t) + 1))
  }
  st.query(n - 1, n)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 40;
    let m = 6;
    let sts = vec![(20, 30), (1, 10), (10, 20), (20, 30), (15, 25), (30, 40)];
    assert_eq!(solve(n, m, sts), 4);
  }
}
