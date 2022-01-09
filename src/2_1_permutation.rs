use itertools::Itertools;

fn main() {
  // 辞書順の順列の列挙
  // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.permutations
  let perms = (5..8).permutations(2);
  itertools::assert_equal(perms, vec![
      vec![5, 6],
      vec![5, 7],
      vec![6, 5],
      vec![6, 7],
      vec![7, 5],
      vec![7, 6],
  ]);
}
