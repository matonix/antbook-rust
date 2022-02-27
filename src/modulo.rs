// モジュロ演算のサポート
// https://en.wikipedia.org/wiki/Modular_arithmetic#Integers_modulo_n
// https://en.wikipedia.org/wiki/Multiplicative_group_of_integers_modulo_n
// 参考: 万能 int 型 https://qiita.com/qryxip/items/91355125e6bf2897bfeb
use num::traits::Pow;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

// v ∈ [0, m) && m ∈ [0, u32::MAX)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mod {
  v: usize,
  m: usize,
}

#[rustfmt::skip] impl Mod { pub fn create(self, v: usize) -> Mod { Mod { v: v % self.m, m: self.m, } } }

#[rustfmt::skip] impl Add for Mod { type Output = Self; fn add(self, rhs: Self) -> Self { Mod { v: (self.v + rhs.v) % self.m, m: self.m } } }
#[rustfmt::skip] impl Sub for Mod { type Output = Self; fn sub(self, rhs: Self) -> Self { Mod { v: (self.v + self.m - rhs.v) % self.m, m: self.m } } }
#[rustfmt::skip] impl Mul for Mod { type Output = Self; fn mul(self, rhs: Self) -> Self { Mod { v: (self.v * rhs.v) % self.m, m: self.m } } }

// べき乗: 繰り返し二乗法 O(log n)
// 使用は 2_6_carmichael_numbers にて
impl Pow<usize> for Mod {
  type Output = Self;
  fn pow(self, rhs: usize) -> Mod {
    let mut n = rhs;
    let mut x = self;
    let mut res = self.create(1);
    while n > 0 {
      if n & 1 == 1 {
        res = res * x;
      }
      x = x * x;
      n >>= 1;
    }
    res
  }
}

#[rustfmt::skip] impl Pow<Mod> for Mod { type Output = Self; fn pow(self, rhs: Mod) -> Mod { self.pow(rhs.v) } }

// 除算は m と割る数が互いに素でないとできない: https://www.creativ.xyz/modulo-basic/
// ということで Z/mZ 上での逆元との積を計算することになる: フェルマーの小定理

// #[rustfmt::skip] impl Div for Mod { type Output = Self; fn div(self, rhs: Self) -> Self { Mod(self.0 / rhs.0) } }
// #[rustfmt::skip] impl Rem for Mod { type Output = Self; fn rem(self, rhs: Self) -> Self { Mod(self.0 % rhs.0) } }

pub struct ModFactory {
  m: usize,
}

impl ModFactory {
  #[rustfmt::skip]  pub fn new(m: usize) -> ModFactory { ModFactory { m } }
  #[rustfmt::skip]  pub fn create(&self, v: usize) -> Mod { Mod { v: v % self.m, m: self.m, } }
}
