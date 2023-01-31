mod roman;

use std::{collections::HashMap, str::FromStr};

use roman::RomanNumeral;

#[derive(Debug)]
pub struct NumericSystem {
    numerals: HashMap<String, RomanNumeral>,
    credits: HashMap<String, f32>,
}

impl NumericSystem {
    pub fn convert(&self, question: &str) -> Result<String, String> {
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

impl FromStr for NumericSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self {
            numerals: HashMap::new(),
            credits: HashMap::new(),
        };

        let split_on = " is ";

        for line in s.lines() {
            let mut split = line.trim().split(split_on);

            if let (Some(left), Some(right)) = (split.next(), split.next()) {
                if right.len() == 1 {
                    result
                        .numerals
                        .insert(left.to_string(), right.parse::<RomanNumeral>()?);
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
            } else {
                return Err(format!(
                    "Invalid line \'{line}\', failed to split on \'{split_on}\'"
                ));
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::NumericSystem;

    fn initialize() -> NumericSystem {
        "glob is I
        prok is V
        pish is X
        tegj is L
        glob glob Silver is 34 Credits
        glob prok Gold is 57800 Credits
        pish pish Iron is 3910 Credits"
            .parse::<NumericSystem>()
            .expect("string to be correct")
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
