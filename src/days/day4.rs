use crate::fs;

struct Pair(u64, u64);

impl Pair {
    fn contains(&self, other: &Pair) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn overlap(&self, other: &Pair) -> bool {
        self.0 <= other.0 && self.1 >= other.0
    }
}

pub fn part1() -> u64 {
    fs::get_input(4).unwrap().lines().fold(0, |acc, line| {
        let (first, second) = line.split_once(',').unwrap();
        let p1 = {
            let (from, to) = first.split_once('-').unwrap();

            Pair(from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap())
        };
        let p2 = {
            let (from, to) = second.split_once('-').unwrap();

            Pair(from.parse().unwrap(), to.parse().unwrap())
        };

        if p1.contains(&p2) || p2.contains(&p1) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part2() -> u64 {
    fs::get_input(4).unwrap().lines().fold(0, |acc, line| {
        let (first, second) = line.split_once(',').unwrap();
        let p1 = {
            let (from, to) = first.split_once('-').unwrap();

            Pair(from.parse::<u64>().unwrap(), to.parse::<u64>().unwrap())
        };
        let p2 = {
            let (from, to) = second.split_once('-').unwrap();

            Pair(from.parse().unwrap(), to.parse().unwrap())
        };

        if p1.overlap(&p2) || p2.overlap(&p1) {
            acc + 1
        } else {
            acc
        }
    })
}
