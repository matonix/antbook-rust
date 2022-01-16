use proconio::input;
use std::cmp;

fn main() {
  input! {
    c: [usize; 6],
    mut a: usize
  }
  println!("{}", solve(c, a));
}

fn solve(c: Vec<usize>, mut a: usize) -> usize {
  let v = vec![1, 5, 10, 50, 100, 500];
  let mut cnt = 0;
  for i in (0..6).rev() {
    let need = a / v[i];
    let used = cmp::min(need, c[i]);
    cnt += used;
    a -= used * v[i];
  }
  cnt
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    assert_eq!(solve(vec![3, 2, 1, 3, 0, 2], 620), 6);
  }
}
