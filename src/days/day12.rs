use std::collections::VecDeque;

use crate::fs;


fn grid_from_str(str: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let mut end = (0, 0);

    (
        str.lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(o, c)| match c {
                        'S' => 'a' as u8 - 1,
                        'E' => {
                            end = (i, o);
                            'z' as u8 + 1
                        },
                        c if c.is_ascii_lowercase() => c as u8,
                        _ => panic!("this shouldn't happen"),
                    })
                    .collect()
            })
            .collect(),
        end,
    )
}

pub fn part1() -> u64 {
    let (grid, pos) = grid_from_str(&fs::get_input(12).unwrap());
    navigate(&grid, pos, 'a' as u8 - 1)
}

pub fn part2() -> u64 {
    let (grid, pos) = grid_from_str(&fs::get_input(12).unwrap());
    navigate(&grid, pos, 'a' as u8)
}

fn navigate(grid: &Vec<Vec<u8>>, pos: (usize, usize), start: u8) -> u64 {
    let (max_x, max_y) = (grid.len(), grid[0].len());
    let mut path_lengts: Vec<Vec<Option<u64>>> = vec![vec![None; max_y]; max_x];
    let mut deque = VecDeque::from([(pos.0, pos.1, 0u64)]);

    path_lengts[pos.0][pos.1] = Some(0);

    while let Some((new_x, new_y, new_length)) = deque.pop_front() {
        if grid[new_x][new_y] == start {
            return new_length;
        }

        let height = grid[new_x][new_y];
        let unexplored = adjacent_indices(new_x, new_y, max_x, max_y)
            .into_iter()
            .filter(|(x, y)| grid[*x][*y] >= height - 1 && path_lengts[*x][*y] == None)
            .collect::<Vec<_>>();

        for &(x, y) in unexplored.iter() {
            path_lengts[x][y] = Some(new_length + 1);
            deque.push_back((x, y, new_length + 1));
        }
    }

    panic!("h");
}

fn adjacent_indices(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::with_capacity(4);

    if x > 0 {
        v.push((x - 1, y));
    }
    if x < max_x - 1 {
        v.push((x + 1, y));
    }
    if y > 0 {
        v.push((x, y - 1));
    }
    if y < max_y - 1 {
        v.push((x, y + 1));
    }

    v
}
