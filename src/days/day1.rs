use crate::fs;

pub fn part1() -> u64 {
    get_calories_of_elves()
        .into_iter()
        .max().unwrap()
}

pub fn part2() -> u64 {
    let mut cals = get_calories_of_elves();
    cals.sort_by_key(|a| std::cmp::Reverse(*a));

    cals[..3].into_iter().sum()
}

fn get_calories_of_elves() -> Vec<u64> {
    fs::get_input(1)
        .unwrap()
        .split("\n\n")
        .map(|packs| {
            packs.lines().fold(0, |sum, item| {
                sum + match item.parse::<u64>() {
                    Ok(num) => num,
                    Err(_) => return sum,
                }
            })
        })
        .collect()
}
