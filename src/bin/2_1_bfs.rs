use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
  }
  println!("{}", solve(n, m, &mut a));
}

fn solve(n: usize, m: usize, a: &mut Vec<Vec<char>>) -> usize {
  // find S
  if let Some((i, j)) = a
    .concat()
    .iter()
    .position(|&e| e == 'S')
    .map(|idx| (idx / n, idx % n))
  {
    a[i][j] = '#'; // visited
    let mut q: Vec<(usize, usize, usize)> = next_4way(i, j, 0, n, m);
    // bfs
    while !q.is_empty() {
      if let Some((i, j, t)) = q.pop() {
        if a[i][j] == 'G' {
          return t;
        } else if a[i][j] == '.' {
          a[i][j] = '#'; // visited
          q.append(&mut next_4way(i, j, t, n, m))
        }
      }
    }
  }
  panic!("not found")
}

fn next_4way(i: usize, j: usize, t: usize, n: usize, m: usize) -> Vec<(usize, usize, usize)> {
  // overflow 対策に wrapping_sub/add を使う。
  // 負の値にはならないので上限 n, m のみをチェックする (usizeの上限に近い値とかでなければ大丈夫)
  vec![
    (i.wrapping_sub(1), j, t + 1),
    (i.wrapping_add(1), j, t + 1),
    (i, j.wrapping_sub(1), t + 1),
    (i, j.wrapping_add(1), t + 1),
  ]
  .into_iter()
  .filter(|(i, j, _)| *i < n && *j < m)
  .collect()
}

#[cfg(test)]
mod tests {
  use super::solve;
  use proconio::input;
  use proconio::marker::Chars;
  use proconio::source::auto::AutoSource;

  #[test]
  fn example1() {
    let n: usize = 10;
    let m: usize = 10;
    // raw string を使った読み込み https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    let source = AutoSource::from(
      r"
      #S######.#
      ......#..#
      .#.##.##.#
      .#........
      ##.##.####
      ....#....#
      .#######.#
      ....#.....
      .####.###.
      ....#...G#
      ",
    );
    input! {
      from source,
      mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
    }
    print!("{:?}", a);

    assert_eq!(solve(n, m, &mut a), 22);
  }
}
