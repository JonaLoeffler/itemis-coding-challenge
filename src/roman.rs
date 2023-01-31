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

impl FromIterator<RomanNumeral> for i32 {
    fn from_iter<T: IntoIterator<Item = RomanNumeral>>(iter: T) -> Self {
        let items = iter.into_iter().collect::<Vec<RomanNumeral>>();

        let mut iter = items.iter().peekable();

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

#[derive(Debug)]
pub struct RomanNumber(Vec<RomanNumeral>);

impl FromStr for RomanNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RomanNumber(
            s.chars()
                .map(|c| RomanNumeral::from_str(&c.to_string()))
                .collect::<Result<Vec<RomanNumeral>, String>>()?,
        ))
    }
}

impl FromIterator<RomanNumeral> for RomanNumber {
    fn from_iter<T: IntoIterator<Item = RomanNumeral>>(iter: T) -> Self {
        RomanNumber(iter.into_iter().collect())
    }
}

impl From<RomanNumber> for i32 {
    fn from(value: RomanNumber) -> Self {
        value.0.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::roman::RomanNumber;

    #[test]
    fn test_roman_numerals() {
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
}
