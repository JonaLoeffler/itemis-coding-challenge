use std::{collections::HashMap, str::FromStr};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
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
            _ => Err(format!("Unknown character {s}")),
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
struct RomanNumber(Vec<RomanNumeral>);

impl RomanNumber {
    fn value(self) -> i32 {
        self.0.into_iter().collect()
    }
}

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

#[derive(Debug)]
struct NumericSystem {
    numerals: HashMap<String, RomanNumeral>,
    credits: HashMap<String, f32>,
}

impl NumericSystem {
    fn new(infos: Vec<&str>) -> Result<Self, String> {
        let mut result = Self {
            numerals: HashMap::new(),
            credits: HashMap::new(),
        };

        for info in infos {
            let mut split = info.split(" is ");

            if let (Some(left), Some(right)) = (split.next(), split.next()) {
                if vec!["I", "V", "X", "L", "C", "D", "M"].contains(&right) {
                    if let Ok(parsed) = right.parse::<RomanNumeral>() {
                        result.numerals.insert(left.to_string(), parsed);
                    }
                } else {
                    let value: i32 = left
                        .split(' ')
                        .filter_map(|s| result.numerals.get(s))
                        .cloned()
                        .collect();

                    let material = match left.split(' ').last() {
                        Some(m) => m,
                        None => return Err("Empty left side".to_string()),
                    };

                    split = right.split(" ");

                    if let Some(creditamount) = split.next() {
                        if let Ok(amount) = creditamount.parse::<i32>() {
                            result
                                .credits
                                .insert(material.to_string(), amount as f32 / value as f32);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn convert(&self, question: &str) -> Result<String, String> {
        let val = match question.split(" is ").nth(1) {
            Some(s) => s.replace(" ?", ""),
            None => return Err("I have no idea what you are talking about".to_string()),
        };

        let amount: i32 = val
            .split(' ')
            .filter_map(|s| self.numerals.get(s))
            .cloned()
            .collect();

        let last = val.split(' ').last().unwrap_or_default();

        let material: Option<&str> = if self.credits.keys().any(|k| k == last) {
            Some(last)
        } else {
            None
        };

        let factor = self
            .credits
            .get(material.unwrap_or_default())
            .unwrap_or(&1.0);

        Ok(format!(
            "{val} is {}{}",
            amount as f32 * factor,
            if material.is_some() { " Credits" } else { "" }
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{NumericSystem, RomanNumber};

    fn initialize() -> NumericSystem {
        NumericSystem::new(vec![
            "glob is I",
            "prok is V",
            "pish is X",
            "tegj is L",
            "glob glob Silver is 34 Credits",
            "glob prok Gold is 57800 Credits",
            "pish pish Iron is 3910 Credits",
        ])
        .unwrap()
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

    #[test]
    fn test_roman_numerals() {
        assert_eq!(
            "MCMIII"
                .parse::<RomanNumber>()
                .expect("should be able to parse")
                .value(),
            1903
        );

        assert_eq!(
            "MMVI"
                .parse::<RomanNumber>()
                .expect("should be able to parse")
                .value(),
            2006
        );

        assert_eq!(
            "MCMXLIV"
                .parse::<RomanNumber>()
                .expect("should be able to parse")
                .value(),
            1944
        );
    }
}
