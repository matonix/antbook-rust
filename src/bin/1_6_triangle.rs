use std::cmp::Reverse;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n]
    }
    println!("{}", solve(n, &mut a));
}

fn solve(n: usize, a: &mut [i32]) -> i32 {
    a.sort_by_key(|w| Reverse(*w));
    for i in 0..n - 2 {
        if a[i] < a[i + 1] + a[i + 2] {
            return a[i] + a[i + 1] + a[i + 2];
        }
    }
    return 0;
}

fn _solve(n: usize, a: &mut [i32]) -> i32 {
    a.sort_by_key(|w| Reverse(*w));
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] < a[j] + a[k] {
                    return a[i] + a[j] + a[k];
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example1() {
        assert_eq!(solve(5, &mut [2, 3, 4, 5, 10]), 12);
    }
    #[test]
    fn example2() {
        assert_eq!(solve(4, &mut [4, 5, 10, 20]), 0);
    }
}
