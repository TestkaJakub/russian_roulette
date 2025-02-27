use std::io::{stdin, stdout, Read, Write};

mod roulette;
use roulette::*;

pub mod bool_enums;
pub mod uvec;

mod get_players;
use get_players::*;

mod get_cylinder;
use get_cylinder::*;

mod get_spin;
use get_spin::*;

mod inputs;

pub mod revolver;
use revolver::*;

mod player;
use player::*;

mod player_manager;

use clap::Parser;

#[derive(clap::Subcommand, Debug)]
pub enum CylinderData {
    /// Provide an explicit cylinder sequence e.g., "001001"
    Sequence {
        #[arg(short='c', long="cylinder_sequence")]
        sequence: String,
    },
    /// Provide the capacity of randomly generated cylinder
    Capacity {
        #[arg(short='C', long="cylinder_capacity")]
        capacity: u8,
    }
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
pub enum SpinMode {
    /// No spin
    NoSpin,
    /// Perform a single spin
    Spin,
    /// Spin before each shot
    SpinBeforeShot,
}

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Player configuration file location
    #[arg(short = 'p', long)]
    player_conf_file: Option<String>,
    
    /// Cylinder_sequence and cylinder_capacity are mutually exclusive, provide none or either one but not both
    #[command(subcommand)]
    cylinder_data: Option<CylinderData>,

    /// Select the spin mode
    #[arg(short = 's', long, value_enum)]
    spin_mode: Option<SpinMode>,
}

fn main() {
    let args = Args::parse();

    let players = get_players(args.player_conf_file);
    let cylinder_data = get_cylinder(args.cylinder_data);
    let spin_mode = get_spin(args.spin_mode);
    play(players, cylinder_data, spin_mode);

    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

