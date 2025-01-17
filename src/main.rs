mod revolver;
use revolver::*;

fn main() {
    let mut revolver = Revolver::new("00111001");
    
    println!("{}", revolver.attempt_shooting().fired())
}