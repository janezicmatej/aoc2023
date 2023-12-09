#![feature(pattern)]

pub mod parsers;
pub mod template;

use std::mem::swap;

pub fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

pub fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        swap(&mut min, &mut max)
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
