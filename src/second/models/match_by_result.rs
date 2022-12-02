use super::{end_result::EndResult, r#move::Move};

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
