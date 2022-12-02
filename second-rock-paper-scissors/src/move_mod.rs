use std::cmp::Ordering;

#[derive(Debug)]
pub struct Match(pub Move, pub Move);

impl Match {
    pub fn get_result(&self) -> u16 {
        let mut points: u8 = 0;

        if self.0 < self.1 {
            points += 6;
        } else if self.0 == self.1 {
            points += 3
        }

        points += self.1.get_score();

        points.into()
    }
}

#[derive(Debug)]
pub struct MatchByResult(pub Move, pub EndResult);

impl MatchByResult {
    pub fn get_result(&self) -> u16 {
        let mut points: u8 = 0;

        match &self.1 {
            EndResult::Lose => {}
            EndResult::Draw => points += 3,
            EndResult::Win => points += 6,
        }

        points += self.0.get_target_move(&self.1).get_score();

        points.into()
    }
}

#[derive(Debug)]
pub enum EndResult {
    Lose,
    Draw,
    Win,
}

impl EndResult {
    pub fn new(char: char) -> Result<EndResult, ()> {
        match char {
            'X' => Ok(EndResult::Lose),
            'Y' => Ok(EndResult::Draw),
            'Z' => Ok(EndResult::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn new(char: char) -> Result<Move, ()> {
        match char {
            'A' => Ok(Move::Rock {}),
            'B' => Ok(Move::Paper {}),
            'C' => Ok(Move::Scissors {}),

            'X' => Ok(Move::Rock {}),
            'Y' => Ok(Move::Paper {}),
            'Z' => Ok(Move::Scissors {}),
            _ => Err(()),
        }
    }

    pub fn get_target_move(&self, result: &EndResult) -> Self {
        match result {
            EndResult::Draw => match self {
                Move::Paper => Move::Paper,
                Move::Rock => Move::Rock,
                Move::Scissors => Move::Scissors,
            },
            EndResult::Lose => match self {
                Move::Paper => Move::Rock,
                Move::Rock => Move::Scissors,
                Move::Scissors => Move::Paper,
            },
            EndResult::Win => match self {
                Move::Paper => Move::Scissors,
                Move::Rock => Move::Paper,
                Move::Scissors => Move::Rock,
            },
        }
    }

    fn get_score(&self) -> u8 {
        match &self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.get_score() == other.get_score()
    }

    fn ne(&self, other: &Self) -> bool {
        self.get_score() != other.get_score()
    }
}

impl PartialOrd for Move {
    fn ge(&self, other: &Self) -> bool {
        match self {
            Move::Paper => match other {
                Move::Paper => true,
                Move::Rock => true,
                Move::Scissors => false,
            },
            Move::Rock => match other {
                Move::Paper => false,
                Move::Rock => true,
                Move::Scissors => true,
            },
            Move::Scissors => match other {
                Move::Paper => true,
                Move::Rock => false,
                Move::Scissors => true,
            },
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match self {
            Move::Paper => match other {
                Move::Paper => false,
                Move::Rock => true,
                Move::Scissors => false,
            },
            Move::Rock => match other {
                Move::Paper => false,
                Move::Rock => false,
                Move::Scissors => true,
            },
            Move::Scissors => match other {
                Move::Paper => true,
                Move::Rock => false,
                Move::Scissors => false,
            },
        }
    }

    fn le(&self, other: &Self) -> bool {
        !(self > other)
    }

    fn lt(&self, other: &Self) -> bool {
        !(self >= other)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Move::Paper => match other {
                Move::Paper => Some(Ordering::Equal),
                Move::Rock => Some(Ordering::Greater),
                Move::Scissors => Some(Ordering::Less),
            },
            Move::Rock => match other {
                Move::Paper => Some(Ordering::Less),
                Move::Rock => Some(Ordering::Equal),
                Move::Scissors => Some(Ordering::Greater),
            },
            Move::Scissors => match other {
                Move::Paper => Some(Ordering::Greater),
                Move::Rock => Some(Ordering::Less),
                Move::Scissors => Some(Ordering::Equal),
            },
        }
    }
}
