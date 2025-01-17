pub use hashbrown::HashSet;
use crate::player::*;


pub struct PlayerManager<'a> {
    pub player_set: HashSet<&'a RcRefPlayer>
}

impl<'a> PlayerManager<'a> {
    pub fn new(player_set : Option<HashSet<&'a RcRefPlayer>>) -> Self {
        let p_set = player_set.unwrap_or(HashSet::new());
        
        let player_manager = PlayerManager {
            player_set : p_set,
        };
        
        player_manager
    }
    
    #[allow(dead_code)]
    fn add_player(&mut self, player : &'a RcRefPlayer) {
        self.player_set.insert(player);
    }

    pub fn get_alive(&self) -> HashSet<&RcRefPlayer> {
        let mut alive_players = HashSet::new();
        
        for player in self.player_set.iter() {
            if player.borrow().alive {
                alive_players.insert(* player);
            }
        }

        alive_players
    }
}