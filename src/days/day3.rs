use crate::fs;

pub fn part1() -> u64 {
    fs::get_input(3)
        .unwrap()
        .lines()
        .map(|l| {
            let (first, second) = l.split_at(l.len() / 2);
            for f in first.chars() {
                for s in second.chars() {
                    if f == s {
                        if f.is_ascii_lowercase() {
                            return (f as u8 - 96) as u64;
                        } else {
                            return (f as u8 - 38) as u64;
                        }
                    }
                }
            }
            0
        })
        .sum()
}

pub fn part2() -> u64 {
    let lines = fs::get_input(3).unwrap();
    let mut lines = lines.lines();
    let mut last = vec![];
    let mut sum = 0;

    while let Some(line) = lines.next() {
        if last.len() != 2 {
            last.push(line);
        } else {
            last.push(line);
            let mut eqs = vec![];

            'f: for f in last[0].chars() {
                for s in last[1].chars() {
                    if f == s {
                        eqs.push(s);
                        break;
                    }
                }
                for t in last[2].chars() {
                    if f == t && eqs.contains(&f) {
                        if f.is_ascii_lowercase() {
                            sum += (f as u8 - 96) as u64
                        } else {
                            sum += (f as u8 - 38) as u64;
                        }
                        break 'f;
                    }
                }
            }
            last = vec![]
        }
    }

    sum
}
