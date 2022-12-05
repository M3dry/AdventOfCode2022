mod days;
mod fs;

use crate::days::*;

pub fn all() {
    println!("Day 1-1: {}", day1::part1());
    println!("Day 1-2: {}", day1::part2());
    println!("Day 2-1: {}", day2::part1());
    println!("Day 2-2: {}", day2::part2());
    println!("Day 3-1: {}", day3::part1());
    println!("Day 3-2: {}", day3::part2());
    println!("Day 4-1: {}", day4::part1());
    println!("Day 4-2: {}", day4::part2());
    println!("Day 5-1: {}", day5::part1());
    println!("Day 5-2: {}", day5::part2());
}
