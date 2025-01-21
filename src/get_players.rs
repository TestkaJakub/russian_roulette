use crate::uvec;
use uvec::UVec;
use std::fs::read_to_string;
use crate::inputs;
use inputs::*;


pub fn get_players(player_conf_file: Option<String>) -> UVec<String> {
    match player_conf_file {
        Some(v) => {
            let file = match read_to_string(v) {
            // let file = match File::open(v) {
                Ok(v) => Some(v),
                Err(e) => {
                    println!("{e}");
                    None
                }
            };

            match file {
                Some (v) => {
                    let players_from_file = extract_players_from_file(Some(v));
                    if players_from_file.len() < 2 {
                        println!("Minimum two players are required to start a game");
                        get_players_cli()
                    }
                    else {
                        players_from_file
                    }
                },
                None => get_players_cli(),
            }
        }, // Read players from file provided in arg
        None => {
            println!("Player configuration file was not specified");
            get_players_cli()
        },
    }
}

fn extract_players_from_file(file_content: Option<String>) -> UVec<String> {
    let content = match file_content {
        Some(content) => content,
        None => {
            loop {
                let file_path = input_value::<String>(Some("Specify player configuration file"));
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => break content,
                    Err(e) => {
                        println!("{e}. Try again!");
                        println!("{}", file_path)
                    },
                }
            }
        },
    };

    let players = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();

    players
}

fn get_players_cli() -> UVec<String> {
    loop {     
        let option : u8 = input_value(Some("You can either specify player configuration file now or provide each player by hand\n1. Specify player configuration file\n2. Specify players by hand\n(expecting 1 or 2)"));
        match option {
            1 => break extract_players_from_file(None), // Chosen to specify player configuration file
            2 => {
                println!("Chosen to provide each player by hand.");
                break input_players()
            },
            _ => {
                println!("{} is not a valid option. Try again!", option);
            }
        }
    }
}

fn input_players() -> UVec<String> {
    let mut players = UVec::new();
    
    let mut  i = 1;
    'manual_player_input : while i < u8::MAX {
        println!("Please provide the name for player number {}, or exit player initialization by writing 'exit'", i);
        let name : String = input_value(None);
        match name.to_lowercase().as_str() {
            "exit" => {
                if i < 3 {
                    println!("Minimum two players are required to start a game");
                    continue;
                }
                else {
                    break 'manual_player_input
                }
            },
            _ => {
                players.add(name);
                i += 1;
            },
        }
    }

    players
}