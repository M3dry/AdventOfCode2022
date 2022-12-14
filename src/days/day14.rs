use std::collections::HashSet;

use crate::fs;

macro_rules! sand_movement {
    ($board:ident, $sand:ident, $count:ident, $label:tt) => {
        if !$board.contains(&($sand.0, $sand.1 + 1)) {
            $sand = ($sand.0, $sand.1 + 1);
        } else if !$board.contains(&($sand.0 - 1, $sand.1 + 1)) {
            $sand = ($sand.0 - 1, $sand.1 + 1);
        } else if !$board.contains(&($sand.0 + 1, $sand.1 + 1)) {
            $sand = ($sand.0 + 1, $sand.1 + 1);
        } else if $sand == (500, 0) {
            $count += 1;
            break $label;
        } else {
            $board.insert($sand);
            break;
        }
    };
}

pub fn part1() -> u64 {
    let mut board = get_grid().0;
    let mut count = 0;

    'o: loop {
        let mut sand = (500, 0);
        let mut repeat = 0;

        loop {
            sand_movement!(board, sand, count, 'o);

            if repeat > 1000 {
                break 'o;
            }

            repeat += 1;
        }

        count += 1;
    }

    count
}

pub fn part2() -> u64 {
    let board = get_grid();
    let floor = board.1 + 2;
    let mut board = board.0;
    let mut count = 0;

    'o: loop {
        let mut sand = (500, 0);

        loop {
            if sand.1 + 1 == floor {
                board.insert(sand);
                break
            }

            sand_movement!(board, sand, count, 'o);
        }

        count += 1;
    }

    count
}

fn get_grid() -> (HashSet<(u64, u64)>, u64) {
    let mut board: HashSet<(u64, u64)> = HashSet::new();
    let mut floor = 0;
    fs::get_input(14).unwrap().lines().for_each(|line| {
        for wall in line.split(" -> ").collect::<Vec<&str>>().windows(2) {
            let moves = (
                {
                    let m = wall[0]
                        .split(",")
                        .map(|m| m.parse().unwrap())
                        .collect::<Vec<u64>>();
                    floor = floor.max(m[1]);
                    (m[0], m[1])
                },
                {
                    let m = wall[1]
                        .split(",")
                        .map(|m| m.parse().unwrap())
                        .collect::<Vec<u64>>();
                    floor = floor.max(m[1]);
                    (m[0], m[1])
                },
            );

            if moves.0 .0 == moves.1 .0 {
                for i in moves.0 .1.min(moves.1 .1)..=moves.0 .1.max(moves.1 .1) {
                    board.insert((moves.0 .0, i));
                }
            } else if moves.0 .1 == moves.1 .1 {
                for i in moves.0 .0.min(moves.1 .0)..=moves.0 .0.max(moves.1 .0) {
                    board.insert((i, moves.0 .1));
                }
            } else {
                panic!("")
            }
        }
    });

    (board, floor)
}
