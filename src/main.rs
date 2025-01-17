pub use hashbrown::HashSet;

pub mod bool_enums;
pub mod uvec;

pub mod revolver;
use revolver::*;

mod player;
use player::*;

mod player_manager;
use player_manager::*;

fn main() {
    let mut player_manager = PlayerManager::new();
    let mut revolver = Revolver::new("111111");
    revolver.spin(None);
    
    player_manager.add_player("p1");
    player_manager.add_player("p2");
    player_manager.add_player("p3");
    player_manager.add_player("p4");
    player_manager.add_player("p5");
    player_manager.add_player("p6");

    'game: loop
    {
        let player_count = player_manager.player_set.len();
        for i in 0..player_count {
            if player_manager.get_alive_players().len() <= 1 {
                println!("The winner is {}", player_manager.get_alive_players()[0].name);
                break 'game;
            }
            if revolver.is_loaded().is_loaded() == false {
                println!("No bullets left, remaining players are:");
                for still_alive in player_manager.get_alive_players() {
                    println!("{}", still_alive.name);
                }
                break 'game;
            }
            revolver = player_manager.player_set[i].play_turn(revolver);
        }
    }
}