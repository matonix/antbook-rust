pub trait Heap<T>
where
  T: Clone + Ord,
{
  fn new() -> Self;
  fn peek(&self) -> Option<&T>;
  fn push(&mut self, item: T) -> ();
  fn pop(&mut self) -> Option<T>;
}

#[derive(Debug)]
struct BinaryHeap<T>(Vec<T>);

impl<T> Heap<T> for BinaryHeap<T>
where
  T: Clone + Ord,
{
  fn new() -> Self {
    BinaryHeap(Vec::new())
  }
  fn peek(&self) -> Option<&T> {
    self.0.last()
  }
  fn push(&mut self, item: T) {
    self.0.push(item);
    let mut i = self.0.len() - 1;
    while i > 0 {
      let p = (i - 1) / 2;
      if self.0[p] <= self.0[i] {
        break;
      }
      self.0.swap(i, p);
      i = p;
    }
  }
  fn pop(&mut self) -> Option<T> {
    let last = self.0.len() - 1;
    self.0.swap(0, last); // 削除対象を最後尾に
    if let Some(ret) = self.0.pop() {
      let mut i = 0;
      let size = self.0.len();
      while i * 2 + 1 < size {
        let mut a = i * 2 + 1;
        let b = i * 2 + 2;
        if b < size && self.0[b] < self.0[a] {
          a = b;
        }
        if self.0[a] >= self.0[i] {
          break;
        }
        self.0.swap(i, a);
        i = a;
      }
      Some(ret)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut h = BinaryHeap::new();
    h.push(6);
    h.push(4);
    h.push(2);
    h.push(5);
    h.push(1);
    h.push(3);
    assert_eq!(h.pop(), Some(1));
    assert_eq!(h.pop(), Some(2));
    assert_eq!(h.pop(), Some(3));
    assert_eq!(h.pop(), Some(4));
    assert_eq!(h.pop(), Some(5));
    assert_eq!(h.pop(), Some(6));
  }
}
