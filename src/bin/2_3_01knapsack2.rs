use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    n: usize,
    wvs: [(usize, usize); n],
    w: usize,
  }
  println!("{}", solve(n, wvs, w));
}

// 重さの総和の最小値 f(n, j) が w_max 以下となるような最大の j <= sup vs
// 初期条件 
// f(i, j) = | 0      if n == 0 /\ j == 0 ※最初の最初だけ正しい解を与えておく
//           | sup ws otherwise           ※まだ解がない部分は取りうる最大の値としておく
// 漸化式
// f(i+1, j) = | min(f(i, j - v[i+1]) + w[i+1], f(i, j)) if j >= v[i+1] 
//             | f(i, j)                                 otherwise

fn solve(n: usize, wvs: Vec<(usize, usize)>, w_max: usize) -> usize {
  let v_sup: usize = wvs.iter().map(|(_, v)| v).sum(); // アイテムの価値の総和（価値の上限）
  let w_sup: usize = wvs.iter().map(|(w, _)| w).sum(); // アイテムの重さの総和（重さの上限） ※雑に usize::MAX にしたらOverflowしてしまったので適当に十分大きい値を与えた
  let mut dp = vec![vec![w_sup; v_sup+1]; n+1];
  dp[0][0] = 0;
  for (i, &(w, v)) in wvs.iter().enumerate() {
    for j in 0..=v_sup {
      dp[i+1][j] = if j >= v { min(dp[i][j - v] + w, dp[i][j]) } else { dp[i][j] };
    }
  }
  // 後ろから線形探索
  for j in (1..v_sup).rev() {
    if dp[n][j] <= w_max {
      return j
    }
  }
  usize::MAX // 到達しない（はず）
}

// 余談：DPを書くときの考え方
// 1. 1つの概念（「アイテム」とか「価値」とか）だけで貪欲に目的関数の最適化（最小化/最大化）ができず、あちらを立てればこちらが立たずな状況が生まれうることを確認したら、DPを使うことを検討する
// 2. 2つ以上の概念を使って最適化すべき値は何かを考える。プロコンの場合、メタ戦略として直積をとってTLEしない程度の計算量となるような制約の組み合わせを添字に使うことを検討する（裏を返せば添字に使うとTLEになりそうな値を最適化する）
// 3. ループの順序を考える。概念に依存関係がある（任意の「価値」の場合を考慮して「アイテム」を選ばないといけない）場合は、依存される側を外側に持ってくる。（後でカンが外れて入れ替えることはあり得るけど…）
// 4. 漸化式を考える。（人によるけど自分は）「f(i, j - v[i+1]) + w[i+1]」「f(i, j)」のように最適値の候補を挙げて、目的の最適化（最大化/最小化)をする。（つまり再帰部分から考える）
// 5. うっかり配列外参照したり、初期条件に解が無いみたいな状況を回避するように条件式とか配列の初期化をイジる。

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 4;
    let wvs = vec![(2, 3), (1, 2), (3, 4), (2, 2)];
    let w = 5;
    assert_eq!(solve(n, wvs, w), 7);
  }
}
