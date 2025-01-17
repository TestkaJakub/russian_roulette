use rand::prelude::*;

macro_rules! impl_state_check {
    ($enum_name:ident, $variant:ident, $fn_name:ident) => {
        impl $enum_name {
            #[allow(dead_code)]
            pub fn $fn_name(&self) -> bool {
                matches!(self, $enum_name::$variant)
            }
        }
    };
}
pub enum Trigger {
    Fire,
    DryFire,
}

impl_state_check!(Trigger, Fire, fired);

pub enum IsLoaded {
    Loaded,
    Unloaded,
}

impl_state_check!(IsLoaded, Loaded, loaded);

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
    
    pub fn spin(&mut self, x : Option<u8>) {
        let mut rng = rand::thread_rng();
        let spin_strength  = x.unwrap_or_else(|| rng.gen_range(0..self.cylinder_capacity));

        self.cylinder_sequence = self.cylinder_sequence.rotate_right(spin_strength as u32);
    }

    #[allow(dead_code)]
    pub fn attempt_shooting(&mut self) -> Trigger {
        let result = match self.cylinder_sequence % 2 == 1 {
            true => Trigger::Fire,
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