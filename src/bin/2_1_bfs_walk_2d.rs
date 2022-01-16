use proconio::marker::Chars;
use proconio::{fastout, input};
use std::iter;
use antbook::walk_2d::{Point, Walk2D};

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [Chars; n] // `a` is Vec<Vec<char>>, (n, m)-matrix.
  }
  println!("{}", solve(n, m, a));
}

fn solve(n: usize, m: usize, a: Vec<Vec<char>>) -> usize {
  let mut a = Walk2D::new(n, m, a);
  // find S
  if let Some(p) = a.find(|e| *e == 'S') {
    a[p] = '#'; // visited
    let mut q: Vec<(Point, usize)> = a.next_4way_p(p).into_iter().zip(iter::repeat(1)).collect();
    // bfs
    while !q.is_empty() {
      if let Some((p, t)) = q.pop() {
        if a[p] == 'G' {
          return t;
        } else if a[p] == '.' {
          a[p] = '#'; // visited
          q.append(
            &mut a
              .next_4way_p(p)
              .into_iter()
              .zip(iter::repeat(t + 1))
              .collect(),
          )
        }
      }
    }
  }
  panic!("not found")
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

    assert_eq!(solve(n, m, a), 22);
  }
}
