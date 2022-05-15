use itertools::Itertools;
use proconio::input;
use antbook::binary_search::BinarySearch;

fn main() {
  input! {
    n: usize,
    a: [i64; n],
    b: [i64; n],
    c: [i64; n],
    d: [i64; n],
  }
  println!("{}", solve(n, a, b, c, d));
}

// 半分全列挙 O(n^2 log(n^2))
fn solve(_n: usize, a: Vec<i64>, b: Vec<i64>, c: Vec<i64>, d: Vec<i64>) -> usize {
  // 0 - (a + b) = c + d となる c + d の左端と右端を探す
  let ab = a.iter().cartesian_product(b).map(|(x, y)| 0 - (x + y));
  let cd = c.iter().cartesian_product(d).map(|(x, y)| x + y).sorted().collect_vec();
  let mut cnt = 0;
  for v in ab {
    let l = cd.lower_bound(&v);
    let r = cd.upper_bound(&v);
    cnt += r - l;
  }
  cnt
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 6;
    let a = vec![-45, -41, -36, -36, 26, -32];
    let b = vec![22, -27, 53, 30, -38, -54];
    let c = vec![42, 56, -37, -75, -10, -6];
    let d = vec![-16, 30, 77, -46, 62, 45];
    assert_eq!(solve(n, a, b, c, d), 5);
  }
}
