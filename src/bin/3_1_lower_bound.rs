use proconio::input;
use antbook::binary_search::BinarySearch;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
    k: usize
  }
  println!("{}", solve(n, a, k));
}

fn solve(_n: usize, a: Vec<usize>, k: usize) -> usize {
  a.lower_bound(&k)
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 5;
    let a = vec![2, 3, 3, 5, 6];
    let k = 3;
    assert_eq!(solve(n, a, k), 1);
  }
}
