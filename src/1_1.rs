use itertools::*;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: i32,
        k: [i32; n]
    }
    let ans = if solve(n, m, &k) { "Yes" } else { "no" };
    println!("{}", ans);
}

// O(n^2 log n^2)
fn solve(_n: usize, m: i32, k: &[i32]) -> bool {
    let prod: &Vec<i32> = &k
        .iter()
        .combinations(2)
        .map(|v| v.into_iter().sum())
        .sorted()
        .collect::<Vec<_>>();
    for i in prod {
        if let Ok(_) = prod.binary_search(&(m - i)) {
            return true;
        }
    }
    return false;
}

// O(n^3 log n)
fn _solve(_n: usize, m: i32, k: &[i32]) -> bool {
    for a in k {
        for b in k {
            for c in k {
                let d = m - a - b - c;
                if let Ok(_) = k.binary_search(&d) {
                    return true;
                }
            }
        }
    }
    return false;
}

// O(n^4)
fn __solve(_n: usize, m: i32, k: &[i32]) -> bool {
    for a in k {
        for b in k {
            for c in k {
                for d in k {
                    if a + b + c + d == m {
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example1() {
        assert_eq!(solve(3, 10, &[1, 3, 5]), true);
    }
    #[test]
    fn example2() {
        assert_eq!(solve(3, 9, &[1, 3, 5]), false);
    }
}
