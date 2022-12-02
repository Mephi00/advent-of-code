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
