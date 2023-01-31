use itemis::NumericSystem;
use std::{
    error::Error,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut system = NumericSystem::new();

    for line in io::stdin().lock().lines() {
        let line: &str = &line?;

        if line.trim().to_ascii_lowercase().starts_with("how") || line.trim().ends_with("?") {
            match system.convert(line.trim()) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {}", e),
            }
        } else {
            match system.update(line.trim()) {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
    }

    Ok(())
}
