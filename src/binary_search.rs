use proconio::{fastout, input};

// deprecated: supersliceを使うべき
// https://docs.rs/superslice/1.0.0/superslice/trait.Ext.html

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        xs: [usize; n]
    }
    println!("{:?}", xs.lower_bound(&x));
}
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    // x 以上の要素のうち最初の要素の添字を返す。
    // xs には昇順ソート済みを想定する。
    fn lower_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let mid = (left + right) / 2;
            if &self[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }

    // x より大きい要素のうち最初の要素の添字を返す。
    // xs には昇順ソート済みを想定する。
    fn upper_bound(&self, x: &T) -> usize {
        let mut left = 0;
        let mut right = self.len();
        while left != right {
            let mid = (left + right) / 2;
            if &self[mid] <= x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower1_found_exact() {
        assert_eq!([1, 2, 3, 4, 5].lower_bound(&3), 2);
    }
    #[test]
    fn lower2_found_exact_dup() {
        assert_eq!([1, 2, 3, 3, 3, 4, 5].lower_bound(&3), 2);
    }
    #[test]
    fn lower3_found_not_exact() {
        assert_eq!([1, 2, 4, 5].lower_bound(&3), 2);
    }
    #[test]
    fn lower4_not_found() {
        assert_eq!([1, 2, 2].lower_bound(&3), 3);
    }
    #[test]
    fn upper1_found_exact() {
        assert_eq!([1, 2, 3, 4, 5].upper_bound(&3), 3);
    }
    #[test]
    fn upper2_found_exact_dup() {
        assert_eq!([1, 2, 3, 3, 3, 4, 5].upper_bound(&3), 5);
    }
    #[test]
    fn upper3_found_not_exact() {
        assert_eq!([1, 2, 4, 5].upper_bound(&3), 2);
    }
    #[test]
    fn upper4_not_found() {
        assert_eq!([1, 2, 2].upper_bound(&3), 3);
    }
}
