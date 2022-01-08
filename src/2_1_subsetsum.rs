use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {
      n: usize,
      a: [usize; n],
      k: usize
  }
  println!("{}", if solve(n, &a, k) { "Yes" } else { "no" });
}

fn solve(n: usize, a: &[usize], k: usize) -> bool {
  fn dfs(n: usize, a: &[usize], k: usize, i: usize, sum: usize) -> bool {
    if i == n {
      return sum == k;
    }
    return dfs(n, a, k, i + 1, sum + a[i]) || dfs(n, a, k, i + 1, sum);
  }
  dfs(n, a, k, 0, 0)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    assert_eq!(solve(4, &[1, 2, 4, 7], 13), true);
  }

  #[test]
  fn example2() {
    assert_eq!(solve(4, &[1, 2, 4, 7], 15), false);
  }
}
