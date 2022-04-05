use proconio::input;

fn main() {
  input! {
    m: usize,
    p: f64,
    x: usize
  }
  dbg!(solve(m, p, x));
}

fn solve(m: usize, p: f64, x: usize) -> f64 {
  let m = m as u32;
  let g = 1000000;
  let mpow = 2usize.pow(m);
  let inf = g + 1;
  let mut ps = vec![(inf, 0.0); mpow + 1];
  ps[0] = (0, 0.0);
  ps[mpow] = (g, 1.0);
  for i in 1..=m {
    for j in (0..=mpow).step_by(2usize.pow(m - i as u32)) {
      let mut cand_p = vec![];
      for k in (0..=2usize.pow(m - 1)).step_by(2usize.pow(m - i as u32 + 1)) {
        // 半分だけ
        let l = (2 * j).wrapping_sub(k); // j を中心に k と対称な位置（配列外もありうる）: (l + k)/2 == j を満たす
        if l <= mpow {
          let ap = ps[k].1;
          let bp = ps[l].1;
          if k > l {
            cand_p.push(ap * p + bp * (1.0 - p));
          } else {
            cand_p.push(bp * p + ap * (1.0 - p));
          }
        }
      }
      let step = 2usize.pow(m - i as u32);
      let next_x = if ps[j].0 == inf { (ps[j + step].0 + ps[j - step].0) / 2 } else { ps[j].0 };
      ps[j] = (next_x, cand_p.iter().fold(ps[j].1, |m, v| v.max(m)));
    }
  }
  ps.iter().rev().find(|(y, _)| y <= &x).unwrap().1
}

#[cfg(test)]
mod tests {
  use super::solve;
  #[test]
  fn example1() {
    let m = 1;
    let p = 0.5;
    let x = 500000;
    assert_eq!(solve(m, p, x), 0.500000)
  }
  #[test]
  fn example2() {
    let m = 3;
    let p = 0.75;
    let x = 600000;
    assert_eq!(solve(m, p, x), 0.843750)
  }
}
