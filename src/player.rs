use std::{cell::RefCell, hash::Hash, ops::Shr, rc::{Rc, Weak}};

use crate::player_manager::*;

pub type RcRefPlayer = Rc<RefPlayer>;

#[derive(Debug)]
pub struct RefPlayer(RefCell<Player>);

impl Hash for  RefPlayer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl PartialEq for  RefPlayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RefPlayer {

}

impl std::ops::Deref for RefPlayer {
    type Target = RefCell<Player>;


    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Player {
    name: String,
    pub next_player: Option<Weak<RefPlayer>>,
    pub alive: bool,
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Eq for Player { }

impl Player {
    pub fn new(name: &str) -> RcRefPlayer {
        Rc::new(RefPlayer(RefCell::new(Player {
            name: name.to_string(),
            next_player: None,
            alive: true,
        })))
    }

    fn shoot(&mut self, cylinder : u128) -> bool {
        if cylinder % 2 == 1 {
            self.alive = false;
            return true;
        }
        false
    }

    fn play_one_turn(&mut self, mut cylinder : u128) -> Option<u128> {

        if self.shoot(cylinder) && self.alive {
            println!("Player {} died!", self.name);
            cylinder = cylinder.shr(1);
        }
        
        self.next_player.as_ref()
                        .and_then(|x| x.upgrade())
                        .and_then(|x| x.try_borrow_mut()
                                                            .ok()
                                                            .and_then(|mut x| x.play_one_turn(cylinder)))
                        .or_else(|| Some(cylinder)) 
                        
    }

    pub fn play_roulette(&mut self, cylinder: Option<u128>, player_manager : &PlayerManager) -> Option<u128> {
        let alive_players = player_manager.get_alive();
        if alive_players.is_empty(){
            println!("No alive players left!");
            return None;
        }
        
        match cylinder.and_then(|v: u128| 
            {
                if v > 0
                {
                    self.play_one_turn(v)
                }
                else 
                {
                    None
                }
            } 
        )
                .or_else(|| None) 
                {
                    Some(v) if v > 0 => self.play_roulette(Some(v), player_manager),
                    _ => { 
                        println!("No bullets left, game ends!");
                        None
                    }
                }
    }
}