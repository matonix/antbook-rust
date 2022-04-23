use proconio::input;
use antbook::binary_search::BinarySearch;

fn main() {
  input! {
    n: usize,
    s: usize,
    a: [usize; n],
  }
  println!("{}", solve(n, s, a));
}

// 連続する部分列(contiguous subsequence)全体を調べるのは O(n^2): 始点 s と終点 t [s, t) の組み合わせ
// 連続する部分列に対する評価関数が、隣接要素の追加と削除をサポートできるならば、しゃくとり法が適用可能 O(n)
// ※ ある連続する部分列を S としたとき、 append(S, e) と remove(e, S) をサポートするという感じ（今回は単純な加減算）
// 2_3_multicombination.rs (p.67 重複組み合わせ)の式変形とかは実質しゃくとり法な気がする…。
fn solve(n: usize, sum: usize, a: Vec<usize>) -> usize {
  let mut s = 0;
  let mut t = 0; // [s, t)
  let mut acc = 0; // メトリック(アキュムレータ)
  let mut ans = n+1; // 存在しない解から始める
  while s < n { // 終了条件(の¬)
    if acc < sum { // 条件未達の場合
      if t == n { // 終了条件2
        break;
      }
      t += 1;
      acc += a[t-1]; // 隣接要素の追加
    } else { // 条件達成の場合
      ans = std::cmp::min(ans, t - s); // 解の更新
      acc -= a[s]; // 隣接要素の削除
      s += 1;
    }
  }
  if ans == n+1 {
    ans = 0
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 10;
    let s = 15;
    let a = vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8];
    assert_eq!(solve(n, s, a), 2);
  }
  #[test]
  fn example2() {
    let n = 5;
    let s = 11;
    let a = vec![1, 2, 3, 4, 5];
    assert_eq!(solve(n, s, a), 3);
  }
}
