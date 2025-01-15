// Type your code here, or load an example.
use std::{cell::RefCell, rc::Rc, ops::Shr};

struct Player {
    name: String,
    next_player: Option<Rc<RefCell<Player>>>,
}

impl Player {
    fn shoot(&self, cylinder : u8) -> bool {
        if cylinder % 2 == 1 {
            return true;
        }
        false
    }

    fn play(&self, cylinder : u8) {
        if cylinder <= 0 {
            println!("No bullets left, game ends!");
            return;
        }

        if self.shoot(cylinder) {
            println!("Player {} died!", self.name);
        }

        match &self.next_player {
            Some(v) => v.borrow().play(cylinder.shr(1)),
            None => println!("Game ends!")
        }
        
    }
}

fn main() {
    let player1 = Rc::new(RefCell::new(Player {
        name: "p1".to_string(),
        next_player: None,
    }));

    let player2 = Rc::new(RefCell::new(Player {
        name: "p2".to_string(),
        next_player: Some(player1.clone()),
    }));

    let player3 = Rc::new(RefCell::new(Player {
        name: "p3".to_string(),
        next_player: Some(player2.clone()),
    }));

    let player4 = Rc::new(RefCell::new(Player {
        name: "p4".to_string(),
        next_player: Some(player3.clone()),
    }));

    let player5 = Rc::new(RefCell::new(Player {
        name: "p5".to_string(),
        next_player: Some(player4.clone()),
    }));

    player1.borrow_mut().next_player = Some(player5.clone());

    player1.borrow().play(0b10000);
}