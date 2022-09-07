#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    score: Vec<u16>
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { score: vec![] }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins < 11 {
            self.score.push(pins);
            return Ok(())
        }
        Err(Error::NotEnoughPinsLeft)
    }

    pub fn score(&self) -> Option<u16> {
        
    }
}
