use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
  input! {
    n: usize,
    s: Chars
  }
  println!("{}", solve(n, s))
}

fn solve(n: usize, s: Vec<char>) -> String {
  let mut s: VecDeque<char> = VecDeque::from(s);
  let mut z: VecDeque<char> = s.clone().into_iter().rev().collect();
  let mut t = Vec::new();
  // 1 要素の比較だと同じ値のときにもう １ 歩深く比較する必要が出てしまうので、初めから辞書順で比較する。
  for _ in 0..n {
    println!("{:?} vs {:?}", s, z);
    if s <= z {
      t.push(s.pop_front().unwrap());
    } else {
      t.push(z.pop_front().unwrap());
    }
  }
  t.iter().collect()
}

#[cfg(test)]
mod tests {
  use super::solve;
  use proconio::input;
  use proconio::marker::Chars;
  use proconio::source::auto::AutoSource;

  #[test]
  fn example1() {
    let n = 6;
    let source = AutoSource::from("ACDBCB");
    input! {
      from source,
      mut s: Chars
    }
    assert_eq!(solve(n, s), "ABCBCD")
  }
}
