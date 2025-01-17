use crate::impl_state_check;
use crate::Revolver;

pub enum IsAlive {
    Alive,
    Dead,
}

impl_state_check!(IsAlive, Alive, is_alive);

pub struct Player {
    name: String,
    pub is_alive: IsAlive,
}

impl Player {
    pub fn new(name: &str) -> Self {
        return Player {
            name: name.to_string(),
            is_alive: IsAlive::Alive,
        }
    }

    fn pull_the_trigger(&mut self, mut revolver : Revolver) -> Revolver {
        if revolver.attempt_shooting().did_fired() {
            println!("Player {} shot themselves", self.name);
            self.is_alive = IsAlive::Dead;
        }
        else {
            println!("Player {} got lucky this time", self.name);
        }
        revolver
    }

    pub fn play_turn(&mut self, mut revolver : Revolver) -> Revolver {
        if self.is_alive.is_alive() {
            revolver = self.pull_the_trigger(revolver);
        }

        revolver
    }
}