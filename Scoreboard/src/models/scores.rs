use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CribScores {
    pub player_1_name: String,
    pub player_1_score: u16,
    pub player_2_name: String,
    pub player_2_score: u16,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Eq, PartialEq, Clone)]
pub enum Action {
    ChangeScore(i16),
    ChangeName(String),
}

impl Default for CribScores {
    fn default() -> Self {
        CribScores {
            player_1_name: "Player 1".to_owned(),
            player_1_score: 0,
            player_2_name: "Player 2".to_owned(),
            player_2_score: 0,
        }
    }
}

impl CribScores {
    pub fn new() -> Result<Self, ()> {
        Ok(Self::default())
    }
    pub fn update(&mut self, player: Player, action: Action) {
        match (player, action) {
            (Player::PlayerOne, Action::ChangeScore(delta)) => {
                if delta > 0 {
                    self.player_1_score = 121.min(self.player_1_score + delta as u16)
                }
                if delta < 0 {
                    self.player_1_score =
                        self.player_1_score - (delta.unsigned_abs().min(self.player_1_score))
                }
            }
            (Player::PlayerOne, Action::ChangeName(ref name)) => {
                self.player_1_name = name.clone();
            }
            (Player::PlayerTwo, Action::ChangeScore(delta)) => {
                if delta > 0 {
                    self.player_2_score = 121.min(self.player_2_score + delta as u16)
                }
                if delta < 0 {
                    self.player_2_score =
                        self.player_2_score - (delta.unsigned_abs().min(self.player_2_score))
                }
            }
            (Player::PlayerTwo, Action::ChangeName(ref name)) => {
                self.player_2_name = name.clone();
            }
        }
    }
}
