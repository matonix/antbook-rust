// use antbook::binary_indexed_tree::BIT;
use antbook::segtree::{Monoid, SegTree};
use proconio::input;

// 参考: http://hos.ac/slides/20140319_bit.pdf p.57~
// http://poj.org/problem?id=3468

enum Input {
  C(usize, usize, usize), // l, r, x
  Q(usize, usize),        // l, r
}

fn main() {
  input! {
    n: usize,
    q: usize,
    a: [usize; n]
  }
  let mut qs_ = vec![];
  for _ in 0..q {
    input! {
      query_type: char
    }
    if query_type == 'C' {
      input! {
        c: [usize; 3]
      }
      qs_.push(Input::C(c[0], c[1], c[2]))
    } else {
      input! {
        q: [usize; 2]
      }
      qs_.push(Input::Q(q[0], q[1]))
    }
  }
  println!("{:?}", solve(n, q, a, qs_));
}

// use segtree
fn solve(n: usize, q: usize, a: Vec<usize>, qs: Vec<Input>) -> Vec<usize> {
  let mut ans = vec![];
  // let mut bit = BIT::new(n);

  ans
}

// a. その節点の区間全体に一様に加えられた値
// b. その節点の区間に一様でなく加えられた値の和
#[derive(Debug, Clone, PartialEq)]
pub struct Node {
  a: usize,
  b: usize,
}

impl Monoid for Node {
  fn op(x: Self, y: Self) -> Self {
    Self {
      a: x.a + y.a,
      b: x.b + y.b,
    }
  }
  fn e() -> Self {
    Self { a: 0, b: 0 }
  }
}

impl Node {
  fn new(a: usize, b: usize) -> Self {
    Self { a, b }
  }
}

#[cfg(test)]
mod tests {
  use super::solve;
  use super::Input::*;

  #[test]
  fn example1() {
    let n = 10;
    let q = 5;
    let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let qs = vec![Q(4, 4), Q(1, 10), Q(2, 4), C(3, 6, 3), Q(2, 4)];
    let expect = vec![4, 55, 9, 15];
    assert_eq!(solve(n, q, a, qs), expect)
  }
}
