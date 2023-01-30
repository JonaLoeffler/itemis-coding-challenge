use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

enum RomanNumeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanNumeral {
    fn value(&self) -> usize {
        match self {
            RomanNumeral::I => 1,
            RomanNumeral::V => 5,
            RomanNumeral::X => 10,
            RomanNumeral::L => 50,
            RomanNumeral::C => 100,
            RomanNumeral::D => 500,
            RomanNumeral::M => 1000,
        }
    }
}

impl FromStr for RomanNumeral {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "I" => Ok(RomanNumeral::I),
            "V" => Ok(RomanNumeral::V),
            "X" => Ok(RomanNumeral::X),
            "L" => Ok(RomanNumeral::L),
            "C" => Ok(RomanNumeral::C),
            "D" => Ok(RomanNumeral::D),
            "M" => Ok(RomanNumeral::M),
            _ => Err(format!("Unknown character {s}")),
        }
    }
}

#[derive(Debug)]
struct NumericSystem {}

impl NumericSystem {
    fn new() -> Self {
        Self {}
    }

    fn add(self, info: &str) -> Self {
        todo!()
    }

    fn convert(&self, question: &str) -> Result<String, String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::NumericSystem;

    fn initialize() -> NumericSystem {
        NumericSystem::new()
            .add("glob is I")
            .add("prok is V")
            .add("pish is X")
            .add("tegj is L")
            .add("glob glob Silver is 34 Credits")
            .add("glob prok Gold is 57800 Credits")
            .add("pish pish Iron is 3910 Credits")
    }

    #[test]
    fn test1() {
        assert_eq!(
            initialize().convert("how much is pish tegj glob glob ?"),
            Ok("pish tegj glob glob is 42".to_string())
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            initialize().convert("how many Credits is glob prok Silver ?"),
            Ok("glob prok Silver is 68 Credits".to_string())
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            initialize().convert("how many Credits is glob prok Gold ?"),
            Ok("glob prok Gold is 57800 Credits".to_string())
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            initialize().convert("how many Credits is glob prok Iron ?"),
            Ok("glob prok Iron is 782 Credits".to_string())
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            initialize()
                .convert("how much wood could a woodchuck chuck if a woodchuck could chuck wood ?"),
            Err("I have no idea what you are talking about".to_string())
        );
    }
}
