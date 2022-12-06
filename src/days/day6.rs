use crate::fs;

pub fn part1() -> usize {
    msg(4)
}

pub fn part2() -> usize {
    msg(14)
}

fn msg(window: usize) -> usize {
    fs::get_input(6)
        .unwrap()
        .as_bytes()
        .windows(window)
        .position(|cs| !duplicate(cs))
        .unwrap()
        + window
}

fn duplicate<T: PartialEq + Eq + std::fmt::Debug>(arr: &[T]) -> bool {
    (1..arr.len()).any(|i| arr[i..].contains(&arr[i - 1]))
}
