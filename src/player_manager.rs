use crate::{uvec::UVec, Player};

pub struct PlayerManager {
    pub player_set: UVec<Player>
}

impl PlayerManager {
    pub fn new() -> Self {
        return PlayerManager {
            player_set: UVec::new(),
        }
    }

    pub fn add_player(&mut self, x : &str) {
        self.player_set.add(Player::new(x));
    }

    pub fn get_alive_players(&self) -> UVec<&Player> {
        self.player_set.iter().filter(|x| x.is_alive.is_alive()).collect()
    }
}