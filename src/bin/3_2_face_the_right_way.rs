use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;

fn main() {
  input! {
    n: usize,
    a: Chars,
  }
  let (k, m) = solve(n, a);
  dbg!(k, m);
}

// contiguous subsequenceへの操作
// 最小化: 操作回数 m
// 次に最小化: 操作する連続列の長さ k (<= n)
// 方針: 各 k に対して m_k を算出して、 最小の m を見つける。複数ある場合は k が最小のものを選ぶ
// ｋ のループ → N 上の初期位置 i に対するループ → k 内での反転操作 で O(n^3) だが、 10^9 を超えるのでアウト
// 解より: dp[i]: [i, i+k-1] を反転させた回数（元の向きと合わせて偶数ならOK）
// 反転判定: 自身 i が含まれうるレンジ dp[i - 1], dp[i - 2], .. dp[i - k + 1] における反転回数の総和 + a[i] が奇数の場合、反転する。
// これをしゃくとりっぽく acc + dp[i - 1] - dp[i - k + 1] としてあげると総和のためのループを避けられる
fn solve(n: usize, a: Vec<char>) -> (usize, usize) {
  let sup = n+1;
  let mut ms = vec![sup; n];
  let a = a.iter().rev().map(|&c| if c == 'F' { 0 } else { 1 }).collect_vec();
  for k in 1..=n { // 列の長さ
    let mut acc = 0; // 区間内での反転回数の和
    let mut cnt = 0; // 反転操作を行った数
    let mut dp = vec![0; n-k+1];
    for i in 0..=n-k {
      if (a[i] + acc) % 2 != 0 {
        // 反転する
        dp[i] = 1;
        cnt += 1;
      }
      let s = if i + 1 >= k { dp[i + 1 - k] } else { 0 };
      acc = acc + dp[i] - s;
    }
    // n-k+1..n について、すべて前を向いているかチェック
    let mut ok = true;
    for i in n-k+1..n {
      if (a[i] + acc) % 2 != 0 {
        ok = false;
        break; // 失敗
      }
      let s = if i + 1 >= k { dp[i - k + 1] } else { 0 };
      acc = acc - s;
    }
    if ok {
      ms[k] = cnt;
    }
  }
  let m = *ms.iter().min().unwrap();
  let k = ms.iter().position(|&e| e == m).unwrap();
  (k, m)
}

// BFを二進数として解釈し、kの反転操作をxorとして扱うことで末端のループをO(1)にできる。よって O(n^2)
// と思ったけど、N=5000とかなのでビットで表現できないわ・・・。

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 7;
    let a = "BBFBFBB".chars().collect();
    assert_eq!(solve(n, a), (3, 3));
  }
  #[test]
  fn example2() {
    let n = 11;
    let a = "BBBFFBFFBBB".chars().collect();
    assert_eq!(solve(n, a), (5, 3));
  }
}
