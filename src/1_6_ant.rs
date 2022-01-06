use std::cmp::*;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: i32,
        n: usize,
        x: [i32; n]
    }
    let (min, max) = solve(l, n, &x);
    println!("min = {}", min);
    println!("max = {}", max);
}

fn solve(l: i32, _n: usize, x: &[i32]) -> (i32, i32) {
    let min = x.iter().map(|&x| min(x, l - x)).max().unwrap();
    let max = x.iter().map(|&x| max(x, l - x)).max().unwrap();
    return (min, max);
}

fn _solve(l: i32, _n: usize, x: &[i32]) -> (i32, i32) {
    let min = (l/2) - x.iter().map(|x| (x - (l/2)).abs()).min().unwrap();
    let max = (l/2) + x.iter().map(|x| (x - (l/2)).abs()).max().unwrap();
    return (min, max);
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn example1() {
        assert_eq!(solve(10, 3, &[2, 6, 7]), (4, 8));
    }
}
