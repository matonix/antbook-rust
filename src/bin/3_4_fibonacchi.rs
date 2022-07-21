use proconio::input;

fn main() {
  input! {
    n: usize,
  }
  println!("{}", solve(n));
}
fn solve(n: usize) -> usize {
  type Mat = [[usize; 2]; 2];
  let mat: Mat = [[1, 1], [1, 0]];
  fn mul(a: Mat, b: Mat) -> Mat {
    let mut c: Mat = [[0, 0], [0, 0]];
    for i in 0..2 {
      for k in 0..2 {
        for j in 0..2 {
          c[i][j] += a[i][k] * b[k][j];
        }  
      }
    }
    c
  }
  fn pow(a: Mat, n: usize) -> Mat {
    let mut n = n;
    let mut a: Mat = a;
    let mut b: Mat = [[1, 0], [0, 1]]; // I
    while n > 0 {
      if n & 1 == 1 {
        b = mul(b, a);
      }
      a = mul(a, a);
      n >>= 1;
    }
    b
  }
  pow(mat, n)[1][0] // 行列の左下
}

#[cfg(test)]
mod tests {
  use super::solve;

  #[test]
  fn example1() {
    let n = 10;
    assert_eq!(solve(n), 55);
  }
}
