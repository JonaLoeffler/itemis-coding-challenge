use std::error::Error;

use itemis::NumericSystem;

fn main() -> Result<(), Box<dyn Error>> {
    let system = "glob is I
prok is V
pish is X
tegj is L
glob glob Silver is 34 Credits
glob prok Gold is 57800 Credits
pish pish Iron is 3910 Credits"
        .parse::<NumericSystem>()?;

    println!(
        "{}",
        system.convert("how many Credits is glob prok Iron ?")?
    );

    Ok(())
}
