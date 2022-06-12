use antbook::binary_indexed_tree::BIT;
use proconio::input;
use antbook::{ProportionalLazySegTree, RSQRAQ};

// 参考(SegTree): https://algo-logic.info/segment-tree/#toc_id_6_5
// 入力例: http://poj.org/problem?id=3468
// 可視化: https://www.desmos.com/calculator/lnmk40eo1o
// bit0の役割: 範囲更新のレンジ外における累積和の算出
// bit1の枠割: 範囲更新のレンジ内における累積和の算出（インデックスが掛けられるので増加列になる）

enum Input {
  C(usize, usize, isize), // l, r, x
  Q(usize, usize),        // l, r
}

fn main() {
  input! {
    n: usize,
    q: usize,
    a: [isize; n]
  }
  let mut qs_ = vec![];
  for _ in 0..q {
    input! {
      query_type: char
    }
    if query_type == 'C' {
      input! {
        c: [isize; 3]
      }
      qs_.push(Input::C(c[0] as usize, c[1] as usize, c[2]))
    } else {
      input! {
        q: [usize; 2]
      }
      qs_.push(Input::Q(q[0], q[1]))
    }
  }
  println!("{:?}", solve(n, q, a, qs_));
}

// use bit
fn solve(n: usize, _q: usize, a: Vec<isize>, qs: Vec<Input>) -> Vec<isize> {
  let mut ans = vec![];
  let mut bit0 = BIT::<isize>::from(a); // 1-indexed
  let mut bit1 = BIT::<isize>::new(n);
  // i に関する一次式
  // let prefix_sum = |i| bit1.sum(i) * i as isize + bit0.sum(i);
  for q in qs {
    match q {
      Input::C(l, r, x) => {
        bit0.add(l, -x * (l as isize - 1));
        bit0.add(r + 1, x * r as isize);
        bit1.add(l, x);
        bit1.add(r + 1, -x);
      },
      Input::Q(l, r) => {
        let r_sum = bit1.sum(r) * r as isize + bit0.sum(r);
        let l_sum = bit1.sum(l - 1) * (l - 1) as isize + bit0.sum(l - 1);
        ans.push(r_sum - l_sum);
      }, 
    }
  }
  ans
}

// use segtree
fn _solve(_n: usize, _q: usize, a: Vec<isize>, qs: Vec<Input>) -> Vec<isize> {
  let mut ans = vec![];
  let mut st = ProportionalLazySegTree::<RSQRAQ<isize>>::new(a); // 0-indexed
  for q in qs {
    match q {
      Input::C(l, r, x) => st.update(l - 1, r, x),
      Input::Q(l, r) => ans.push(st.query(l - 1, r)), 
    }
  }
  ans
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
