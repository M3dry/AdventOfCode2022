use crate::fs;

macro_rules! view {
    ($iter:expr , $tree:ident, $ret:expr$(, $max:tt)?) => {{
        for i in $iter {
            $ret += 1;
            if $tree <= view!(@flow, i $(, $max)?) {
                break;
            }
        }
    }};
    (@flow, $i:ident, $max:tt) => {
        $i$max
    };
    (@flow, $i:ident) => {
        *$i
    };
}

pub fn part1() -> u64 {
    let grid = get_grid();
    (0..grid.len()).fold(0, |sum, row| {
        sum + (0..grid[row].len()).fold(0, |sum, col| {
            if !is_hidden(&grid, (row, col)) {
                sum + 1
            } else {
                sum
            }
        })
    })
}

pub fn part2() -> u32 {
    let grid = get_grid();
    (0..grid.len()).fold(0, |max, row| {
        max.max((0..grid[row].len()).fold(max, |max, col| max.max(view(&grid, (row, col)))))
    })
}

fn get_grid() -> Vec<Vec<u32>> {
    fs::get_input(8)
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn is_hidden(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> bool {
    let height = grid[pos.0][pos.1];

    !(pos.0 == 0 || pos.0 == grid.len() - 1 || pos.1 == 0 || pos.1 == grid[0].len())
        && grid[pos.0][pos.1 + 1..].iter().any(|h| *h >= height)
        && grid[pos.0][..pos.1].iter().any(|h| *h >= height)
        && grid[..pos.0].iter().any(|row| row[pos.1] >= height)
        && grid[pos.0 + 1..].iter().any(|row| row[pos.1] >= height)
}

fn view(grid: &Vec<Vec<u32>>, pos: (usize, usize)) -> u32 {
    let height = grid[pos.0][pos.1];
    let mut ret = (0, 0, 0, 0);

    view!(grid[pos.0][pos.1 + 1..].iter(), height, ret.0);
    view!(grid[pos.0][..pos.1].iter().rev(), height, ret.1);
    view!(grid[..pos.0].iter().rev(), height, ret.2, [pos.1]);
    view!(grid[pos.0 + 1..].iter(), height, ret.3, [pos.1]);

    ret.0 * ret.1 * ret.2 * ret.3
}
