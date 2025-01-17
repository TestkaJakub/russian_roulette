// Type your code here, or load an example.
use std::rc::Rc;

mod player;
use player::*;

mod player_manager;
use player_manager::*;


fn main() {
    let player1 = Player::new("p1");
    let player2 = Player::new("p2");
    let player3 = Player::new("p3");
    let player4 = Player::new("p4");
    let player5 = Player::new("p5");
    let player6 = Player::new("p6");

    player1.borrow_mut().next_player = Some(Rc::downgrade(&player2));
    player2.borrow_mut().next_player = Some(Rc::downgrade(&player3));
    player3.borrow_mut().next_player = Some(Rc::downgrade(&player4));
    player4.borrow_mut().next_player = Some(Rc::downgrade(&player5));
    player5.borrow_mut().next_player = Some(Rc::downgrade(&player6));
    player6.borrow_mut().next_player = Some(Rc::downgrade(&player1));

    let player_hash : hashbrown::HashSet<&Rc<RefPlayer>> = HashSet::from([&player1, &player2, &player3, &player4, &player5, &player6]);
    
    let player_manager = PlayerManager::new(Some(player_hash));

    for player in player_manager.get_alive().iter() {
        println!("{:?}", &player);
    }

    player1.borrow_mut().play_roulette(Some(0b1111111111111111), &player_manager);
}