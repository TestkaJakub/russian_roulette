// Type your code here, or load an example.
use std::{cell::RefCell, ops::Shr, rc::{Rc, Weak}};

#[derive(Debug)]
struct Player {
    name: String,
    next_player: Option<Weak<RefCell<Player>>>,
}

impl Player {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Player {
            name: name.to_string(),
            next_player: None,
        }))
    }

    fn shoot(&self, cylinder : u128) -> bool {
        if cylinder % 2 == 1 {
            return true;
        }
        false
    }

    fn play_one_turn(&self, cylinder : u128) -> Option<u128> {

        if self.shoot(cylinder) {
            println!("Player {} died!", self.name);
        }
        
        self.next_player.as_ref()
                        .and_then(|x| x.upgrade())
                        .and_then(|x| x.try_borrow_mut()
                                                            .ok()
                                                            .and_then(|x| x.play_one_turn(cylinder.shr(1))))
                        .or_else(|| Some(cylinder.shr(1))) 
                        
    }

    fn play_roulette(&self, cylinder: Option<u128>) -> Option<u128> {
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
                    Some(v) if v > 0 => self.play_roulette(Some(v)),
                    _ => { 
                        println!("No bullets left, game ends!");
                        None
                    }
                }
    }


}

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

    player1.borrow_mut().play_roulette(Some(0b100000000000));
}