use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CribScores {
    pub player_1_name: String,
    pub player_1_score: u16,
    pub player_2_name: String,
    pub player_2_score: u16,
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
}
