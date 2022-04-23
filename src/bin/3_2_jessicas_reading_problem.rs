use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
  input! {
    p: usize,
    a: [usize; p],
  }
  println!("{}", solve(p, a));
}

#[derive(Debug)]
struct MultiSet<K> {
  internal: HashMap<K, usize>,
}
impl<K> MultiSet<K>
where
  K: Eq + Hash,
{
  fn new() -> Self {
    MultiSet {
      internal: HashMap::new(),
    }
  }
  fn insert(&mut self, value: K) {
    if let Some(&count) = self.internal.get(&value) {
      self.internal.insert(value, count + 1);
    } else {
      self.internal.insert(value, 1);
    }
  }
  fn remove(&mut self, value: K) {
    if let Some(&count) = self.internal.get(&value) {
      if count > 1 {
        self.internal.insert(value, count - 1);
      } else {
        self.internal.remove(&value);
      }
    } else {
      // remove operation failed
    }
  }
  fn len(&self) -> usize {
    self.internal.len()
  }
}

// 連続するページですべての事柄をカバー → 事柄の数を最大化するcontiguous subsequence
fn solve(n: usize, a: Vec<usize>) -> usize {
  let mut s = 0;
  let mut t = 0; // [s, t)
  let max = HashSet::<_>::from_iter(a.iter()).len();
  let mut acc = MultiSet::<usize>::new(); // メトリック(アキュムレータ)
  let mut ans = n; // 自明な解
  let insert = |acc: &mut MultiSet<usize>, e| acc.insert(e);
  let remove = |acc: &mut MultiSet<usize>, e| acc.remove(e);
  let cond = |acc: &MultiSet<usize>| acc.len() == max;
  while s < n {
    if cond(&acc) {
      ans = std::cmp::min(ans, t - s);
      remove(&mut acc, a[s]);
      s += 1;
    } else {
      if t == n {
        break;
      }
      t += 1;
      insert(&mut acc, a[t - 1]);
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let p = 5;
    let a = vec![1, 8, 8, 8, 1];
    assert_eq!(solve(p, a), 2);
  }
}
