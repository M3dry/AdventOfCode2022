use std::cmp::Ordering;
use std::str::FromStr;

use crate::fs;

#[derive(Debug)]
enum Packet {
    Num(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(lhs), Packet::Num(rhs)) => lhs.cmp(rhs),
            (Packet::List(lhs), Packet::List(rhs)) => lhs.cmp(rhs),
            (Packet::Num(_), Packet::List(rhs)) => std::slice::from_ref(self).cmp(rhs),
            (Packet::List(lhs), Packet::Num(_)) => lhs.as_slice().cmp(std::slice::from_ref(other)),
        }
    }
}

impl FromStr for Packet {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err('s');
        }

        let mut stack = vec![];
        let mut packet = vec![];
        let mut chars = s[1..s.len() - 1].chars().peekable();

        while let Some(c) = chars.next() {
            let pac = match c {
                '[' => {
                    stack.push(packet);
                    packet = vec![];
                    continue;
                }
                ']' => {
                    let pac = Packet::List(packet);
                    if !stack.is_empty() {
                        packet = stack.pop().unwrap();
                    } else {
                        packet = vec![];
                    }
                    pac
                }
                ',' => continue,
                c if c.is_digit(10) => {
                    let mut digits = c.to_digit(10).unwrap();

                    while let Some(c) = chars.peek() {
                        if *c == ']' || *c == ',' {
                            break;
                        } else {
                            digits = digits * 10 + chars.next().unwrap().to_digit(10).unwrap()
                        }
                    }

                    Packet::Num(digits)
                }
                _ => return Err(c),
            };

            packet.push(pac);
        }

        Ok(Packet::List(packet))
    }
}

pub fn part1() -> usize {
    let packets = fs::get_input(13)
        .unwrap()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Packet>>();
    let packets_len = packets.len();
    let mut sum = 0;

    for i in (0..packets_len).step_by(2) {
        if packets[i].partial_cmp(&packets[i + 1]) == Some(Ordering::Less) {
            sum += 1 + i / 2
        }
    }

    sum
}

pub fn part2() -> usize {
    let mut packets = fs::get_input(13)
        .unwrap()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Packet>>();
    packets.append(&mut vec![
        Packet::List(vec![Packet::List(vec![Packet::Num(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Num(6)])]),
    ]);
    packets.sort();
    let mut sum = 1;

    for (i, packet) in packets.iter().enumerate() {
        if *packet == Packet::List(vec![Packet::List(vec![Packet::Num(2)])])
            || *packet == Packet::List(vec![Packet::List(vec![Packet::Num(6)])])
        {
            sum *= i + 1;
        }
    }

    sum
}
