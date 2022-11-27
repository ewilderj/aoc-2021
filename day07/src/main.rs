use itertools::min;
use itertools::max;
use memoize::memoize;
use std::cmp;

#[memoize]
fn triangle(n: u32) -> u32 {
    (1..=n).sum()
}

fn distance(a: u32, b:u32) -> u32 {
    cmp::max(a, b) - cmp::min(a, b)
}

fn compute(m: &Vec<u32>, f: fn(u32, u32) -> u32) -> u32 {
    let (lo, hi) = (*min(m.iter()).unwrap(), *max(m.iter()).unwrap());
    let mut od = u32::MAX;

    // if we go lo to hi at some point we reach the min distance
    for p in lo..=hi {
        let d = m.iter()
            .map(|x| f(*x, p))
            .sum();
        if d > od { // then p-1 is the best position, with distance od
            break;
        } else {
            od = d;
        }
    }
    return od;
}

fn main() {
    let m: Vec<u32> = include_str!("../input.txt")
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part1: {}", compute(&m, distance));
    println!("part2: {}", compute(&m, |a, b| { triangle(distance(a,b)) }));

    // my initial idea was to find the median, but didn't know how to do
    // that quickly in Rust. turns out slice::select_nth_unstable can do it
    // cf https://github.com/timvisee/advent-of-code-2021/blob/master/day07a/src/main.rs
    // quick way of doing the second one is to find the mean, and search around it
}
