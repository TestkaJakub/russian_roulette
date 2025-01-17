use crate::impl_state_check;
use crate::Revolver;

#[derive(PartialEq)]
pub enum IsAlive {
    Alive,
    Dead,
}

impl_state_check!(IsAlive, Alive, is_alive);

#[derive(PartialEq)]
pub struct Player {
    pub name: String,
    pub is_alive: IsAlive,
}

impl Player {
    pub fn new(name: &str) -> Self {
        return Player {
            name: name.to_string(),
            is_alive: IsAlive::Alive,
        }
    }
    
    pub fn play_turn(&mut self, revolver : &mut Revolver) {
        if self.is_alive.is_alive() {
            self.pull_the_trigger(revolver);
        }
    }

    fn pull_the_trigger(&mut self, revolver : &mut Revolver) {
        if revolver.attempt_shooting().did_fired() {
            println!("Player {} shot themselves", self.name);
            self.is_alive = IsAlive::Dead;
        }
        else {
            println!("Player {} got lucky this time", self.name);
        }
    }

}