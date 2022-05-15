use antbook::binary_search::BinarySearch;
use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;

fn main() {
  input! {
    n: usize,
    ws: [usize; n],
    vs: [usize; n],
    w: usize,
  }
  println!("{}", solve(n, ws, vs, w));
}

// w も v も大きすぎるが、 n が小さめ → 2^40 は厳しいが 2^20 = 1048576 ならセーフ
// 半分全列挙 "Split and List" O(2^(n/2) n)
// よくわからんかった
// https://rustforbeginners.hatenablog.com/entry/huge-knapsack-dpl-1-h
fn solve(n: usize, ws: Vec<usize>, vs: Vec<usize>, w_max: usize) -> usize {
  let half = n / 2;
  let wvs = ws.into_iter().zip(vs).collect_vec();
  let l = powerset(wvs[..half].iter().collect_vec())
    .iter()
    .map(|s| s.iter().fold((0, 0), |(wa, va), (w, v)| (wa + w, va + v)))
    .collect_vec();
  // v については降順にしておくと簡単（らしい）
  let mut r = powerset(wvs[half..].iter().collect_vec())
    .iter()
    .map(|s| s.iter().fold((0, 0), |(wa, va), (w, v)| (wa + w, va + v)))
    .sorted_by_key(|&(w, v)| (w, Reverse(v)))
    .collect_vec();
  // r の不要な v の塗りつぶし (mapAccumでもよい(ない))
  let mut accum = 0;
  for i in 0..r.len() {
    r[i].1 = r[i].1.max(accum);
    accum = r[i].1;
  }
  // 二分探索
  let mut ans = 0;
  for (w, v) in l {
    if w <= w_max {
      let i = r.upper_bound(&(w_max - w, usize::MAX)) - 1; // 重さの条件を満たす中でもっとも右の要素（だから1を引く）
      ans = ans.max(v + r[i].1);
    }
  }
  ans
}

// https://stackoverflow.com/questions/40718975/how-to-get-every-subset-of-a-vector-in-rust
fn powerset<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
  (0..=items.len())
    .map(|count| items.clone().into_iter().combinations(count))
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let ws = vec![2, 1, 3, 2];
    let vs = vec![3, 2, 4, 2];
    let w = 5;
    assert_eq!(solve(n, ws, vs, w), 7);
  }
}
