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

// めぐる式
// https://qiita.com/SakiKuroe/items/37246516767a692f1e4e#%E4%BA%8C%E5%88%86%E6%8E%A2%E7%B4%A2-binary-search-lower-bound-upper-bound

impl<T: Ord> BinarySearch<T> for [T] {
    // x 以上の要素のうち最初の要素の添字を返す。
    // xs には昇順ソート済みを想定する。
    fn lower_bound(&self, x: &T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if x <= &self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }

    // x より大きい要素のうち最初の要素の添字を返す。
    // xs には昇順ソート済みを想定する。
    fn upper_bound(&self, x: &T) -> usize {
        let mut ng = -1 as isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if x < &self[mid as usize] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
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
