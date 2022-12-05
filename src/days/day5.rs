use std::collections::VecDeque;

use crate::fs;

pub fn part1() -> String {
    let lines = fs::get_input(5).unwrap();
    let mut lines = lines.lines().peekable();
    let mut crate_stacks: Vec<&str> = vec![];
    let mut stacks = Vec::<VecDeque<char>>::new();

    while let Some(line) = lines.peek() {
        if line.as_bytes()[1].is_ascii_digit() {
            for crate_stack in &crate_stacks {
                let mut o = 0;
                let chars = crate_stack.as_bytes();
                for i in (1..).step_by(4) {
                    if chars.len() <= i {
                        break;
                    }
                    if chars[i].is_ascii_alphabetic() {
                        while stacks.len() <= o {
                            stacks.push(VecDeque::new())
                        }
                        stacks[o].push_front(chars[i] as char);
                    }
                    o += 1
                }
            }
            lines.next();
            lines.next();
        } else if line.starts_with('m') {
            let mut nums = [0; 3];
            let mut i = 0;
            for c in lines.next().unwrap().chars() {
                if c.is_ascii_digit() {
                    nums[i] = nums[i] * 10 + c.to_digit(10).unwrap();
                } else if nums[i] != 0 {
                    i += 1;
                }
            }

            for _ in 0..nums[0] {
                let pop = stacks[nums[1] as usize - 1].pop_back().unwrap();
                stacks[nums[2] as usize - 1].push_back(pop);
            }
        } else {
            crate_stacks.push(lines.next().unwrap());
            continue;
        }
    }
    let mut ret = String::new();

    for mut stack in stacks {
        if let Some(pop) = stack.pop_back() {
            ret.push(pop)
        }
    }

    ret
}

pub fn part2() -> String {
    let lines = fs::get_input(5).unwrap();
    let mut lines = lines.lines().peekable();
    let mut crate_stacks: Vec<&str> = vec![];
    let mut stacks = Vec::<VecDeque<char>>::new();

    while let Some(line) = lines.peek() {
        if line.as_bytes()[1].is_ascii_digit() {
            for crate_stack in &crate_stacks {
                let mut o = 0;
                let chars = crate_stack.as_bytes();
                for i in (1..).step_by(4) {
                    if chars.len() <= i {
                        break;
                    }
                    if chars[i].is_ascii_alphabetic() {
                        while stacks.len() <= o {
                            stacks.push(VecDeque::new())
                        }
                        stacks[o].push_front(chars[i] as char);
                    }
                    o += 1
                }
            }
            lines.next();
            lines.next();
        } else if line.starts_with('m') {
            let mut nums = [0; 3];
            let mut i = 0;
            for c in lines.next().unwrap().chars() {
                if c.is_ascii_digit() {
                    nums[i] = nums[i] * 10 + c.to_digit(10).unwrap();
                } else if nums[i] != 0 {
                    i += 1;
                }
            }

            let mut stack = VecDeque::new();
            for _ in 0..nums[0] {
                stack.push_front(stacks[nums[1] as usize - 1].pop_back().unwrap())
            }
            stacks[nums[2] as usize - 1].append(&mut stack);
        } else {
            crate_stacks.push(lines.next().unwrap());
            continue;
        }
    }
    let mut ret = String::new();

    for mut stack in stacks {
        if let Some(pop) = stack.pop_back() {
            ret.push(pop)
        }
    }

    ret
}
