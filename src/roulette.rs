use crate::{player_manager::*, revolver::*, uvec::UVec, CylinderData, SpinMode};

pub fn play(players : UVec<String>, cylinder_data : CylinderData, spin_mode : SpinMode) {
    let mut player_manager = PlayerManager::new();
    
    for player in players.iter() {
        player_manager.add_player(player);
    }
    
    // let mut revolver = Revolver::new(CylinderData::Capacity{ capacity: player_manager.get_alive_players().len() as u8 });
    let mut revolver = Revolver::new(cylinder_data, spin_mode);

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