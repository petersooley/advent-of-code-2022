use std::cmp;
use std::str;

#[derive(PartialEq, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl str::FromStr for Choice {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err(format!("{} is not a valid choice", s)),
        }
    }
}

impl cmp::PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self {
            Self::Rock => match other {
                Self::Rock => Some(cmp::Ordering::Equal),
                Self::Paper => Some(cmp::Ordering::Less),
                Self::Scissors => Some(cmp::Ordering::Greater),
            },
            Self::Paper => match other {
                Self::Rock => Some(cmp::Ordering::Greater),
                Self::Paper => Some(cmp::Ordering::Equal),
                Self::Scissors => Some(cmp::Ordering::Less),
            },
            Self::Scissors => match other {
                Self::Rock => Some(cmp::Ordering::Less),
                Self::Paper => Some(cmp::Ordering::Greater),
                Self::Scissors => Some(cmp::Ordering::Equal),
            },
        }
    }
}

#[derive(Debug)]
struct Battle {
    us: Choice,
    them: Choice,
}

impl Battle {
    fn score(&self) -> usize {
        let outcome = if self.us < self.them {
            0
        } else if self.us > self.them {
            6
        } else {
            3
        };
        outcome + self.us.score()
    }
}

impl str::FromStr for Battle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();
        let them: Choice = parts
            .next()
            .ok_or_else(|| format!("invalid battle line {}", s))?
            .parse()?;

        let desired = parts
            .next()
            .ok_or_else(|| format!("invalid battle line {}", s))?;
        let us = match desired {
            // we must lose
            "X" => match them {
                Choice::Rock => Choice::Scissors,
                Choice::Scissors => Choice::Paper,
                Choice::Paper => Choice::Rock,
            },
            // we must draw
            "Y" => match them {
                Choice::Rock => Choice::Rock,
                Choice::Scissors => Choice::Scissors,
                Choice::Paper => Choice::Paper,
            },
            // we must win
            "Z" => match them {
                Choice::Rock => Choice::Paper,
                Choice::Scissors => Choice::Rock,
                Choice::Paper => Choice::Scissors,
            },
            _ => return Err(format!("invalid battle line {}", s)),
        };

        Ok(Self { us, them })
    }
}

fn main() {
    let mut total = 0usize;

    for line in include_str!("./input.txt").lines() {
        let battle: Battle = line.parse().unwrap();

        // println!("line {}, battle {:?}, score {}", line, battle, battle.score());

        total += battle.score();
    }
    println!("total {}", total);

    // part 1 total was: 12794
}
