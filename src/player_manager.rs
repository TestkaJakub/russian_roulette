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
        let mut alive_players = UVec::new();
        for player in self.player_set.iter() {
            if player.is_alive.is_alive() {
                alive_players.add(player); // Adds a reference to the player
            }
        }
        alive_players
    }
}