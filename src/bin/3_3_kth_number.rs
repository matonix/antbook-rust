use std::ops::Mul;

use itertools::Itertools;
use superslice::{self, Ext};
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; n],
    qs: [(usize, usize, usize); m],
  }
  println!("{:?}", solve(n, m, a, qs));
}

fn solve(n: usize, _m: usize, a: Vec<usize>, qs: Vec<(usize, usize, usize)>) -> Vec<usize> {
  let a_ = a.clone().into_iter().sorted().collect_vec();
  let b = (n as f32).log2().mul(n as f32).sqrt().floor() as usize; // √(n log n)
  let bs = a.clone().into_iter().chunks(b).into_iter().map(|c| c.sorted().collect_vec()).collect_vec();
  let mut ans = vec![];
  for (i, j, k) in qs {
    let i = i - 1; // 半開区間
    let bi = i / b;
    let bj = j / b;
    let i = a_.upper_bound_by(|x| {
      let mut cnt = 0;
      if bi == bj {
        cnt += a[i..=j].iter().filter(|&y| y < x).count();
      } else {
        for s in bi..=bj {
          if s == bi {
            cnt += a[i..=b*(bi+1)].iter().filter(|&y| y <= x).count();
          } else if s == bj {
            cnt += a[b*bj..j].iter().filter(|&y| y <= x).count();
          } else {
            cnt += bs[bi].upper_bound(&x);
          }
        } 
      }
      cnt.cmp(&k)
    });
    ans.push(a_[i]);
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 7;
    let m = 3;
    let a = vec![1, 5, 2, 6, 3, 7, 4];
    let qs = vec![(2, 5, 3), (4, 4, 1), (1, 7, 3)];
    let expect = vec![5, 6, 3];
    assert_eq!(solve(n, m, a, qs), expect)
  }
}
