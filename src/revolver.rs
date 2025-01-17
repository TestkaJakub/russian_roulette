use crate::impl_state_check;

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
    cylinder_sequence: u128,
    cylinder_capacity: u8,
}

impl Revolver {
    pub fn new(x : &str) -> Self {
        let mut revolver = Self {
            cylinder_sequence: 0,
            cylinder_capacity: 0,
        };
        revolver.initialize(x);
        revolver
    }

    #[allow(dead_code)]
    pub fn load(&mut self, x : &str) {
        self.initialize(x);
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

    #[allow(dead_code)]
    pub fn attempt_shooting(&mut self) -> Trigger {
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

    fn initialize(&mut self, x: &str) {
        self.cylinder_sequence = u128::from_str_radix(x, 2)
            .expect("Invalid cylinder sequence. Expected binary number");
        self.cylinder_capacity = x.len() as u8;
    }

    #[allow(dead_code)]
    pub fn is_loaded(&self) -> IsLoaded {
        match self.cylinder_sequence > 0 {
            true => IsLoaded::Loaded,
            false => IsLoaded::Unloaded
        }
    }  
}