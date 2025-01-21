use crate::{impl_state_check, uvec::UVec, CylinderData, SpinMode};
use rand::prelude::*;

pub enum Trigger {
    Fire,
    DryFire,
}

impl_state_check!(Trigger, Fire, did_fired);

pub enum IsLoaded {
    Loaded,
    Unloaded,
}

impl_state_check!(IsLoaded, Loaded, is_loaded);

pub struct Revolver {
    pub cylinder_sequence: u128,
    cylinder_capacity: u8,
    spin_mode: SpinMode
}

impl Revolver {
    pub fn new(x : CylinderData, y : SpinMode) -> Self {
        let mut revolver = Self {
            cylinder_sequence: 0,
            cylinder_capacity: 0,
            spin_mode: y,
        };
        match x {
            CylinderData::Sequence { sequence: data } => revolver.initialize(data),
            CylinderData::Capacity { capacity: data } => revolver.random_cylinder(Some(data)),
        }
        if revolver.spin_mode == SpinMode::Spin {
            revolver.spin(None)
        }
        
        revolver
    }

    pub fn load(&mut self, x : &str) {
        self.initialize(x.to_string());
    }
    
    pub fn spin(&mut self, x: Option<u8>) {

        let mut rng = rand::thread_rng();
    
        // Default spin strength is a random number within the cylinder capacity
        let spin_strength = x.unwrap_or_else(|| rng.gen_range(0..self.cylinder_capacity)) as u32;
    
        // Mask to ensure we only work within the `cylinder_capacity` bits
        let mask = (1u128 << self.cylinder_capacity) - 1; // e.g., for 4 bits, mask = 0b1111
    
        // Apply the mask to ensure the cylinder_sequence fits within the cylinder capacity
        self.cylinder_sequence &= mask;
    
        // Perform the rotation as a `cylinder_capacity`-bit sequence
        let rotated = (self.cylinder_sequence.rotate_right(spin_strength) & mask)
            | ((self.cylinder_sequence << ((self.cylinder_capacity as u32) - spin_strength)) & mask);
    
        self.cylinder_sequence = rotated;
    }

    pub fn attempt_shooting(&mut self) -> Trigger {
        if self.spin_mode == SpinMode::SpinBeforeShot {
            self.spin(None);
        }

        let result = match self.cylinder_sequence % 2 == 1 {
            true => {
                self.cylinder_sequence = self.cylinder_sequence - 1;
                Trigger::Fire
            },
            false => Trigger::DryFire
        };
        self.spin(Some(1));
        result
    }

    fn initialize(&mut self, x: String) {
        let x = x.replace("_", "");
        self.cylinder_sequence = u128::from_str_radix(&x, 2)
            .expect("Invalid cylinder sequence. Expected binary number");
        self.cylinder_capacity = x.len() as u8;
    }

    pub fn shuffle(&mut self, ones: Option<u8>) {
        let mut possible_bullet_positions: UVec<u8> = (0..self.cylinder_capacity).collect();

        possible_bullet_positions.shuffle(thread_rng());

        let ones = ones.unwrap_or_else(|| self.cylinder_sequence.count_ones() as u8);

        let selected_positions = possible_bullet_positions.iter()
                                                                                           .take(ones as usize);
        
        let mut result = 0;
        for &pos in selected_positions {
            result |= 1 << pos;
        }

        self.cylinder_sequence = result;
    }

    pub fn random_cylinder(&mut self, cyl_len : Option<u8>) {
        let cl = cyl_len.unwrap_or_else(|| self.cylinder_capacity);
        self.cylinder_capacity = cl;
        let mut rng = thread_rng();

        let cyl_len = rng.gen_range(1..=cl);

        self.shuffle(Some(cyl_len));
    }

    #[allow(dead_code)]
    pub fn is_loaded(&self) -> IsLoaded {
        match self.cylinder_sequence > 0 {
            true => IsLoaded::Loaded,
            false => IsLoaded::Unloaded
        }
    }  
}