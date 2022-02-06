use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
  input! {
    n: usize,
    k: usize,
    q: [(usize, usize, usize); k]
  }
  println!("{}", solve(n, k, q));
}

fn _solve(n: usize, _k: usize, q: Vec<(usize, usize, usize)>) -> usize {
  let mut ans = 0;
  let mut uf: UnionFind<usize> = UnionFind::new(3*n);
  for (t, x, y) in q {
    if 1 > x || x > n || 1 > y || y > n { // ng
      ans += 1;
      continue;
    }
    if t == 1 { // x と y が同じ種類の場合
      let mut b = false;
      for i in 0..3 {
        b |= uf.equiv(x+i*n, y+((i+1)%3)*n);
        b |= uf.equiv(x+((i+1)%3)*n, y+i*n);
      }
      if b { // ng
        ans += 1;
        continue;
      }
      for i in 0..3 { // x = y = a \/ x = y = b \/ x = y = c
        uf.union(x+i*n, y+i*n);
      }
    } else { // x が y を食べる場合
      let mut b = false;
      for i in 0..3 {
        b |= uf.equiv(x+i*n, y+i*n);
        b |= uf.equiv(x+((i+1)%3)*n, y+i*n);
      }
      if b { // ng
        ans += 1;
        continue;
      }
      for i in 0..3 { // x = a /\ y = b \/ x = b /\ y = c \/ x = c /\ y = a
        uf.union(x+i*n, y+((i+1)%3)*n);
      }
    }
  }
  ans
}

// 解答より: equiv においては a, b, c の区別は調べなくて良さそう
fn solve(n: usize, _k: usize, q: Vec<(usize, usize, usize)>) -> usize {
  let mut ans = 0;
  let mut uf: UnionFind<usize> = UnionFind::new(3*n);
  for (t, x, y) in q {
    if 1 > x || x > n || 1 > y || y > n { // ng
      ans += 1;
      continue;
    }
    if t == 1 { // x と y が同じ種類の場合
      if uf.equiv(x, y+n) || uf.equiv(x+n, y) { // ng
        ans += 1;
      } else {
        for i in 0..3 { // x = y = a \/ x = y = b \/ x = y = c
          uf.union(x+i*n, y+i*n);
        }
      }
    } else { // x が y を食べる場合
      if uf.equiv(x, y) || uf.equiv(x+n, y) { // ng
        ans += 1;
      } else {
        for i in 0..3 { // x = a /\ y = b \/ x = b /\ y = c \/ x = c /\ y = a
          uf.union(x+i*n, y+((i+1)%3)*n);
        }  
      }
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 100;
    let k = 7;
    let q = vec![
      (1, 101, 1),
      (2, 1, 2),
      (2, 2, 3),
      (2, 3, 3),
      (1, 1, 3),
      (2, 3, 1),
      (1, 5, 5),
    ];
    assert_eq!(solve(n, k, q), 3);
  }
}
