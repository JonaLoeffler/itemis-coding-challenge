mod roman;

use std::{collections::HashMap, str::FromStr};

use roman::{RomanNumber, RomanNumeral};

#[derive(Debug)]
pub struct NumericSystem {
    numerals: HashMap<String, RomanNumeral>,
    materials: HashMap<String, f32>,
}

impl NumericSystem {
    pub fn new() -> Self {
        Self {
            numerals: HashMap::new(),
            materials: HashMap::new(),
        }
    }

    pub fn update(&mut self, line: &str) -> Result<(), String> {
        let split_on = " is ";
        let mut split = line.trim().split(split_on);

        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            if right.len() == 1 {
                self.numerals
                    .insert(left.to_string(), right.parse::<RomanNumeral>()?);
            } else {
                let number: RomanNumber = left
                    .split(' ')
                    .filter_map(|s| self.numerals.get(s))
                    .cloned()
                    .collect::<Vec<RomanNumeral>>()
                    .try_into()?;

                let value: f32 = Into::<i32>::into(number) as f32;

                let material = match left.split(' ').last() {
                    Some(m) => m,
                    None => return Err("Empty left side".to_string()),
                };

                let next = right
                    .split(" ")
                    .next()
                    .ok_or(format!("Expected a credit amount in {right}"))?;

                let amount = match next.parse::<i32>() {
                    Ok(amount) => amount,
                    Err(e) => return Err(format!("Unable to parse integer {next}: {e}")),
                } as f32;

                self.materials.insert(material.to_string(), amount / value);
            }
        } else {
            return Err(format!(
                "Invalid line \'{line}\', failed to split on \'{split_on}\'"
            ));
        }

        Ok(())
    }

    pub fn convert(&self, question: &str) -> Result<String, String> {
        let right = match question.split(" is ").nth(1) {
            Some(s) => s.replace(" ?", ""),
            None => return Err("I have no idea what you are talking about".to_string()),
        };

        let number: RomanNumber = right
            .split(' ')
            // Right side can contain a material which we do not want for computing the amount
            .filter(|s| !self.materials.keys().any(|k| k == s))
            // Map the strings to the desired Roman numerals
            .map(|s| -> Result<RomanNumeral, String> {
                self.numerals
                    .get(s)
                    .ok_or(format!("Unknown intergalactic numeral {s}"))
                    .cloned()
            })
            .collect::<Result<Vec<RomanNumeral>, String>>()?
            .try_into()?;

        let amount: i32 = number.into();

        // The last segment might be a material
        let last = right.split(' ').last().unwrap_or_default();
        let material: Option<&str> = if self.materials.keys().any(|k| k == last) {
            Some(last)
        } else {
            None
        };

        // The material determines a factor when available
        let factor = self
            .materials
            .get(material.unwrap_or_default())
            .unwrap_or(&1.0);

        // Format the result string
        Ok(format!(
            "{right} is {}{}",
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
            materials: HashMap::new(),
        };

        for line in s.lines() {
            result.update(line)?;
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
