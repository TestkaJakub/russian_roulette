
pub mod bool_enums;

pub mod revolver;
use revolver::*;

mod player;
use player::*;

fn main() {
    let mut revolver = Revolver::new("00111000");
    let mut p1 = Player::new("p1");
    
    while p1.is_alive.is_alive() {
        revolver = p1.play_turn(revolver);
    }
}