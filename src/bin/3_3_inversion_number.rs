use antbook::binary_indexed_tree::BIT;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n]
  }
  println!("{:?}", solve(n, a));
}

// バブルソートの交換回数 = 反転数（転倒数: inversion number）を求める
// BIT 解: https://scrapbox.io/pocala-kyopro/%E8%BB%A2%E5%80%92%E6%95%B0
// 別解（分割統治法）: https://future-architect.github.io/articles/20210720a/

// i = 0..n に対して以下
//   i - (BIT の a_i までの和）を答えに加える → a_i より右にある 1 の個数 → a_i より大きい値の個数
//   BIT の場所 a_i に 1 を加算する → a_i 自身の位置に 1 を置く

fn solve(n: usize, a: Vec<usize>) -> usize {
  let mut ans = 0;
  let mut bit = BIT::new(n);
  for i in 0..n {
    ans += i - bit.sum(a[i]);
    bit.add(a[i], 1);
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let a = vec![3, 1, 4, 2];
    assert_eq!(solve(n, a), 3)
  }
}
