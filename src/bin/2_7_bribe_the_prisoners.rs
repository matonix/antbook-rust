use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
  input! {
    p: usize,
    q: usize,
    a: [usize; q],
  }
  dbg!(solve(p, q, a));
}

// 貪欲法 O(q log q): より中心に近いものを先に選ぶことでその後の計算が悪化することはない
// 想定解よりだいぶ早いっぽいけど、あってる？？
fn solve(p: usize, _q: usize, a: Vec<usize>) -> usize {
  let mut emptiness = vec![0, p + 1];
  let mut cost = 0;
  let mut a = a;
  a.sort();
  while a.len() > 0 {
    // 一番中心に近い要素
    let center = if a.len() > 1 {
      let c_pos = a.lower_bound(&(p/2));
      let center_post = a[c_pos];
      let center_pre = a[c_pos - 1];
      if center_post - p/2 < p/2 - center_pre { center_post } else { center_pre }
    } else { // |a| == 1
      a[0]
    };
    let pos = emptiness.lower_bound(&center);
    // i の左側の空き部屋の番号 → i より小さい最大の数
    let left = emptiness[pos - 1];
    // i の右側の空き部屋の番号 → i より大さい最小の数 (狭義単調増加なのでlbでok)
    let right = emptiness[pos];
    // 支払額の合計を加算
    cost += right - left - 2;
    // center を空にする
    emptiness.insert(pos, center);
  }
  cost
}

// 全探索 O(q! q log q)
fn _solve(p: usize, q: usize, a: Vec<usize>) -> usize {
  let mut ans = usize::MAX;
  for a in a.into_iter().permutations(q) {
    let mut emptiness = vec![0, p + 1];
    let mut cost = 0;
    for i in a {
      let pos = emptiness.lower_bound(&i);
      // i の左側の空き部屋の番号 → i より小さい最大の数
      let left = emptiness[pos - 1];
      // i の右側の空き部屋の番号 → i より大さい最小の数 (狭義単調増加なのでlbでok)
      let right = emptiness[pos];
      // 支払額の合計を加算
      cost += right - left - 2;
      // i を空にする
      emptiness.insert(pos, i);
    }
    // ans と最小値比較
    ans = std::cmp::min(ans, cost);
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;
  #[test]
  fn example1() {
    let p = 8;
    let q = 1;
    let a = vec![3];
    assert_eq!(solve(p, q, a), 7)
  }
  #[test]
  fn example2() {
    let p = 20;
    let q = 3;
    let a = vec![3, 6, 14];
    assert_eq!(solve(p, q, a), 35)
  }
}
