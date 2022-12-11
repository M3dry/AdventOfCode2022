use std::collections::VecDeque;

use crate::fs;

pub fn part1() -> i64 {
    let mut reg = 1;
    let mut cycle = 0;
    let mut signal = 0;

    for line in fs::get_input(10).unwrap().lines() {
        cycle += 1;
        if cycle % 40 == 20 {
            signal += cycle * reg;
        }
        if &line[..4] == "addx" {
            let add = &line[5..].parse::<i64>().unwrap();
            cycle += 1;
            if cycle % 40 == 20 {
                signal += cycle * reg;
            }
            reg += add;
        }
    }

    signal
}

pub fn part2() -> String {
    let mut screen: Vec<Vec<char>> = vec![vec![]];
    let mut screen_pos: (usize, usize) = (0, 0);
    let mut sprite_pos: (i32, i32) = (0, 1);
    let mut cycle = 0;

    for line in fs::get_input(10).unwrap().lines() {
        cycle += 1;
        draw_pixel(&mut screen, &mut screen_pos, &mut sprite_pos);
        if cycle == 40 {
            cycle = 0;
            screen_pos.1 = 0;
            screen_pos.0 += 1;
            sprite_pos.0 += 1;
            screen.push(vec![]);
        }

        if &line[..4] == "addx" {
            cycle += 1;
            draw_pixel(&mut screen, &mut screen_pos, &mut sprite_pos);

            let add = &line[5..].parse::<i32>().unwrap();
            sprite_pos.1 += *add;
            if cycle == 40 {
                cycle = 0;
                screen_pos.1 = 0;
                screen_pos.0 += 1;
                sprite_pos.0 += 1;
                screen.push(vec![]);
            }
        }
    }

    draw_screen(screen)
}

fn draw_pixel(screen: &mut Vec<Vec<char>>, screen_pos: &mut (usize, usize), sprite_pos: &mut (i32, i32)) {
    if (sprite_pos.1.is_positive()
        && (*screen_pos == (sprite_pos.0 as usize, sprite_pos.1 as usize - 1)
            || *screen_pos == (sprite_pos.0 as usize, sprite_pos.1 as usize)
            || *screen_pos == (sprite_pos.0 as usize, sprite_pos.1 as usize + 1)))
        || (*screen_pos == (sprite_pos.0 as usize, sprite_pos.1 as usize))
        || (*screen_pos == (sprite_pos.0 as usize, (sprite_pos.1 + 1) as usize))
    {
        screen[screen_pos.0].push('█');
    } else {
        screen[screen_pos.0].push(' ');
    }
    screen_pos.1 += 1;
}

fn draw_screen(screen: Vec<Vec<char>>) -> String {
    let inline = true;
    let hi = !inline;
    let mut buf = String::new();

    for row in screen {
        for col in row {
            buf.push(col);
        }
        buf.push('\n');
    }

    buf.pop();
    buf
}
