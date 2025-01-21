use std::{io, str::FromStr};

pub fn input_value<T>(prompt : Option<&str>) -> T where T : std::str::FromStr, <T as FromStr>::Err: std::fmt::Display {
    loop {
        match prompt {
            Some(v) => println!("{v}"),
            None => (),
        }
    
        let mut x = String::new();

        io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

        match x.trim().parse() {
            Ok(v) => break v,
            Err(e) => {
                println!("{e}. Try again!");
                continue;
            }
        };
    }
}