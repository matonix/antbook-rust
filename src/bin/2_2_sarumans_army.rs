use proconio::input;
use antbook::binary_search::BinarySearch;

fn main() {
  input! {
    n: usize,
    r: usize,
    x: [usize; n]
  }
  println!("{}", solve(n, r, x));
}

// 方針1: 巻き込んだ点の数が多い順→同着のときは？→この点に抜けられると被覆できなくなるような点の数順
// 方針2: ヌケモレないようにできるだけ右の点からとっていく（左端から初めて、ある点から 2r 進んだ座標に一番近い左側の点）
// - これ以上右の点を選ぶと最小解でなくなる（左に１つ印が必要になる）

// 解: r 進んだ座標に一番近い左側の点をとってから、さらに r 進んだ座標に一番近い右側の点を次の探索開始地点とする

fn solve(n: usize, r: usize, x: Vec<usize>) -> usize {
  let mut cnt = 0;
  let mut left = 0;
  while left < n {
    // 点から r 進んだ座標 p に一番近い左側の点を探す
    let p = x[left] + r;
    let center = x.upper_bound(&p) - 1; // p より大きい点の一つ手前
    // さらに r 進んだ座標に一番近い右側の点を探す（r 進んだ座標は含まない）
    let q = x[center] + r;
    left = x.upper_bound(&q); // q より大きい点
    cnt += 1;
  }
  cnt
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 6;
    let r = 10;
    let x = vec![1, 7, 15, 20, 30, 50];
    assert_eq!(solve(n, r, x), 3);
  }
}
