use std::collections::VecDeque;

use crate::fs;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multipy(u64),
    MultipyItself,
}

impl Operation {
    fn new_worry_level(&self, old_worry: u64) -> u64 {
        match self {
            Operation::Add(add) => old_worry + add,
            Operation::Multipy(multiplier) => old_worry * multiplier,
            Operation::MultipyItself => old_worry * old_worry,
        }
    }
}

#[derive(Debug)]
struct Monke {
    items: VecDeque<u64>,
    operation: Operation,
    test_div_by: u64,
    if_true: usize,
    if_false: usize,
}

impl Monke {
    fn from_str(str: &str) -> Monke {
        let mut lines = str.lines();
        lines.next();

        Monke {
            items: lines.next().unwrap()[18..]
                .split(", ")
                .filter_map(|num| num.parse().ok())
                .collect(),
            operation: match &lines.next().unwrap()[23..] {
                "* old" => Operation::MultipyItself,
                op if &op[..1] == "*" => Operation::Multipy(op[2..].parse().unwrap()),
                op if &op[..1] == "+" => Operation::Add(op[2..].parse().unwrap()),
                _ => panic!("wtf"),
            },
            test_div_by: lines.next().unwrap()[21..].parse().unwrap(),
            if_true: lines.next().unwrap()[29..].parse().unwrap(),
            if_false: lines.next().unwrap()[30..].parse().unwrap(),
        }
    }
}

pub fn part1() -> u64 {
    play_for(20, 3)
}

pub fn part2() -> u64 {
    play_for(10_000, 1)
}

fn play_for(rounds: usize, div_by: u64) -> u64 {
    let mut monkes = monkes();
    let mut activity = vec![0; monkes.len()];

    for _ in 0..rounds {
        play_round(&mut monkes, &mut activity, div_by);
    }

    let max = activity.iter().fold((0, 0), |mut max, cur| {
        if cur > &max.0 {
            max.1 = max.0;
            max.0 = *cur;
        } else if cur > &max.1 {
            max.1 = *cur;
        }

        max
    });

    max.0 * max.1
}

fn monkes() -> Vec<Monke> {
    let lines = fs::get_input(11).unwrap();
    let mut monkes = vec![];
    let mut tmp = vec![];

    for line in lines.lines() {
        if line.is_empty() {
            monkes.push(Monke::from_str(tmp.join("\n").as_str()));
            tmp.clear();
        } else {
            tmp.push(line);
        }
    }
    monkes.push(Monke::from_str(tmp.join("\n").as_str()));

    monkes
}

fn play_round(monkes: &mut [Monke], activity: &mut [u64], div_by: u64) {
    let common_div: u64 = monkes.iter().map(|m| m.test_div_by).product();

    for i in 0..monkes.len() {
        while let Some(item) = monkes[i].items.pop_front() {
            activity[i] += 1;
            let new_worry = (monkes[i].operation.new_worry_level(item) / div_by) % common_div;

            if new_worry % monkes[i].test_div_by == 0 {
                monkes[monkes[i].if_true].items.push_back(new_worry);
            } else {
                monkes[monkes[i].if_false].items.push_back(new_worry);
            }
        }
    }
}
