// 再帰関数
pub fn fact(n: u32) -> u32 {
  if n == 0 {
    return 1;
  }
  return n * fact(n - 1);
}

#[cfg(test)]
mod tests_fact {
  use super::fact;

  #[test]
  fn fact0() {
    assert_eq!(fact(0), 1);
  }
  #[test]
  fn fact1() {
    assert_eq!(fact(1), 1);
  }
  #[test]
  fn fact10() {
    assert_eq!(fact(10), 3628800);
  }
}

pub fn fib(n: u32) -> u32 {
  if n <= 1 {
    return n;
  }
  return fib(n - 1) + fib(n - 2);
}

// メモ化再帰
pub fn fib_memo(n: usize) -> usize {
  const MAX_N: usize = 100 + 1;
  let mut memo: [usize; MAX_N] = [0; MAX_N];
  fn fib(n: usize, memo: &mut [usize; MAX_N]) -> usize {
    if n <= 1 {
      return n;
    }
    if memo[n] != 0 {
      return memo[n];
    }
    memo[n] = fib(n - 1, memo) + fib(n - 2, memo);
    return memo[n];
  }
  fib(n, &mut memo)
}

#[cfg(test)]
mod tests_fib {
  use super::fib_memo as fib;

  #[test]
  fn fib0() {
    assert_eq!(fib(0), 0);
  }
  #[test]
  fn fib1() {
    assert_eq!(fib(1), 1);
  }
  #[test]
  fn fib10() {
    assert_eq!(fib(10), 55);
  }
}

// スタック
// https://doc.rust-lang.org/std/collections/index.html#modules
// Vecが適当。スタックトップは最後の要素となることに注意。

pub fn stack() {
  println!("stack demo");
  let mut s = Vec::new();
  s.push(1); // push back
  s.push(2);
  s.push(3);
  println!("{:?}", s.last()); // peek 3
  s.pop(); // pop last
  println!("{:?}", s.last()); // peek 2
  s.pop();
  println!("{:?}", s.last()); // peek 1
  s.pop();
  println!("{:?}", s.last()); // empty
}

// キュー
// VecDequeが適当。双方向可能。

pub fn queue() {
  use std::collections::VecDeque;
  println!("queue demo");
  let mut q = VecDeque::new();
  q.push_back(1);
  q.push_back(2);
  q.push_back(3);
  println!("{:?}", q.front()); // 1
  q.pop_front();
  println!("{:?}", q.front()); // 2
  q.pop_front();
  println!("{:?}", q.front()); // 3
  q.pop_front();
  println!("{:?}", q.front()); // empty
}

fn main() {
  stack();
  queue()
}
