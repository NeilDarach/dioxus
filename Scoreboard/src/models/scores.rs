use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

fn get_store_path() -> PathBuf {
    if let Some(base_dirs) = BaseDirs::new() {
        let mut path = PathBuf::from(base_dirs.data_dir());
        path.extend(&["Scoreboard"]);
        std::fs::create_dir_all(&path).expect("Unable to create data directory");
        return path;
    } else {
        let mut fallback = PathBuf::from(".");
        fallback.extend(&["Scoreboard"]);
        std::fs::create_dir_all(&fallback).ok();
        fallback
    }
}
fn get_crib_path() -> PathBuf {
    let mut path = get_store_path();
    path.extend(&["crib.toml"]);
    path
}

impl CribScores {
    pub fn new() -> Result<Self, ()> {
        let store_path = get_crib_path();
        if store_path.exists() {
            let data = fs::read(&store_path).expect("Failed to read score file");
            //let toml = String::from_utf8(data).expect("Score file is not a string");
            Ok(toml::from_slice(&data).expect("Failed to deserialize score file"))
        } else {
            let default = CribScores::default();
            let toml = toml::to_string(&default).expect("Failed to serialize new score file");
            fs::write(&store_path, &toml).expect("Failed to write new score file");
            Ok(default)
        }
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
        self.save().expect("Failed to save score");
    }

    pub fn save(&self) -> Result<(), ()> {
        let toml = toml::to_string(self).expect("Failed to serialize settings");
        fs::write(get_crib_path(), toml).expect("fs write failed");
        Ok(())
    }
}
