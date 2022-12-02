use super::r#move::Move;

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
