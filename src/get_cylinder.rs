use crate::inputs::input_value;
use crate::CylinderData;

pub fn get_cylinder(cylinder_data: Option<CylinderData>) -> CylinderData {
    match cylinder_data {
        Some(v) => v, 
        None => {
            get_cylinder_cli()
        },
    }
}

fn get_cylinder_cli() -> CylinderData {
    loop {     
        let option : u8 = input_value(Some("Revolver can either be loaded with specific cylinder sequence or generate one based on cylinder capacity.\n1. Provide cylinder sequence e.x. '001001'\n2. Provide cylinder capacity e.x. 7\n(expecting 1 or 2)"));
        match option {
            1 => break CylinderData::Sequence { sequence: (input_value(Some("Please provide specific cylinder sequence"))) },
            2 => break CylinderData::Capacity { capacity: (input_value(Some("Please provide cylinder capacity")))}, 
            _ => println!("{} is not a valid option. Try again!", option),
        }
    }
}