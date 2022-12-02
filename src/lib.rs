mod days;
mod fs;

use crate::days::*;

pub fn all() {
    println!("Day 1-1: {}", day1::part1());
    println!("Day 1-2: {}", day1::part2());
    println!("Day 2-1: {}", day2::part1());
    println!("Day 2-2: {}", day2::part2());
}
