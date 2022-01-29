use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
    m: [usize; n],
    k: usize,
  }
  println!("{}", if solve(n, a, m, k) { "Yes" } else { "no" });
}

// 構成可能かどうかの真理値 f(n, k)
// f(i + 1, j) = \/h={0..min(j/a[i+1], m[i+1])} f(i, j - h * a[i+1])
// 考察： 個数制限無しナップサックと類似して、 f(i + 1, j - (h - 1) * a[i+1]) が true ならば f(i + 1, j - h * a[i+1]) も true となるはず
// f(i + 1, j) = f(i + 1, j - a[i+1]) || f(i, j) if j/a[i] > 0 && j/a[i] <= m[i] ※1つ以上アイテムが置けて、置こうとするアイテムの数 j/a[i] が存在しているアイテムの数 m[i] を超えなければ置ける
//             = f(i, j) otherwise
// f(_, 0) = true
// f(_, _) = false
// 斜め方向の参照がないので、１次元テーブルで計算可能

//O(nK) = 10^8 かつかつ
fn solve(n: usize, a: Vec<usize>, m: Vec<usize>, k: usize) -> bool {
  let mut dp = vec![false; k+1];
  dp[0] = true;
  for i in 0..n {
    for j in 0..=k {
      if j/a[i] > 0 && j/a[i] <= m[i] {
        dp[j] = dp[j - a[i]] || dp[j]
      }
    }
  }
  dp[k]
}

// 一般にBool値DPは無駄があるらしく、同じ計算量でもっと情報量を増やせる → たとえば「作れる」「作れない」ではなく、作った時に「余る個数」とか（つくれないときは-1とかにする）。

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 3;
    let a = vec![3, 5, 8];
    let m = vec![3, 2, 2];
    let k = 17;
    assert_eq!(solve(n, a, m, k), true);
  }
}
