use clap::Parser;
use itemis::NumericSystem;
use std::{error::Error, fs, io, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// The file to read
    input_file: Option<PathBuf>,

    /// Whether to run the interactive console
    #[arg(short, long)]
    interactive: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut system = NumericSystem::new();

    if let Some(file) = args.input_file {
        let content = fs::read_to_string(&file).expect("Should have been able to read this file");

        for line in content.lines() {
            process(&mut system, line)?;
        }
    }

    if args.interactive {
        println!("\n\n>> Interactive mode, type the next line:");

        let mut buffer = String::new();
        let stdin = io::stdin();
        loop {
            stdin.read_line(&mut buffer)?;

            process(&mut system, &buffer.trim())?;

            buffer = "".to_string();
        }
    }

    Ok(())
}

fn process(system: &mut NumericSystem, line: &str) -> Result<(), Box<dyn Error>> {
    if line.starts_with("how") || line.ends_with("?") {
        match system.convert(line) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("Error: {}", e),
        }
    } else {
        match system.update(line) {
            Ok(()) => (),
            Err(e) => println!("Error: {}", e),
        }
    }

    Ok(())
}
