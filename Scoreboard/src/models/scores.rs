use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CribScores {
    pub player_1: CribScore,
    pub player_2: CribScore,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CribScore {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub score: u16,
    #[serde(default)]
    pub previous_score: u16,
    #[serde(skip)]
    pub lastclick: Option<Instant>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Player {
    PlayerOne,
    PlayerTwo,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Action {
    ResetScore,
    ChangeScore(i16),
    ChangeName(String),
}

impl Default for CribScores {
    fn default() -> Self {
        CribScores {
            player_1: CribScore {
                name: "Player 1".to_owned(),
                score: 0,
                previous_score: 0,
                lastclick: None,
            },
            player_2: CribScore {
                name: "Player 2".to_owned(),
                score: 0,
                previous_score: 0,
                lastclick: None,
            },
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
            if let Ok(scores) = toml::from_slice(&data) {
                return Ok(scores);
            }
        }

        let default = CribScores::default();
        default.save().expect("Failed to save new default toml");
        Ok(default)
    }
    pub fn update(&mut self, player: Player, action: Action) {
        let score = match player {
            Player::PlayerOne => &mut self.player_1,
            Player::PlayerTwo => &mut self.player_2,
        };
        match action {
            Action::ResetScore => {
                score.score = 0;
                score.previous_score = 0;
            }
            Action::ChangeScore(delta) => {
                if delta > 0 {
                    if let Some(instant) = score.lastclick {
                        if instant.elapsed().as_secs() > 2 {
                            score.previous_score = score.score;
                        }
                    }
                    score.lastclick = Some(Instant::now());
                    score.score = 121.min(score.score + delta as u16)
                }
                if delta < 0 {
                    score.score -= delta.unsigned_abs().min(score.score);
                }
            }
            Action::ChangeName(ref newname) => {
                score.name = newname.clone();
            }
        };
        self.save().expect("Failed to save score");
    }

    pub fn save(&self) -> Result<(), ()> {
        let toml = toml::to_string(self).expect("Failed to serialize settings");
        fs::write(get_crib_path(), toml).expect("fs write failed");
        Ok(())
    }
}
