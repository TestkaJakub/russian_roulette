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
    player_manager.add_player("Filip");
    player_manager.add_player("PSEU");
    player_manager.add_player("Gwiezdny");
    player_manager.add_player("Igor");
    player_manager.add_player("Lord");
    player_manager.add_player("Janek");
    player_manager.add_player("Q2CK");
    player_manager.add_player("KubolV");
    player_manager.add_player("Krzysiu");
    
    let mut revolver = Revolver::new(RevolverData::CylinderCapacity(player_manager.get_alive_players().len() as u8));
    
    println!("{:b}", revolver.cylinder_sequence);

    
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
            player_manager.player_set[i].play_turn(&mut revolver);
        }
    }
}