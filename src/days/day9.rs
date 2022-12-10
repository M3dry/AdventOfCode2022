use std::collections::HashSet;

use crate::fs;

pub fn part1() -> usize {
    simulate_rope(2)
}

pub fn part2() -> usize {
    simulate_rope(10)
}

fn simulate_rope(len: usize) -> usize {
    let mut seen = HashSet::<(i32, i32)>::new();
    let mut rope = vec![(0, 0); len];

    for line in fs::get_input(9).unwrap().lines() {
        let cmd = line.split_once(' ').unwrap();
        for _ in 0..cmd.1.parse::<i32>().unwrap() {
            calc_move_multi(&cmd.0[..1], &mut rope, &mut seen);
        }
    }

    seen.len()
}

fn calc_move_multi(dir: &str, rope: &mut [(i32, i32)], seen: &mut HashSet<(i32, i32)>) {
    let dir = match dir {
        "U" => (1, 0),
        "D" => (-1, 0),
        "R" => (0, 1),
        "L" => (0, -1),
        _ => panic!("wtf"),
    };
    rope[0].0 += dir.0;
    rope[0].1 += dir.1;

    for i in 1..rope.len() {
        let dx = rope[i - 1].1 - rope[i].1;
        let dy = rope[i - 1].0 - rope[i].0;

        if dx >= 2 || dx <= -2 || dy >= 2 || dy <= -2 {
            let dx = (-1).max(1.min(dx));
            let dy = (-1).max(1.min(dy));

            rope[i].1 += dx;
            rope[i].0 += dy;
        }
    }
    seen.insert(*rope.last().unwrap());
}
