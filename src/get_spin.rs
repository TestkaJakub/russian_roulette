use crate::inputs::input_value;
use crate::SpinMode;

pub fn get_spin(spin_mode: Option<SpinMode>) -> SpinMode {
    match spin_mode {
        Some(v) => v, 
        None => {
            get_spin_cli()
        },
    }
}

fn get_spin_cli() -> SpinMode {
    loop {     
        let option : u8 = input_value(Some("The cylinder can either remain unspun, be spun once after being loaded, or be spun before each player's turn.\n1. Do not spin the cylinder.\n2. Spin the cylinder once after loading.\n3. Spin the cylinder at the beginning of each player's turn. \n(expecting 1, 2 or 3)"));
        match option {
            1 => break SpinMode::NoSpin,
            2 => break SpinMode::Spin, 
            3 => break SpinMode::SpinBeforeShot, 
            _ => println!("{} is not a valid option. Try again!", option),
        }
    }
}