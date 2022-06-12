use num::{Zero, FromPrimitive, One};

// 区間に比例した作用素を持つ Lazy Segment tree (Range Sum Query 向け)
// s, t が与えられたとき、 [s, t] の最小値を求める O(log n)
// s, t, f が与えられたとき、 for each k in [s, t], a_k = f(k) O(log n)

pub trait RightActModule {
  type X;
  type M;
  fn opx(a: &Self::X, b: &Self::X) -> Self::X;
  fn ex() -> Self::X;
  fn opm(a: &Self::M, b: &Self::M) -> Self::M;
  fn em() -> Self::M;
  fn act(a: &Self::X, b: &Self::M) -> Self::X;
  fn mult(a: &Self::M, b: usize) -> Self::M;
}

#[derive(Debug)]
pub struct ProportionalLazySegTree<T: RightActModule> {
  nodes: Vec<T::X>, 
  lazys: Vec<T::M>, 
  n: usize, 
  leaves: usize
}

impl<T: RightActModule> ProportionalLazySegTree<T>
where
  T::X: Clone,
  T::M: Clone + PartialEq,
{
  pub fn new(v: Vec<T::X>) -> Self {
    let len = v.len();
    // n == x^2 かつ x は len < x^2 を満たす最小の x
    let mut n = 1;
    while len > n {
      n = n << 1;
    }
    // 2冪にする
    let mut vec = vec![vec![T::ex(); n - 1], v, vec![T::ex(); n - len]].concat();
    for k in (0..=n - 2).rev() {
      vec[k] = T::opx(&vec[k * 2 + 1], &vec[k * 2 + 2]);
    }
    let size = vec.len();
    ProportionalLazySegTree {
      nodes: vec,
      lazys: vec![T::em(); size], 
      n, 
      leaves: len
    }
  }

  // [s, t) の最小値を求める
  // 見つからない場合は T::ex() を返す
  pub fn query(&mut self, s: usize, t: usize) -> T::X {
    // k は接点の番号であり、 [l, r) に対応する
    self.query_rec(s, t, 0, 0, self.n)
  }

  fn query_rec(&mut self, s: usize, t: usize, k: usize, l: usize, r: usize) -> T::X {
    self.eval(k, r - l);
    if r <= s || t <= l { // [s, t), [l, r) に共通部分がない場合は終了
      T::ex()
    } else if s <= l && r <= t { // [s, t) が [l, r) を包摂する場合はノードの値を返す
      self.nodes[k].clone()
    } else {
      let vl = self.query_rec(s, t, k * 2 + 1, l, (l + r) / 2);
      let vr = self.query_rec(s, t, k * 2 + 2, (l + r) / 2, r);
      T::opx(&vl, &vr)
    }
  }

  pub fn update(&mut self, s: usize, t: usize, x: T::M) {
    self.update_rec(s, t, x, 0, 0, self.n)
  }

  fn update_rec(&mut self, s: usize, t: usize, x: T::M, k: usize, l: usize, r: usize) {
    self.eval(k, r - l);
    if s <= l && r <= t { // [s, t) が [l, r) を包摂する場合
      self.lazys[k] = T::opm(&self.lazys[k], &x);
      self.eval(k, r - l);
    } else if s < r && l < t { // [s, t), [l, r) に一部共通部分がある場合
      self.update_rec(s, t, x.clone(), k * 2 + 1, l, (l + r) / 2);
      self.update_rec(s, t, x, k * 2 + 2, (l + r) / 2, r);
      self.nodes[k] = T::opx(&self.nodes[k * 2 + 1], &self.nodes[k * 2 + 2]);
    }
  }

  fn eval(&mut self, k: usize, len: usize) {
    if self.lazys[k] == T::em() {
      return
    } else if k < self.n - 1 {
      self.lazys[k * 2 + 1] = T::opm(&self.lazys[k * 2 + 1], &self.lazys[k]);
      self.lazys[k * 2 + 2] = T::opm(&self.lazys[k * 2 + 2], &self.lazys[k]);
    }
    // M 上のスカラー乗法を用いた高速化
    self.nodes[k] = T::act(&self.nodes[k], &T::mult(&self.lazys[k], len));
    self.lazys[k] = T::em();
  }

  pub fn size(&self) -> usize {
    self.leaves
  }
}

// 具体的な右可群右作用付きモノイド

// Range Sum Query + Range Add Query
#[derive(Debug)]
pub struct RSQRAQ<T>(T);
impl<T> RightActModule for RSQRAQ<T>
where
  T: Zero + One + FromPrimitive + PartialEq + Clone, // Zero は Add 上に定義されている
{
  type X = T;
  type M = T;
  fn opx(a: &Self::X, b: &Self::X) -> Self::X {
    a.clone() + b.clone()
  }
  fn ex() -> Self::X {
    T::zero()
  }
  fn opm(a: &Self::M, b: &Self::M) -> Self::M {
    a.clone() + b.clone()
  }
  fn em() -> Self::M {
    T::zero()
  }
  fn act(a: &Self::X, b: &Self::M) -> Self::X {
    a.clone() + b.clone()
  }
  fn mult(a: &Self::M, b: usize) -> Self::M {
    a.clone() * T::from_usize(b).unwrap()
  }
}
