use std::{iter::Peekable, str::Lines};

use crate::fs;

#[derive(Debug)]
enum Fs {
    File(u64),
    Dir(String, Vec<Fs>, u64),
}

impl Fs {
    fn sum(&mut self) -> u64 {
        match self {
            Fs::Dir(_, subs, sum) => {
                *sum = subs.iter_mut().fold(*sum, |acc, cur| acc + cur.sum());
                *sum
            }
            Fs::File(size) => *size,
        }
    }

    fn sum_under(&self, limit: u64) -> u64 {
        match self {
            Fs::Dir(_, subs, sum) => subs
                .iter()
                .fold(if *sum <= limit { *sum } else { 0 }, |sum, s| {
                    sum + s.sum_under(limit)
                }),
            _ => 0,
        }
    }

    fn bigger_than(&self, size: u64) -> u64 {
        fn iter(fss: &Fs, size: u64, last: u64) -> u64 {
            match fss {
                Fs::Dir(_, subs, dsize) if *dsize >= size && *dsize < last => subs
                    .iter()
                    .fold(*dsize, |last, cur| (iter(cur, size, last)).min(last)),
                _ => last,
            }
        }

        iter(self, size, u64::MAX)
    }
}

pub fn part1() -> u64 {
    let lines = fs::get_input(7).unwrap();
    let mut lines = lines.lines().peekable();

    let mut root = Fs::Dir("/".to_string(), cd(&mut lines, true), 0);
    root.sum();
    root.sum_under(100_000)
}

pub fn part2() -> u64 {
    let lines = fs::get_input(7).unwrap();
    let mut lines = lines.lines().peekable();

    let mut root = Fs::Dir("/".to_string(), cd(&mut lines, true), 0);
    let sum = root.sum();
    root.bigger_than(30000000 - (70000000 - sum))
}

fn ls_dirs(lines: &mut Peekable<Lines>) -> Vec<Fs> {
    let mut fss = vec![];
    lines.next();

    while let Some(line) = lines.peek() {
        if &line[..2] == "$ " {
            break;
        }

        let (size, name) = line.split_once(" ").unwrap();

        if size == "dir" {
            fss.push(Fs::Dir(name.to_string(), vec![], 0))
        } else {
            fss.push(Fs::File(size.parse().unwrap()))
        }

        lines.next();
    }

    fss
}

fn cd(lines: &mut Peekable<Lines>, root: bool) -> Vec<Fs> {
    let mut fss = vec![];

    while let Some(line) = lines.peek() {
        if &line[..2] == "$ " {
            match &line[2..4] {
                "cd" => match &line[5..] {
                    "/" if root => {
                        lines.next();
                    }
                    "/" => break,
                    ".." => {
                        lines.next();
                        break;
                    }
                    dir => {
                        for fs in &mut fss {
                            match fs {
                                Fs::Dir(name, subs, _) if dir == name => {
                                    lines.next();
                                    *subs = cd(lines, false);
                                }
                                _ => continue,
                            }
                        }
                    }
                },
                "ls" => fss.append(&mut ls_dirs(lines)),
                _ => panic!("wtf"),
            }
        }
    }

    fss
}
