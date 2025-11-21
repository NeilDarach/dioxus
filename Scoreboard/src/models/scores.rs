use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CribScores {
    #[serde(default)]
    pub player_1_name: String,
    #[serde(default)]
    pub player_1_score: u16,
    #[serde(default)]
    pub player_1_previous: u16,
    #[serde(skip)]
    pub player_1_lastclick: Option<Instant>,
    #[serde(default)]
    pub player_2_name: String,
    #[serde(default)]
    pub player_2_score: u16,
    #[serde(default)]
    pub player_2_previous: u16,
    #[serde(skip)]
    pub player_2_lastclick: Option<Instant>,
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
            player_1_name: "Player 1".to_owned(),
            player_1_score: 0,
            player_1_previous: 0,
            player_1_lastclick: None,
            player_2_name: "Player 2".to_owned(),
            player_2_score: 0,
            player_2_previous: 0,
            player_2_lastclick: None,
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
            Ok(toml::from_slice(&data).expect("Failed to deserialize score file"))
        } else {
            let default = CribScores::default();
            default.save().expect("Failed to save new default toml");
            Ok(default)
        }
    }
    pub fn update(&mut self, player: Player, action: Action) {
        let (name, score, previous, lastclick) = match player {
            Player::PlayerOne => (
                &mut self.player_1_name,
                &mut self.player_1_score,
                &mut self.player_1_previous,
                &mut self.player_1_lastclick,
            ),
            Player::PlayerTwo => (
                &mut self.player_2_name,
                &mut self.player_2_score,
                &mut self.player_2_previous,
                &mut self.player_1_lastclick,
            ),
        };
        match action {
            Action::ResetScore => {
                *score = 0;
                *previous = 0;
            }
            Action::ChangeScore(delta) => {
                if delta > 0 {
                    if let Some(instant) = lastclick {
                        if instant.elapsed().as_secs() > 2 {
                            *previous = *score;
                        }
                    }
                    *lastclick = Some(Instant::now());
                    *score = 121.min(*score + delta as u16)
                }
                if delta < 0 {
                    *score = *score - (delta.unsigned_abs().min(*score))
                }
            }
            Action::ChangeName(ref newname) => {
                *name = newname.clone();
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
