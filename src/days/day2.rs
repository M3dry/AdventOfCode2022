use crate::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn from_str(str: &str) -> Self {
        match str {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("at the disco"),
        }
    }

    fn from_tuple(tuple: (&str, &str)) -> (Self, Self) {
        (Self::from_str(tuple.0), Self::from_str(tuple.1))
    }

    fn as_num(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn loss(&self, opponent: &Self) -> bool {
        match (self, opponent) {
            (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock)
            | (Self::Rock, Self::Paper) => true,
            _ => false,
        }
    }

    fn to_lose(&self) -> Play {
        match self {
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
            Self::Rock => Self::Paper,
        }
    }

    fn to_win(&self) -> Play {
        match self {
            Self::Scissors => Self::Paper,
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
        }
    }
}

#[derive(Debug)]
enum State {
    Loss,
    Draw,
    Win,
}

impl State {
    fn from_str(str: &str) -> Self {
        match str {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("at the disco"),
        }
    }

    fn from_plays(you: &Play, opponent: &Play) -> Self {
        if opponent == you {
            Self::Draw
        } else if opponent.loss(you) {
            Self::Win
        } else {
            Self::Loss
        }
    }

    fn to_play(&self, opponent: &Play) -> Play {
        match self {
            Self::Loss => opponent.to_win(),
            Self::Draw => opponent.clone(),
            Self::Win => opponent.to_lose(),
        }
    }

    fn as_num(&self) -> u64 {
        match self {
            State::Win => 6,
            State::Loss => 0,
            State::Draw => 3,
        }
    }
}

pub fn part1() -> u64 {
    fs::get_input(2).unwrap().lines().fold(0, |acc, cur| {
        let (opponent, you) = Play::from_tuple(cur.split_once(' ').unwrap());

        acc + you.as_num() + State::from_plays(&you, &opponent).as_num()
    })
}

pub fn part2() -> u64 {
    fs::get_input(2).unwrap().lines().fold(0, |acc, cur| {
        let play = cur.split_once(' ').unwrap();
        let state = State::from_str(play.1);

        acc + state.to_play(&Play::from_str(play.0)).as_num() + state.as_num()
    })
}
