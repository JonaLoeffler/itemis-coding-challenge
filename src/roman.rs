use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum RomanNumeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanNumeral {
    fn value(&self) -> i32 {
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

impl ToString for RomanNumeral {
    fn to_string(&self) -> String {
        match self {
            RomanNumeral::I => "I".to_string(),
            RomanNumeral::V => "V".to_string(),
            RomanNumeral::X => "X".to_string(),
            RomanNumeral::L => "L".to_string(),
            RomanNumeral::C => "C".to_string(),
            RomanNumeral::D => "D".to_string(),
            RomanNumeral::M => "M".to_string(),
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
            _ => Err(format!("Unknown roman numeral {s}")),
        }
    }
}

#[derive(Debug)]
pub struct RomanNumber(Vec<RomanNumeral>);

impl FromStr for RomanNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Regular expression taken from: https://stackoverflow.com/a/267405
        // Modified to prohibit MMMM
        let re = Regex::new(r"^M{0,3}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$").unwrap();

        if s.is_empty() || !re.is_match(s) {
            return Err(format!("Invalid Roman Numeral {s}"));
        }

        Ok(RomanNumber(
            s.chars()
                .map(|c| RomanNumeral::from_str(&c.to_string()))
                .collect::<Result<Vec<RomanNumeral>, String>>()?,
        ))
    }
}

impl TryFrom<Vec<RomanNumeral>> for RomanNumber {
    type Error = String;

    fn try_from(value: Vec<RomanNumeral>) -> Result<Self, Self::Error> {
        value
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
            .parse()
    }
}

impl From<RomanNumber> for i32 {
    fn from(value: RomanNumber) -> Self {
        let mut iter = value.0.iter().peekable();

        let mut result = 0;
        while let Some(next) = iter.next() {
            if let Some(peek) = iter.peek() {
                if next.value() < peek.value() {
                    result -= next.value();
                    continue;
                }
            }

            result += next.value()
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::roman::RomanNumber;

    #[test]
    fn test_valid_roman_numerals() {
        let num1: i32 = "MCMIII"
            .parse::<RomanNumber>()
            .expect("should be able to parse")
            .into();
        assert_eq!(num1, 1903);

        let num2: i32 = "MMVI"
            .parse::<RomanNumber>()
            .expect("should be able to parse")
            .into();
        assert_eq!(num2, 2006);

        let num3: i32 = "MCMXLIV"
            .parse::<RomanNumber>()
            .expect("should be able to parse")
            .into();
        assert_eq!(num3, 1944);
    }

    #[test]
    fn test_invalid_roman_numerals() {
        "CCM".parse::<RomanNumber>().expect_err("invalid number");
        "IIII".parse::<RomanNumber>().expect_err("invalid number");
        "XXXX".parse::<RomanNumber>().expect_err("invalid number");
        "MMMM".parse::<RomanNumber>().expect_err("invalid number");
        "IMIM".parse::<RomanNumber>().expect_err("invalid number");
        "".parse::<RomanNumber>().expect_err("invalid number");
    }
}
